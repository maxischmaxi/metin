use bevy::prelude::*;
use bevy::input::mouse::{MouseMotion, MouseWheel};
use crate::GameState;
use crate::player::Player;

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<SavedCameraState>()
            .init_resource::<FreeCamState>()
            .add_systems(Startup, setup_camera)
            .add_systems(OnEnter(GameState::InGame), switch_to_3d_camera)
            .add_systems(OnExit(GameState::InGame), (save_camera_state, switch_to_ui_camera).chain())
            .add_systems(Update, (
                toggle_free_cam,
                orbit_camera_mouse,
                orbit_camera_zoom,
                update_camera_focus,
                free_cam_movement,
                free_cam_mouse,
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

// Resource to track if free cam is active
#[derive(Resource, Default)]
pub struct FreeCamState {
    pub active: bool,
    pub(crate) saved_orbit: Option<OrbitCamera>,
}

#[derive(Component)]
struct MainCamera;

#[derive(Component)]
struct FreeCam {
    speed: f32,
    speed_boost: f32,
    sensitivity: f32,
}

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
    pause_state: Res<crate::ui::PauseMenuState>,
    settings_state: Res<crate::ui::SettingsMenuState>,
) {
    // Don't rotate camera if pause menu or settings menu is open
    if pause_state.visible || settings_state.visible {
        // Consume events to prevent them from stacking up
        mouse_motion.clear();
        return;
    }
    
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
    pause_state: Res<crate::ui::PauseMenuState>,
    settings_state: Res<crate::ui::SettingsMenuState>,
) {
    // Don't zoom camera if pause menu or settings menu is open
    if pause_state.visible || settings_state.visible {
        // Consume events to prevent them from stacking up
        scroll.clear();
        return;
    }
    
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
    free_cam_state: Res<FreeCamState>,
) {
    // Don't update camera focus if free cam is active
    if free_cam_state.active {
        return;
    }
    
    if let Ok(player_transform) = player_query.get_single() {
        for (mut orbit, mut camera_transform) in camera_query.iter_mut() {
            orbit.focus = player_transform.translation;
            update_camera_transform(&orbit, &mut camera_transform);
        }
    }
}

/// Toggle between orbit camera and free cam mode with F5 key
fn toggle_free_cam(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut free_cam_state: ResMut<FreeCamState>,
    mut commands: Commands,
    mut camera_query: Query<(Entity, &mut Transform, Option<&OrbitCamera>, Option<&FreeCam>), With<MainCamera>>,
    pause_state: Res<crate::ui::PauseMenuState>,
    settings_state: Res<crate::ui::SettingsMenuState>,
) {
    if !keyboard.just_pressed(KeyCode::F5) {
        return;
    }
    
    // Don't allow free cam toggle when pause menu or settings menu is open
    if pause_state.visible || settings_state.visible {
        return;
    }
    
    for (entity, mut transform, orbit_cam, _free_cam) in camera_query.iter_mut() {
        if free_cam_state.active {
            // Switch back to orbit camera
            info!("🎥 Free Cam deactivated - switching to Orbit Camera");
            
            if let Some(saved_orbit) = free_cam_state.saved_orbit.take() {
                // Restore orbit camera
                commands.entity(entity).remove::<FreeCam>();
                commands.entity(entity).insert(saved_orbit.clone());
                update_camera_transform(&saved_orbit, &mut transform);
            }
            
            free_cam_state.active = false;
        } else {
            // Switch to free cam
            info!("🎥 Free Cam activated - use WASD + Mouse to fly (Shift for boost)");
            
            // Save current orbit camera state
            if let Some(orbit_cam) = orbit_cam {
                free_cam_state.saved_orbit = Some(orbit_cam.clone());
                commands.entity(entity).remove::<OrbitCamera>();
            }
            
            // Add free cam component
            commands.entity(entity).insert(FreeCam {
                speed: 5.0,
                speed_boost: 3.0,
                sensitivity: 0.001,  // Reduced from 0.003 for less sensitivity
            });
            
            free_cam_state.active = true;
        }
    }
}

/// WASD movement for free cam
fn free_cam_movement(
    keyboard: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
    mut camera_query: Query<(&mut Transform, &FreeCam)>,
    pause_state: Res<crate::ui::PauseMenuState>,
    settings_state: Res<crate::ui::SettingsMenuState>,
) {
    // Don't move free cam if pause menu or settings menu is open
    if pause_state.visible || settings_state.visible {
        return;
    }
    for (mut transform, free_cam) in camera_query.iter_mut() {
        let mut velocity = Vec3::ZERO;
        let forward = *transform.forward();
        let right = *transform.right();
        let up = Vec3::Y;
        
        // WASD movement
        if keyboard.pressed(KeyCode::KeyW) {
            velocity += forward;
        }
        if keyboard.pressed(KeyCode::KeyS) {
            velocity -= forward;
        }
        if keyboard.pressed(KeyCode::KeyD) {
            velocity += right;
        }
        if keyboard.pressed(KeyCode::KeyA) {
            velocity -= right;
        }
        
        // Up/Down movement (Space/Ctrl)
        if keyboard.pressed(KeyCode::Space) {
            velocity += up;
        }
        if keyboard.pressed(KeyCode::ControlLeft) || keyboard.pressed(KeyCode::ControlRight) {
            velocity -= up;
        }
        
        // Normalize to prevent faster diagonal movement
        if velocity.length() > 0.0 {
            velocity = velocity.normalize();
        }
        
        // Apply speed boost if Shift is pressed
        let speed = if keyboard.pressed(KeyCode::ShiftLeft) || keyboard.pressed(KeyCode::ShiftRight) {
            free_cam.speed * free_cam.speed_boost
        } else {
            free_cam.speed
        };
        
        // Apply movement
        transform.translation += velocity * speed * time.delta_seconds();
    }
}

/// Mouse look for free cam
fn free_cam_mouse(
    mut mouse_motion: EventReader<MouseMotion>,
    mouse_buttons: Res<ButtonInput<MouseButton>>,
    mut camera_query: Query<(&mut Transform, &FreeCam)>,
    pause_state: Res<crate::ui::PauseMenuState>,
    settings_state: Res<crate::ui::SettingsMenuState>,
) {
    // Don't rotate free cam if pause menu or settings menu is open
    if pause_state.visible || settings_state.visible {
        // Consume events to prevent them from stacking up
        mouse_motion.clear();
        return;
    }
    
    // Only rotate when right mouse button is held (same as orbit cam)
    if !mouse_buttons.pressed(MouseButton::Right) {
        return;
    }
    
    let mut delta = Vec2::ZERO;
    for motion in mouse_motion.read() {
        delta += motion.delta;
    }
    
    if delta.length_squared() > 0.0 {
        for (mut transform, free_cam) in camera_query.iter_mut() {
            // Get current rotation
            let (mut yaw, mut pitch, _roll) = transform.rotation.to_euler(EulerRot::YXZ);
            
            // Apply mouse delta
            yaw -= delta.x * free_cam.sensitivity;
            pitch -= delta.y * free_cam.sensitivity;
            
            // Clamp pitch to prevent gimbal lock
            pitch = pitch.clamp(-1.5, 1.5);
            
            // Apply new rotation
            transform.rotation = Quat::from_euler(EulerRot::YXZ, yaw, pitch, 0.0);
        }
    }
}
