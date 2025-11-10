use bevy::prelude::*;
use bevy::input::mouse::{MouseMotion, MouseWheel};
use crate::GameState;
use crate::player::Player;

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<SavedCameraState>()
            .add_systems(Startup, setup_camera)
            .add_systems(OnEnter(GameState::InGame), switch_to_3d_camera)
            .add_systems(OnExit(GameState::InGame), (save_camera_state, switch_to_ui_camera).chain())
            .add_systems(Update, (
                orbit_camera_mouse,
                orbit_camera_zoom,
                update_camera_focus,
            ).run_if(in_state(GameState::InGame)));
    }
}

#[derive(Component, Clone)]
pub struct OrbitCamera {
    pub focus: Vec3,
    pub radius: f32,
    #[allow(dead_code)]
    pub upside_down: bool,
    pub pitch: f32,
    pub yaw: f32,
}

impl Default for OrbitCamera {
    fn default() -> Self {
        OrbitCamera {
            focus: Vec3::ZERO,
            radius: 8.0,
            upside_down: false,
            pitch: -0.3,
            yaw: 0.0,
        }
    }
}

// Resource to save camera state between game sessions
#[derive(Resource, Default)]
struct SavedCameraState {
    camera: Option<OrbitCamera>,
}

#[derive(Component)]
struct MainCamera;

// Setup main camera at startup
fn setup_camera(mut commands: Commands) {
    // Start with a 2D camera for UI menus
    commands.spawn((
        Camera2dBundle::default(),
        MainCamera,
    ));
}

// Switch to 3D camera when entering game
fn switch_to_3d_camera(
    mut commands: Commands,
    camera_query: Query<Entity, With<MainCamera>>,
    saved_state: Res<SavedCameraState>,
) {
    // Despawn old camera
    for entity in camera_query.iter() {
        commands.entity(entity).despawn_recursive();
    }
    
    // Use saved camera state if available, otherwise use default
    let orbit_camera = saved_state.camera.clone().unwrap_or_default();
    
    // Calculate initial transform based on saved orbit camera
    let rot_x = Quat::from_rotation_x(orbit_camera.pitch);
    let rot_y = Quat::from_rotation_y(orbit_camera.yaw);
    let rotation = rot_y * rot_x;
    let offset = rotation * Vec3::new(0.0, 0.0, orbit_camera.radius);
    let camera_pos = orbit_camera.focus + offset;
    
    // Spawn 3D camera with saved or default orbit controls
    commands.spawn((
        Camera3dBundle {
            transform: Transform::from_translation(camera_pos).looking_at(orbit_camera.focus, Vec3::Y),
            ..default()
        },
        orbit_camera,
        MainCamera,
    ));
}

// Save camera state before leaving game
fn save_camera_state(
    camera_query: Query<&OrbitCamera>,
    mut saved_state: ResMut<SavedCameraState>,
) {
    if let Ok(orbit_camera) = camera_query.get_single() {
        saved_state.camera = Some(orbit_camera.clone());
    }
}

// Switch back to UI camera when leaving game
fn switch_to_ui_camera(
    mut commands: Commands,
    camera_query: Query<Entity, With<MainCamera>>,
) {
    // Despawn 3D camera
    for entity in camera_query.iter() {
        commands.entity(entity).despawn_recursive();
    }
    
    // Spawn 2D camera for UI
    commands.spawn((
        Camera2dBundle::default(),
        MainCamera,
    ));
}

fn orbit_camera_mouse(
    mut mouse_motion: EventReader<MouseMotion>,
    mouse_buttons: Res<ButtonInput<MouseButton>>,
    mut query: Query<(&mut OrbitCamera, &mut Transform)>,
) {
    let mut delta = Vec2::ZERO;
    for motion in mouse_motion.read() {
        delta += motion.delta;
    }

    if mouse_buttons.pressed(MouseButton::Right) && delta.length_squared() > 0.0 {
        for (mut orbit, mut transform) in query.iter_mut() {
            orbit.yaw -= delta.x * 0.003;
            orbit.pitch -= delta.y * 0.003;
            orbit.pitch = orbit.pitch.clamp(-1.5, 1.5);

            update_camera_transform(&orbit, &mut transform);
        }
    }
}

fn orbit_camera_zoom(
    mut scroll: EventReader<MouseWheel>,
    mut query: Query<(&mut OrbitCamera, &mut Transform)>,
) {
    let mut total_scroll = 0.0;
    for event in scroll.read() {
        total_scroll += event.y;
    }

    if total_scroll != 0.0 {
        for (mut orbit, mut transform) in query.iter_mut() {
            orbit.radius -= total_scroll * 0.5;
            orbit.radius = orbit.radius.clamp(2.0, 20.0);
            update_camera_transform(&orbit, &mut transform);
        }
    }
}

fn update_camera_transform(orbit: &OrbitCamera, transform: &mut Transform) {
    let rot_x = Quat::from_rotation_x(orbit.pitch);
    let rot_y = Quat::from_rotation_y(orbit.yaw);
    let rotation = rot_y * rot_x;

    let offset = rotation * Vec3::new(0.0, 0.0, orbit.radius);
    transform.translation = orbit.focus + offset;
    transform.look_at(orbit.focus, Vec3::Y);
}

fn update_camera_focus(
    player_query: Query<&Transform, (With<Player>, Without<OrbitCamera>)>,
    mut camera_query: Query<(&mut OrbitCamera, &mut Transform), Without<Player>>,
) {
    if let Ok(player_transform) = player_query.get_single() {
        for (mut orbit, mut camera_transform) in camera_query.iter_mut() {
            orbit.focus = player_transform.translation;
            update_camera_transform(&orbit, &mut camera_transform);
        }
    }
}
