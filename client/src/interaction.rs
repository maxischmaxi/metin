use bevy::prelude::*;
use crate::GameState;
use crate::npc::{Npc, NpcType};
use crate::player::Player;

pub struct InteractionPlugin;

impl Plugin for InteractionPlugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<NpcDialogState>()
            .add_systems(Update, (
                mouse_click_system,
                highlight_nearby_npcs,
            ).run_if(in_state(GameState::InGame)));
    }
}

/// Global interaction range for all NPCs (in meters)
pub const NPC_INTERACTION_RANGE: f32 = 3.0;

/// State for NPC dialog system
#[derive(Resource, Default)]
pub struct NpcDialogState {
    pub active: bool,
    pub npc_entity: Option<Entity>,
    pub npc_type: Option<NpcType>,
    pub npc_name: String,
}

impl NpcDialogState {
    pub fn open_dialog(&mut self, entity: Entity, npc_type: NpcType, name: String) {
        self.active = true;
        self.npc_entity = Some(entity);
        self.npc_type = Some(npc_type);
        self.npc_name = name;
        info!("Opening dialog with NPC: {}", self.npc_name);
    }
    
    pub fn close_dialog(&mut self) {
        info!("Closing NPC dialog");
        self.active = false;
        self.npc_entity = None;
        self.npc_type = None;
        self.npc_name.clear();
    }
}

/// Detect mouse clicks on NPCs using raycasting
fn mouse_click_system(
    mouse_button: Res<ButtonInput<MouseButton>>,
    windows: Query<&Window>,
    camera_query: Query<(&Camera, &GlobalTransform)>,
    npc_query: Query<(Entity, &Transform, &Npc)>,
    player_query: Query<&Transform, With<Player>>,
    mut npc_dialog_state: ResMut<NpcDialogState>,
) {
    // Only process left mouse button clicks
    if !mouse_button.just_pressed(MouseButton::Left) {
        return;
    }
    
    // Don't open new dialogs if one is already active
    if npc_dialog_state.active {
        return;
    }
    
    let Ok(window) = windows.get_single() else { return };
    let Some(cursor_position) = window.cursor_position() else { return };
    let Ok((camera, camera_transform)) = camera_query.get_single() else { return };
    let Ok(player_transform) = player_query.get_single() else { return };
    
    // Convert cursor position to ray in world space
    let Some(ray) = camera.viewport_to_world(camera_transform, cursor_position) else { return };
    
    // Check all NPCs for intersection
    for (entity, npc_transform, npc) in npc_query.iter() {
        let npc_pos = npc_transform.translation;
        let distance_to_player = player_transform.translation.distance(npc_pos);
        
        // First check: Is player within interaction range?
        if distance_to_player > NPC_INTERACTION_RANGE {
            continue;
        }
        
        // Second check: Did player click on this NPC?
        // Simple sphere-ray intersection test
        let to_npc = npc_pos - ray.origin;
        let projection = to_npc.dot(*ray.direction);
        
        if projection > 0.0 {
            let closest_point = ray.origin + *ray.direction * projection;
            let distance = (closest_point - npc_pos).length();
            
            // NPC has radius of 0.4, but we give a generous click radius for easier interaction
            if distance < 1.5 {
                // Successfully clicked on NPC!
                npc_dialog_state.open_dialog(entity, npc.npc_type, npc.name.clone());
                break;
            }
        }
    }
}

/// Highlight NPCs when player is nearby
fn highlight_nearby_npcs(
    player_query: Query<&Transform, With<Player>>,
    npc_query: Query<(&Transform, &Handle<StandardMaterial>), With<Npc>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let Ok(player_transform) = player_query.get_single() else { return };
    
    for (npc_transform, material_handle) in npc_query.iter() {
        let distance = player_transform.translation.distance(npc_transform.translation);
        
        if let Some(material) = materials.get_mut(material_handle) {
            if distance <= NPC_INTERACTION_RANGE {
                // Highlight: Add emissive glow
                material.emissive = Color::srgb(0.5, 0.4, 0.1).into();
            } else {
                // Normal: No glow
                material.emissive = Color::BLACK.into();
            }
        }
    }
}
