use bevy::prelude::*;
use crate::GameState;
use crate::camera::OrbitCamera;
use crate::auth_state::SpawnPosition;
use crate::networking::NetworkClient;
use crate::collision::{Collider, ColliderShape, CollisionType, CollisionLayer, CollidingWith, CollisionPushback};
// Rapier is used via full path to avoid namespace pollution
use crate::GameFont;
use shared::ClientMessage;
use std::time::Duration;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<PositionUpdateTimer>()
            .init_resource::<PlayerAnimations>()
            .add_systems(OnEnter(GameState::InGame), (setup_player, setup_nameplate_ui))
            .add_systems(OnEnter(GameState::CharacterSelection), cleanup_player)
            .add_systems(OnEnter(GameState::Login), cleanup_player)
            .add_systems(OnExit(GameState::InGame), (send_disconnect, cleanup_nameplate_ui))
            .add_systems(Update, (
                player_movement,
                debug_scene_hierarchy,
                setup_animation_player,
                update_player_animation,
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

/// Resource holding animation clip handles
#[derive(Resource)]
struct PlayerAnimations {
    graph: Handle<AnimationGraph>,
    idle_index: AnimationNodeIndex,
    walk_index: AnimationNodeIndex,
}

impl Default for PlayerAnimations {
    fn default() -> Self {
        Self {
            graph: Handle::default(),
            idle_index: AnimationNodeIndex::new(0),
            walk_index: AnimationNodeIndex::new(0),
        }
    }
}

/// Component to track player's animation state
#[derive(Component)]
struct PlayerAnimationState {
    is_moving: bool,
}

/// Marker for the animated model entity
#[derive(Component)]
struct PlayerModel;

fn setup_player(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    player_query: Query<&Player>,
    spawn_position: Res<SpawnPosition>,
    asset_server: Res<AssetServer>,
    mut player_anims: ResMut<PlayerAnimations>,
    mut graphs: ResMut<Assets<AnimationGraph>>,
) {
    // Only spawn player if it doesn't exist yet
    if player_query.is_empty() {
        // Use spawn position from server, or default to (0, 3, 0)
        // IMPORTANT: Y=3.0 so player can fall and test gravity!
        let spawn_pos = if spawn_position.0.length() > 0.1 {
            spawn_position.0
        } else {
            Vec3::new(0.0, 3.0, 0.0)  // Higher spawn to test gravity
        };
        
        info!("Spawning player at position: {:?}", spawn_pos);
        
        // Load animation library
        let model_path = "models/animation_library/Godot/AnimationLibrary_Godot_Standard.glb";
        
        // Load animation clips
        let idle_clip: Handle<AnimationClip> = asset_server.load(GltfAssetLabel::Animation(9).from_asset(model_path));
        let walk_clip: Handle<AnimationClip> = asset_server.load(GltfAssetLabel::Animation(13).from_asset(model_path));
        
        // Create animation graph
        let mut graph = AnimationGraph::new();
        let idle_index = graph.add_clip(idle_clip, 1.0, graph.root);
        let walk_index = graph.add_clip(walk_clip, 1.0, graph.root);
        
        let graph_handle = graphs.add(graph);
        
        player_anims.graph = graph_handle.clone();
        player_anims.idle_index = idle_index;
        player_anims.walk_index = walk_index;
        
        info!("Created animation graph with Idle (index 9) and Walk (index 13)");
        
        // Character model dimensions: Height=1.829, Feet at Y=0, Head at Y=1.829
        // We adjust the collider to match the character precisely
        
        // Spawn player entity with physics (invisible parent)
        let player_entity = commands.spawn((
            SpatialBundle {
                transform: Transform::from_translation(spawn_pos),
                ..default()
            },
            Player { speed: 5.0 },
            // RAPIER PHYSICS - Gravity & Collision!
            bevy_rapier3d::prelude::RigidBody::Dynamic,  // Dynamic = affected by gravity
            bevy_rapier3d::prelude::Velocity::default(),  // Initial velocity (0,0,0)
            bevy_rapier3d::prelude::LockedAxes::ROTATION_LOCKED,  // Don't tip over!
            bevy_rapier3d::prelude::GravityScale(1.0),  // Full gravity
            bevy_rapier3d::prelude::Damping {
                linear_damping: 0.5,   // Air resistance
                angular_damping: 1.0,  // Prevent spinning
            },
            bevy_rapier3d::prelude::Friction {
                coefficient: 0.7,
                combine_rule: bevy_rapier3d::prelude::CoefficientCombineRule::Average,
            },
            // Old collision components (keep for NPCs compatibility)
            Collider {
                shape: ColliderShape::Cylinder {
                    radius: 0.3,
                    height: 1.829,
                },
                collision_type: CollisionType::Dynamic,
                layer: CollisionLayer::Player,
            },
            CollisionPushback { strength: 0.8 },
            CollidingWith::default(),
            LastSentPosition(spawn_pos),
            PlayerAnimationState { is_moving: false },
            GameWorld,
        )).id();
        
        // Add Rapier collider as a child with offset
        // Capsule collider: capsule_y(half_height, radius)
        // Total height = 2*half_height + 2*radius (hemisphere caps)
        // Character height = 1.829, radius = 0.3
        // So: 2*half_height + 2*0.3 = 1.829 → half_height = 0.6145
        commands.entity(player_entity).with_children(|parent| {
            parent.spawn((
                bevy_rapier3d::prelude::Collider::capsule_y(0.6145, 0.3),
                TransformBundle::from(Transform::from_xyz(0.0, 0.9145, 0.0)), // Center at character center of mass
            ));
        });
        
        // Spawn 3D character model as child
        // Note: AnimationPlayer will be automatically created by the GLTF scene
        // Model feet are already at Y=0 in the GLB, so no offset needed
        commands.entity(player_entity).with_children(|parent| {
            parent.spawn((
                SceneBundle {
                    scene: asset_server.load(GltfAssetLabel::Scene(0).from_asset(model_path)),
                    transform: Transform::from_xyz(0.0, 0.0, 0.0), // No offset - feet at origin
                    ..default()
                },
                PlayerModel,
            ));
        });

        // Spawn invisible 3D marker for nameplate (1.2 units above player)
        commands.spawn((
            SpatialBundle {
                transform: Transform::from_translation(spawn_pos + Vec3::Y * 1.2),
                ..default()
            },
            PlayerNameplate,
            GameWorld,
        ));

        // Spawn ground plane (large medieval city area)
        // Visual mesh at Y=0, collider BELOW at Y=-0.5 to avoid clipping
        commands.spawn((
            PbrBundle {
                mesh: meshes.add(Plane3d::default().mesh().size(150.0, 150.0)),
                material: materials.add(Color::srgb(0.3, 0.7, 0.3)),
                transform: Transform::from_xyz(0.0, 0.0, 0.0),
                ..default()
            },
            GameWorld,
        ));
        
        // Invisible ground collider BELOW the visual mesh
        commands.spawn((
            TransformBundle::from_transform(
                Transform::from_xyz(0.0, -0.5, 0.0)  // 0.5 units below surface
            ),
            bevy_rapier3d::prelude::RigidBody::Fixed,
            bevy_rapier3d::prelude::Collider::cuboid(75.0, 0.5, 75.0),  // Thin ground collider
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
        
        
        // ==================== MEDIEVAL CITY WITH KIT MODELS ====================
        // Complete rebuild with Medieval Village Kit assets!
        crate::building::spawn_city_buildings(&mut commands, &asset_server, &mut meshes, &mut materials);

        // Old DirectionalLight removed - now using dynamic sun from skybox.rs!
        // The skybox system provides a moving sun with proper day/night cycle
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
    mut player_query: Query<(&mut bevy_rapier3d::prelude::Velocity, &mut Transform, &Player)>,
    camera_query: Query<&OrbitCamera>,
    free_cam_state: Res<crate::camera::FreeCamState>,
    pause_state: Res<crate::ui::PauseMenuState>,
    settings_state: Res<crate::ui::SettingsMenuState>,
) {
    // Don't move player if pause menu or settings menu is open
    if pause_state.visible || settings_state.visible {
        return;
    }
    
    // Don't move player if free cam is active
    if free_cam_state.active {
        return;
    }
    
    // Get camera yaw for rotation
    let camera_yaw = camera_query
        .get_single()
        .map(|cam| cam.yaw)
        .unwrap_or(0.0);

    for (mut velocity, mut transform, player) in player_query.iter_mut() {
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
            
            // Set horizontal velocity (keep Y velocity for gravity!)
            let speed = player.speed;
            velocity.linvel.x = world_direction.x * speed;
            velocity.linvel.z = world_direction.z * speed;
            
            // Rotate player to face movement direction
            // Calculate target angle from movement direction (world space X and Z)
            let target_angle = world_direction.z.atan2(world_direction.x);
            let target_rotation = Quat::from_rotation_y(-target_angle + std::f32::consts::FRAC_PI_2);
            
            // Smooth rotation (interpolate between current and target)
            let rotation_speed = 12.0; // How fast player rotates (higher = faster)
            transform.rotation = transform.rotation.slerp(target_rotation, rotation_speed * time.delta_seconds());
        } else {
            // No input - stop horizontal movement (but keep falling!)
            velocity.linvel.x = 0.0;
            velocity.linvel.z = 0.0;
        }
    }
}

// enforce_ground_collision() removed - Rapier physics handles this now!

/// Setup animation player on the loaded scene (using Added filter to catch newly loaded scenes)
fn setup_animation_player(
    mut commands: Commands,
    player_model_query: Query<Entity, With<PlayerModel>>,
    // Query for AnimationPlayer that was just added (when GLTF loads)
    mut added_animation_players: Query<(Entity, &mut AnimationPlayer), Added<AnimationPlayer>>,
    children_query: Query<&Children>,
    player_anims: Res<PlayerAnimations>,
) {
    // Check if we have any newly added AnimationPlayers
    for (anim_entity, mut player) in added_animation_players.iter_mut() {
        // Check if this AnimationPlayer belongs to our PlayerModel by traversing up the hierarchy
        if is_descendant_of_player_model(anim_entity, &player_model_query, &children_query) {
            info!("🎬 Found AnimationPlayer in loaded scene! Entity: {:?}", anim_entity);
            
            // Insert the animation graph
            commands.entity(anim_entity).insert(player_anims.graph.clone());
            info!("✅ Animation graph inserted");
            
            // Play idle animation immediately
            info!("▶️  Starting IDLE animation (index: {:?})", player_anims.idle_index);
            player.play(player_anims.idle_index).repeat();
            
            // Add AnimationTransitions for smooth transitions
            commands.entity(anim_entity).insert(AnimationTransitions::new());
            
            info!("🎉 Animation system initialized successfully!");
            break;
        }
    }
}

/// Check if entity is a descendant of PlayerModel by recursively checking all PlayerModel children
fn is_descendant_of_player_model(
    entity: Entity,
    player_model_query: &Query<Entity, With<PlayerModel>>,
    children_query: &Query<&Children>,
) -> bool {
    for player_model_entity in player_model_query.iter() {
        if is_descendant_of(entity, player_model_entity, children_query) {
            return true;
        }
    }
    false
}

/// Recursively check if 'entity' is a descendant of 'ancestor'
fn is_descendant_of(
    entity: Entity,
    ancestor: Entity,
    children_query: &Query<&Children>,
) -> bool {
    if entity == ancestor {
        return true;
    }
    
    if let Ok(children) = children_query.get(ancestor) {
        for &child in children.iter() {
            if is_descendant_of(entity, child, children_query) {
                return true;
            }
        }
    }
    
    false
}

/// Debug system to log scene hierarchy (runs once per PlayerModel)
fn debug_scene_hierarchy(
    player_model_query: Query<(Entity, &Children), (With<PlayerModel>, Without<PlayerAnimationState>)>,
    children_query: Query<&Children>,
    name_query: Query<&Name>,
    animation_player_query: Query<&AnimationPlayer>,
    mut commands: Commands,
) {
    for (player_model_entity, _) in player_model_query.iter() {
        info!("🔍 === PLAYER MODEL SCENE HIERARCHY ===");
        log_hierarchy(player_model_entity, 0, &children_query, &name_query, &animation_player_query);
        info!("🔍 === END HIERARCHY ===");
        
        // Mark as debugged by adding PlayerAnimationState
        commands.entity(player_model_entity).insert(PlayerAnimationState { is_moving: false });
    }
}

/// Recursively log entity hierarchy
fn log_hierarchy(
    entity: Entity,
    depth: usize,
    children_query: &Query<&Children>,
    name_query: &Query<&Name>,
    animation_player_query: &Query<&AnimationPlayer>,
) {
    let indent = "  ".repeat(depth);
    let name = name_query.get(entity)
        .map(|n| n.as_str())
        .unwrap_or("unnamed");
    
    let has_animation_player = animation_player_query.contains(entity);
    let marker = if has_animation_player { "🎬" } else { "📦" };
    
    info!("{}{} Entity {:?}: \"{}\"", indent, marker, entity, name);
    
    if let Ok(children) = children_query.get(entity) {
        for &child in children.iter() {
            log_hierarchy(child, depth + 1, children_query, name_query, animation_player_query);
        }
    }
}

/// Update player animation based on movement
fn update_player_animation(
    mut player_query: Query<(&bevy_rapier3d::prelude::Velocity, &mut PlayerAnimationState), With<Player>>,
    mut animation_players: Query<(&mut AnimationPlayer, &mut AnimationTransitions)>,
    player_anims: Res<PlayerAnimations>,
) {
    let Ok((velocity, mut anim_state)) = player_query.get_single_mut() else { return };
    
    // Check if player is moving based on velocity
    let horizontal_speed = (velocity.linvel.x.powi(2) + velocity.linvel.z.powi(2)).sqrt();
    let is_moving = horizontal_speed > 0.1;
    
    // Get animation player
    if let Ok((mut player, mut transitions)) = animation_players.get_single_mut() {
        // Only update animation if state changed
        if is_moving != anim_state.is_moving {
            anim_state.is_moving = is_moving;
            
            let target_index = if is_moving {
                info!("Switching to WALK animation");
                player_anims.walk_index
            } else {
                info!("Switching to IDLE animation");
                player_anims.idle_index
            };
            
            transitions
                .play(&mut player, target_index, Duration::from_millis(250))
                .repeat();
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
        // Keep nameplate 1.2 units above player (closer to head)
        nameplate_transform.translation = player_transform.translation + Vec3::Y * 1.2;
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
