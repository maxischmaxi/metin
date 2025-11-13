use bevy::prelude::*;
use crate::GameState;
use crate::player::GameWorld;
use crate::collision::{Collider, ColliderShape, CollisionType, CollisionLayer, CollidingWith};
use crate::GameFont;

pub struct NpcPlugin;

impl Plugin for NpcPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::InGame), spawn_npcs)
            .add_systems(OnExit(GameState::InGame), cleanup_npc_nameplate_ui)
            .add_systems(Update, (
                setup_npc_nameplate_ui,
                update_npc_nameplate_ui_positions.after(setup_npc_nameplate_ui),
            ).run_if(in_state(GameState::InGame)));
    }
}

// NPC Types
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NpcType {
    SpecializationTrainer,
    Merchant,
    QuestGiver,
}

fn setup_npc_nameplate_ui(
    mut commands: Commands,
    font: Res<GameFont>,
    npc_query: Query<(Entity, &Npc), Without<HasNameplate>>,
) {
    let font_handle = font.0.clone();

    for (npc_entity, npc) in npc_query.iter() {
        info!("Creating nameplate for NPC: {}", npc.name);
        
        // Mark this NPC as having a nameplate
        commands.entity(npc_entity).insert(HasNameplate);
        
        // Create 2D UI nameplate for each NPC
        commands.spawn((
            NodeBundle {
                style: Style {
                    position_type: PositionType::Absolute,
                    left: Val::Px(0.0),
                    top: Val::Px(0.0),
                    padding: UiRect::all(Val::Px(4.0)),
                    ..default()
                },
                background_color: Color::srgba(0.0, 0.0, 0.0, 0.7).into(),
                z_index: ZIndex::Global(100),
                ..default()
            },
            NpcNameplateUI { npc_entity },
        ))
        .with_children(|parent| {
            parent.spawn(TextBundle::from_section(
                &npc.name,
                TextStyle {
                    font: font_handle.clone(),
                    font_size: 16.0,
                    color: Color::srgb(1.0, 0.84, 0.0), // Golden color for NPCs
                    ..default()
                },
            ));
        });
    }
}

fn cleanup_npc_nameplate_ui(
    mut commands: Commands,
    query: Query<Entity, With<NpcNameplateUI>>,
) {
    for entity in query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}

fn update_npc_nameplate_ui_positions(
    npc_query: Query<(&Transform, Entity), With<Npc>>,
    nameplate_marker_query: Query<(&NpcNameplate, &GlobalTransform)>,
    mut nameplate_ui_query: Query<(&mut Style, &Node, &NpcNameplateUI)>,
    camera_query: Query<(&Camera, &GlobalTransform)>,
) {
    let Ok((camera, camera_transform)) = camera_query.get_single() else { return };

    for (mut style, node, ui) in nameplate_ui_query.iter_mut() {
        // Find the corresponding NPC transform
        let Ok((npc_transform, _)) = npc_query.get(ui.npc_entity) else { continue };
        
        // Calculate nameplate position (1.2 units above NPC, same as player)
        let world_pos = npc_transform.translation + Vec3::Y * 1.2;
        
        // Convert 3D world position to 2D screen position
        if let Some(screen_pos) = camera.world_to_viewport(camera_transform, world_pos) {
            let size = node.size();
            // Center horizontally, position above NPC
            style.left = Val::Px(screen_pos.x - size.x / 2.0);
            style.top = Val::Px(screen_pos.y - size.y);
        }
    }
}


// NPC spawn data: (Position, Name, Type)
const NPC_SPAWN_POSITIONS: &[(Vec3, &str, NpcType)] = &[
    (Vec3::new(5.0, 1.0, 5.0), "Meister der Künste", NpcType::SpecializationTrainer),
];

#[derive(Component)]
pub struct Npc {
    pub name: String,
    pub npc_type: NpcType,
}

/// 3D marker that follows NPC for nameplate positioning
#[derive(Component)]
struct NpcNameplate {
    npc_entity: Entity,
}

/// 2D UI overlay that displays NPC name
#[derive(Component)]
struct NpcNameplateUI {
    npc_entity: Entity,
}

/// Marker to track if NPC has a nameplate
#[derive(Component)]
struct HasNameplate;

fn spawn_npcs(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    npc_query: Query<&Npc>,
) {
    // Only spawn if no NPCs exist yet
    if !npc_query.is_empty() {
        return;
    }

    for (position, name, npc_type) in NPC_SPAWN_POSITIONS {
        info!("Spawning NPC '{}' at {:?}", name, position);
        
        // Spawn NPC model (golden capsule)
        let npc_entity = commands.spawn((
            PbrBundle {
                mesh: meshes.add(Capsule3d::new(0.4, 1.8)),
                material: materials.add(StandardMaterial {
                    base_color: Color::srgb(0.9, 0.7, 0.2), // Golden
                    emissive: Color::BLACK.into(),
                    ..default()
                }),
                transform: Transform::from_translation(*position),
                ..default()
            },
            Npc {
                name: name.to_string(),
                npc_type: *npc_type,
            },
            // RAPIER Collider for physics raycasting
            bevy_rapier3d::prelude::RigidBody::Fixed, // NPCs don't move
            bevy_rapier3d::prelude::Collider::capsule_y(0.9, 0.4), // half_height, radius
            // Old collision system (keep for compatibility)
            Collider {
                shape: ColliderShape::Cylinder {
                    radius: 0.4,
                    height: 1.8,
                },
                collision_type: CollisionType::Static, // NPCs don't move
                layer: CollisionLayer::NPC,  // Phase 3
            },
            CollidingWith::default(),
            GameWorld,
        )).id();

        // Spawn invisible 3D marker for nameplate (1.2 units above NPC, same as player)
        commands.spawn((
            SpatialBundle {
                transform: Transform::from_translation(*position + Vec3::Y * 1.2),
                ..default()
            },
            NpcNameplate { npc_entity },
            GameWorld,
        ));
    }
}
