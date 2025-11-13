use bevy::prelude::*;
use bevy_rapier3d::prelude::*;
use crate::GameState;
use crate::npc::{Npc, NpcType};
use crate::player::Player;
use crate::ui::{UILayerStack, UILayerType};

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
    mut ui_stack: ResMut<UILayerStack>,
    rapier_context: Res<RapierContext>,
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
    let mut closest_npc: Option<(Entity, &Npc, f32)> = None;
    let mut closest_distance = f32::INFINITY;
    
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
                // Calculate distance from camera to NPC
                let camera_to_npc_distance = (npc_pos - ray.origin).length();
                
                // Track closest NPC that was clicked
                if camera_to_npc_distance < closest_distance {
                    closest_distance = camera_to_npc_distance;
                    closest_npc = Some((entity, npc, camera_to_npc_distance));
                }
            }
        }
    }
    
    // If we found a clicked NPC, check if line of sight is clear
    if let Some((entity, npc, distance_to_npc)) = closest_npc {
        let ray_origin = ray.origin;
        let ray_direction = *ray.direction;
        
        // Perform Rapier raycast to check for obstacles
        // We cast from camera to NPC position with max distance
        if let Some((hit_entity, _hit_distance)) = rapier_context.cast_ray(
            ray_origin,
            ray_direction,
            distance_to_npc,
            true, // solid (stop at first hit)
            QueryFilter::default(),
        ) {
            // Check if the first thing we hit is the NPC we're trying to interact with
            if hit_entity == entity {
                // Clear line of sight - open dialog!
                npc_dialog_state.open_dialog(entity, npc.npc_type, npc.name.clone());
                ui_stack.push_layer(UILayerType::NpcDialog);
                info!("NPC clicked with clear line of sight: {}", npc.name);
            } else {
                // Something is blocking - don't open dialog
                info!("NPC click blocked by obstacle (hit entity: {:?})", hit_entity);
            }
        } else {
            // No collision detected at all - this shouldn't happen if NPC has collider
            // but we'll allow interaction anyway
            warn!("No raycast hit detected, but allowing NPC interaction");
            npc_dialog_state.open_dialog(entity, npc.npc_type, npc.name.clone());
            ui_stack.push_layer(UILayerType::NpcDialog);
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
