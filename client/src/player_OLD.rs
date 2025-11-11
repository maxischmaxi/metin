use bevy::prelude::*;
use crate::GameState;
use crate::camera::OrbitCamera;
use crate::auth_state::SpawnPosition;
use crate::networking::NetworkClient;
use crate::collision::{Collider, ColliderShape, CollisionType, CollisionLayer, CollidingWith, CollisionPushback};
use crate::GameFont;
use shared::ClientMessage;
use std::time::Duration;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<PositionUpdateTimer>()
            .add_systems(OnEnter(GameState::InGame), (setup_player, setup_nameplate_ui))
            .add_systems(OnEnter(GameState::CharacterSelection), cleanup_player)
            .add_systems(OnEnter(GameState::Login), cleanup_player)
            .add_systems(OnExit(GameState::InGame), (send_disconnect, cleanup_nameplate_ui))
            .add_systems(Update, (
                player_movement,
                enforce_ground_collision, // WICHTIG: Verhindert unter-Boden fallen
                send_position_updates,
                update_nameplate_marker_position,
                update_nameplate_ui_position,
                update_nameplate_ui_text,
            ).run_if(in_state(GameState::InGame)));
    }
}

#[derive(Component)]
pub struct Player {
    pub speed: f32,
}

#[derive(Component)]
pub struct GameWorld;

/// 3D marker that follows player for nameplate positioning
#[derive(Component)]
struct PlayerNameplate;

/// 2D UI overlay that displays player name and level
#[derive(Component)]
struct NameplateUI;

#[derive(Resource)]
struct PositionUpdateTimer(Timer);

impl Default for PositionUpdateTimer {
    fn default() -> Self {
        Self(Timer::new(Duration::from_millis(50), TimerMode::Repeating))
    }
}

#[derive(Component)]
struct LastSentPosition(Vec3);

fn setup_player(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    player_query: Query<&Player>,
    spawn_position: Res<SpawnPosition>,
) {
    // Only spawn player if it doesn't exist yet
    if player_query.is_empty() {
        // Use spawn position from server, or default to (0, 1, 0)
        let spawn_pos = if spawn_position.0.length() > 0.1 {
            spawn_position.0
        } else {
            Vec3::new(0.0, 1.0, 0.0)
        };
        
        info!("Spawning player at position: {:?}", spawn_pos);
        
        // Spawn player at loaded position
        commands.spawn((
            PbrBundle {
                mesh: meshes.add(Capsule3d::new(0.5, 1.5)),
                material: materials.add(Color::srgb(0.3, 0.5, 0.8)),
                transform: Transform::from_translation(spawn_pos),
                ..default()
            },
            Player { speed: 5.0 },
            Collider {
                shape: ColliderShape::Cylinder {
                    radius: 0.5,
                    height: 1.5,
                },
                collision_type: CollisionType::Dynamic,
                layer: CollisionLayer::Player,  // Phase 3
            },
            CollisionPushback { strength: 0.8 },
            CollidingWith::default(),
            LastSentPosition(spawn_pos),
            GameWorld,
        ));

        // Spawn invisible 3D marker for nameplate (2.5 units above player)
        commands.spawn((
            SpatialBundle {
                transform: Transform::from_translation(spawn_pos + Vec3::Y * 2.5),
                ..default()
            },
            PlayerNameplate,
            GameWorld,
        ));

        // Spawn ground plane (large medieval city area)
        commands.spawn((
            PbrBundle {
                mesh: meshes.add(Plane3d::default().mesh().size(150.0, 150.0)),
                material: materials.add(Color::srgb(0.3, 0.7, 0.3)),
                ..default()
            },
            GameWorld,
        ));

        // ==================== CITY BUILDINGS ====================
        // Medieval city with buildings around a central market square
        // Plaza area: -20 to +20 in X and Z (40x40m open space) - LARGE MARKET SQUARE
        // Player spawns at (0, 1, 0) on the plaza
        // NPC "Meister der Künste" is at (5, 1, 5) at plaza edge
        // Buildings scaled to realistic medieval proportions (player is ~1.8m tall)
        // Buildings have varied sizes, rotations, and positions for organic feel
        
        // NORTH SIDE (behind the plaza)
        
        
        // ==================== CITY BUILDINGS WITH ROOFS ====================
        // All buildings now have roofs and PBR materials (Step 1 complete!)
        crate::building::spawn_city_buildings(&mut commands, &mut meshes, &mut materials);

        // Spawn light
        commands.spawn((
            DirectionalLightBundle {
                directional_light: DirectionalLight {
                    illuminance: 10000.0,
                    shadows_enabled: true,
                    ..default()
                },
                transform: Transform::from_rotation(Quat::from_euler(
                    EulerRot::XYZ,
                    -std::f32::consts::FRAC_PI_4,
                    std::f32::consts::FRAC_PI_4,
                    0.0,
                )),
                ..default()
            },
            GameWorld,
        ));
    }
}

