use bevy::prelude::*;
use crate::GameState;
use crate::player::GameWorld;

pub struct NpcPlugin;

impl Plugin for NpcPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::InGame), spawn_npcs);
    }
}

// NPC Types
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NpcType {
    SpecializationTrainer,
    Merchant,
    QuestGiver,
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
        commands.spawn((
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
            GameWorld,
        ));
    }
}
