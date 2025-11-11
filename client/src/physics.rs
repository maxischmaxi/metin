use bevy::prelude::*;
use bevy_rapier3d::prelude::*;
// GameState not needed here

/// New professional physics plugin using bevy_rapier3d
/// Replaces the custom collision system entirely
pub struct PhysicsPlugin;

impl Plugin for PhysicsPlugin {
    fn build(&self, app: &mut App) {
        app
            // Add Rapier physics
            .add_plugins(RapierPhysicsPlugin::<NoUserData>::default())
            // Debug rendering (F1 to toggle) - DISABLED BY DEFAULT for performance!
            .add_plugins(RapierDebugRenderPlugin {
                enabled: false,  // Start disabled - press F1 to toggle
                ..default()
            })
            .add_systems(Update, toggle_debug_render);
    }
}

/// Toggle Rapier debug rendering with F1 key
fn toggle_debug_render(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut debug_render_context: ResMut<DebugRenderContext>,
) {
    if keyboard.just_pressed(KeyCode::F1) {
        debug_render_context.enabled = !debug_render_context.enabled;
        info!("Rapier Debug Wireframes: {}", if debug_render_context.enabled { "ON" } else { "OFF" });
    }
}

/// Marker component for kinematic character controller
#[derive(Component)]
pub struct PlayerController;

/// Marker for static world geometry
#[derive(Component)]
pub struct WorldGeometry;