fn cleanup_player(
    mut commands: Commands,
    world_query: Query<Entity, With<GameWorld>>,
) {
    // Despawn all game world entities (player, ground, objects, lights)
    for entity in world_query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}

fn player_movement(
    keyboard: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
    mut player_query: Query<(Entity, &mut Transform, &Player, &Collider)>,
    obstacle_query: Query<(&Transform, &Collider), Without<Player>>,
    camera_query: Query<&OrbitCamera>,
    free_cam_state: Res<crate::camera::FreeCamState>,
) {
    // Don't move player if free cam is active
    if free_cam_state.active {
        return;
    }
    
    // Get camera yaw for rotation
    let camera_yaw = camera_query
        .get_single()
        .map(|cam| cam.yaw)
        .unwrap_or(0.0);

    for (_player_entity, mut transform, player, player_collider) in player_query.iter_mut() {
        let mut input_direction = Vec3::ZERO;

        // Get input in local camera space
        if keyboard.pressed(KeyCode::KeyW) {
            input_direction.z -= 1.0; // Forward
        }
        if keyboard.pressed(KeyCode::KeyS) {
            input_direction.z += 1.0; // Backward
        }
        if keyboard.pressed(KeyCode::KeyA) {
            input_direction.x -= 1.0; // Left
        }
        if keyboard.pressed(KeyCode::KeyD) {
            input_direction.x += 1.0; // Right
        }

        if input_direction.length() > 0.0 {
            input_direction = input_direction.normalize();
            
            // Rotate input direction by camera yaw
            let rotation = Quat::from_rotation_y(camera_yaw);
            let world_direction = rotation * input_direction;
            
            // Calculate desired movement
            let movement = world_direction * player.speed * time.delta_seconds();
            
            // PREDICTIVE COLLISION: Test if new position would collide
            let current_pos = transform.translation;
            let desired_pos = current_pos + movement;
            
            // Check collision at desired position BEFORE moving
            let mut final_movement = movement;
            
            for (obstacle_transform, obstacle_collider) in obstacle_query.iter() {
                // Skip triggers - they don't block movement
                if obstacle_collider.collision_type == CollisionType::Trigger {
                    continue;
                }
                
                // Check if this layer can collide with player
                if !player_collider.layer.can_collide_with(&obstacle_collider.layer) {
                    continue;
                }
                
                let obstacle_pos = obstacle_transform.translation;
                let obstacle_rot = obstacle_transform.rotation;
                
                // Test collision at DESIRED position (predictive)
                if let Some(collision_info) = crate::collision::check_collision(
                    desired_pos,
                    transform.rotation,
                    &player_collider.shape,
                    obstacle_pos,
                    obstacle_rot,
                    &obstacle_collider.shape,
                ) {
                    // COLLISION WOULD OCCUR - prevent movement in that direction
                    // Project movement onto plane perpendicular to collision normal
                    // This allows "sliding" along walls
                    let movement_dot_normal = final_movement.dot(collision_info.normal);
                    
                    // Only block movement if pushing INTO the obstacle
                    if movement_dot_normal > 0.0 {
                        // Remove component of movement that goes into obstacle
                        final_movement -= collision_info.normal * movement_dot_normal;
                    }
                }
            }
            
            // Apply the safe movement (either full movement or projected slide)
            transform.translation += final_movement;
            
            // Rotate player to face movement direction
            if world_direction.length() > 0.0 {
                let target_rotation = Quat::from_rotation_y(
                    world_direction.x.atan2(-world_direction.z)
                );
                transform.rotation = target_rotation;
            }
        }
        
        // WICHTIG: Verhindere dass Spieler unter den Boden fällt
        // Minimum Y-Position ist 0.9 (Spieler-Kapsel ist 1.5 hoch, Radius 0.5)
        // Bei Y=0.9 ist der Boden der Kapsel bei 0.9-0.75 = 0.15, also über Y=0
        if transform.translation.y < 0.9 {
            transform.translation.y = 0.9;
        }
    }
}

/// Enforce ground collision - player can NEVER fall below Y=0.9
/// This runs every frame to catch any collision bugs or physics issues
fn enforce_ground_collision(
    mut player_query: Query<&mut Transform, With<Player>>,
) {
    const MIN_Y: f32 = 0.9; // Minimum Y position (Spieler-Kapsel ist 1.5 hoch, Radius 0.5)
    
    for mut transform in player_query.iter_mut() {
        if transform.translation.y < MIN_Y {
            // Snap player back to minimum height
            transform.translation.y = MIN_Y;
        }
    }
}

/// Send position updates to server periodically
fn send_position_updates(
    time: Res<Time>,
    mut timer: ResMut<PositionUpdateTimer>,
    mut player_query: Query<(&Transform, &mut LastSentPosition), With<Player>>,
    network: Option<Res<NetworkClient>>,
) {
    let Some(network) = network else { return };
    
    // Tick the timer
    timer.0.tick(time.delta());
    
    if timer.0.just_finished() {
        for (transform, mut last_sent) in player_query.iter_mut() {
            let current_pos = transform.translation;
            
            // Only send if position changed significantly (> 0.01 units)
            if (current_pos - last_sent.0).length() > 0.01 {
                // Send ABSOLUTE position to server (not delta!)
                if let Err(e) = network.send_message(&ClientMessage::UpdatePosition { 
                    position: current_pos 
                }) {
                    error!("Failed to send position update: {}", e);
                } else {
                    // Update last sent position
                    last_sent.0 = current_pos;
                }
            }
        }
    }
}

/// Send disconnect message when leaving InGame state
fn send_disconnect(network: Option<Res<NetworkClient>>) {
    let Some(network) = network else { return };
    
    info!("Sending disconnect to server (leaving game world)");
    
    if let Err(e) = network.send_message(&ClientMessage::Disconnect) {
        error!("Failed to send disconnect: {}", e);
    }
}

// ============================================================================
// NAMEPLATE SYSTEMS
// ============================================================================

/// Setup the 2D UI overlay for player nameplate
fn setup_nameplate_ui(
    mut commands: Commands,
    font: Res<GameFont>,
    player_stats: Res<crate::ui::PlayerStats>,
) {
    let font_handle = font.0.clone();
    
    // Create UI overlay that will follow the 3D marker
    commands.spawn((
        NodeBundle {
            style: Style {
                position_type: PositionType::Absolute,
                padding: UiRect {
                    left: Val::Px(8.0),
                    right: Val::Px(8.0),
                    top: Val::Px(4.0),
                    bottom: Val::Px(4.0),
                },
                ..default()
            },
            background_color: Color::srgba(0.0, 0.0, 0.0, 0.7).into(),
            z_index: ZIndex::Global(100),
            border_radius: BorderRadius::all(Val::Px(4.0)),
            ..default()
        },
        NameplateUI,
    ))
    .with_children(|parent| {
        parent.spawn(TextBundle::from_section(
            format!("Lvl {} - {}", player_stats.level, player_stats.character_name),
            TextStyle {
                font: font_handle,
                font_size: 18.0,
                color: Color::srgb(1.0, 0.9, 0.3), // Golden text
                ..default()
            },
        ));
    });
}

/// Cleanup nameplate UI when leaving InGame
fn cleanup_nameplate_ui(
    mut commands: Commands,
    nameplate_ui_query: Query<Entity, With<NameplateUI>>,
) {
    for entity in nameplate_ui_query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}

/// Update the 3D marker position to follow player
fn update_nameplate_marker_position(
    player_query: Query<&Transform, (With<Player>, Without<PlayerNameplate>)>,
    mut nameplate_query: Query<&mut Transform, With<PlayerNameplate>>,
) {
    let Ok(player_transform) = player_query.get_single() else { return };
    
    for mut nameplate_transform in nameplate_query.iter_mut() {
        // Keep nameplate 2.5 units above player
        nameplate_transform.translation = player_transform.translation + Vec3::Y * 2.5;
    }
}

/// Update UI position to follow 3D marker on screen
fn update_nameplate_ui_position(
    nameplate_marker_query: Query<&GlobalTransform, With<PlayerNameplate>>,
    mut nameplate_ui_query: Query<(&mut Style, &Node), With<NameplateUI>>,
    camera_query: Query<(&Camera, &GlobalTransform)>,
) {
    let Ok((camera, camera_transform)) = camera_query.get_single() else { return };
    let Ok(marker_transform) = nameplate_marker_query.get_single() else { return };
    
    let world_pos = marker_transform.translation();
    
    // Convert 3D world position to 2D screen position
    if let Some(screen_pos) = camera.world_to_viewport(camera_transform, world_pos) {
        for (mut style, node) in nameplate_ui_query.iter_mut() {
            let size = node.size();
            // Center horizontally, position above marker
            style.left = Val::Px(screen_pos.x - size.x / 2.0);
            style.top = Val::Px(screen_pos.y - size.y - 5.0);
        }
    }
}

/// Update nameplate text when level or name changes
fn update_nameplate_ui_text(
    player_stats: Res<crate::ui::PlayerStats>,
    nameplate_query: Query<&Children, With<NameplateUI>>,
    mut text_query: Query<&mut Text>,
) {
    if player_stats.is_changed() {
        for children in nameplate_query.iter() {
            for &child in children.iter() {
                if let Ok(mut text) = text_query.get_mut(child) {
                    text.sections[0].value = format!("Lvl {} - {}", 
                        player_stats.level, 
                        player_stats.character_name
                    );
                }
            }
        }
    }
}
