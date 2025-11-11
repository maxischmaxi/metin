use bevy::prelude::*;

pub struct SkyboxPlugin;

impl Plugin for SkyboxPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<GameTime>()
            .add_systems(Startup, setup_skybox)
            .add_systems(Update, (
                update_local_game_time,
                update_sun_position,
                update_moon_position,
                update_ambient_light,
                update_sun_visual_color,
                update_moon_visibility,
                update_skybox_color,
            ));
    }
}

/// Current game time (synchronized from server once, then calculated locally)
#[derive(Resource, Debug)]
pub struct GameTime {
    pub hour: f32,                    // 0.0 - 24.0 (current calculated time)
    pub initial_hour: f32,            // Initial time received from server
    pub time_synced: bool,            // Has server sent initial time?
    pub sync_instant: Option<std::time::Instant>, // When was time received?
}

impl Default for GameTime {
    fn default() -> Self {
        Self { 
            hour: 12.0,               // Default fallback
            initial_hour: 12.0,
            time_synced: false,       // Not synced yet
            sync_instant: None,
        }
    }
}

impl GameTime {
    /// Update current time based on elapsed time since sync
    /// 96x speed: 1 real second = 96 game seconds = 0.0267 game hours
    pub fn update(&mut self) {
        if let Some(sync_instant) = self.sync_instant {
            let elapsed_seconds = sync_instant.elapsed().as_secs_f32();
            let game_hours_elapsed = elapsed_seconds * 96.0 / 3600.0; // 96x speed
            self.hour = (self.initial_hour + game_hours_elapsed) % 24.0;
        }
    }
    
    /// Set initial time from server (called once after login)
    pub fn sync_from_server(&mut self, server_hour: f32) {
        self.initial_hour = server_hour;
        self.hour = server_hour;
        self.time_synced = true;
        self.sync_instant = Some(std::time::Instant::now());
        info!("⏰ Time synced from server: {:02.1}:00", server_hour);
    }
}

#[derive(Component)]
struct Sun;

#[derive(Component)]
struct SunVisual;

#[derive(Component)]
struct Moon; // DirectionalLight for moon

#[derive(Component)]
struct MoonVisual;

#[derive(Component)]
struct Skybox;

fn setup_skybox(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    info!("🌅 Setting up skybox and sun...");
    
    // Create skybox (large sphere surrounding the world)
    let skybox_mesh = meshes.add(Sphere::new(500.0).mesh().ico(5).unwrap());
    let skybox_material = materials.add(StandardMaterial {
        base_color: Color::srgb(0.53, 0.81, 0.92), // Light blue sky
        unlit: true,
        cull_mode: Some(bevy::render::render_resource::Face::Front), // Only render inside
        ..default()
    });

    commands.spawn((
        PbrBundle {
            mesh: skybox_mesh,
            material: skybox_material.clone(),
            transform: Transform::from_xyz(0.0, 0.0, 0.0),
            ..default()
        },
        Skybox,
        Name::new("Skybox"),
    ));
    
    info!("🌤️ Skybox created (will change color based on time)");

    // Create directional light (Sun) - Start at a visible position (south, high up)
    let initial_position = Vec3::new(0.0, 80.0, -30.0);
    commands.spawn((
        DirectionalLightBundle {
            directional_light: DirectionalLight {
                color: Color::srgb(1.0, 0.98, 0.9), // Bright warm sunlight
                illuminance: 20000.0, // Bright but not excessive
                shadows_enabled: true,
                shadow_depth_bias: 0.02,
                shadow_normal_bias: 1.8,
                ..default()
            },
            transform: Transform::from_translation(initial_position)
                .looking_at(Vec3::ZERO, Vec3::Y),
            ..default()
        },
        Sun,
        Name::new("Sun (Light)"),
    ));
    
    info!("💡 DirectionalLight created with 20000 lux at {:?}", initial_position);

    // Create visible sun sphere (visual representation)
    let sun_mesh = meshes.add(Sphere::new(15.0).mesh().ico(4).unwrap());
    let sun_material = materials.add(StandardMaterial {
        base_color: Color::srgb(1.0, 0.95, 0.6), // Bright warm yellow
        emissive: LinearRgba::rgb(5.0, 4.0, 1.0), // Strong but not crazy glow
        unlit: true, // Always fully bright
        ..default()
    });

    commands.spawn((
        PbrBundle {
            mesh: sun_mesh,
            material: sun_material,
            transform: Transform::from_translation(initial_position),
            ..default()
        },
        bevy::pbr::NotShadowCaster, // Don't cast shadows!
        bevy::pbr::NotShadowReceiver, // Don't receive shadows!
        SunVisual,
        Name::new("Sun (Visual)"),
    ));

    info!("☀️ Sun spawned at {:?}", initial_position);

    // Create moon directional light (opposite of sun)
    let moon_position = Vec3::new(0.0, -80.0, 30.0);
    commands.spawn((
        DirectionalLightBundle {
            directional_light: DirectionalLight {
                color: Color::srgb(0.6, 0.65, 0.9), // Cool blue moonlight
                illuminance: 3000.0, // Much weaker than sun
                shadows_enabled: true,
                shadow_depth_bias: 0.02,
                shadow_normal_bias: 1.8,
                ..default()
            },
            transform: Transform::from_translation(moon_position)
                .looking_at(Vec3::ZERO, Vec3::Y),
            visibility: Visibility::Hidden, // Hidden during day
            ..default()
        },
        Moon,
        Name::new("Moon (Light)"),
    ));
    
    info!("🌙 Moon DirectionalLight created (hidden during day)");

    // Create moon visual mesh (opposite side of the sun)
    let moon_mesh = meshes.add(Sphere::new(12.0).mesh().ico(4).unwrap());
    let moon_material = materials.add(StandardMaterial {
        base_color: Color::srgb(0.9, 0.9, 1.0), // Pale blue-white
        emissive: LinearRgba::rgb(2.0, 2.0, 2.5), // Subtle glow
        unlit: true,
        ..default()
    });

    commands.spawn((
        PbrBundle {
            mesh: moon_mesh,
            material: moon_material,
            transform: Transform::from_translation(moon_position),
            visibility: Visibility::Hidden, // Start hidden (day time)
            ..default()
        },
        bevy::pbr::NotShadowCaster,
        bevy::pbr::NotShadowReceiver,
        MoonVisual,
        Name::new("Moon (Visual)"),
    ));

    info!("🌙 Moon spawned at {:?} (hidden during day)", moon_position);

    // Ambient light (Resource, will be updated based on time of day)
    // Higher ambient = brighter shadows, lower ambient = darker shadows
    commands.insert_resource(AmbientLight {
        color: Color::srgb(0.7, 0.75, 0.85), // Slight blue tint like sky
        brightness: 800.0, // High enough to see in shadows!
    });
    
    info!("💡 Ambient light set to 800 brightness (good shadow visibility)");
    
    info!("✅ Skybox setup complete!");
}

/// Update local game time calculation (runs every frame)
fn update_local_game_time(mut game_time: ResMut<GameTime>) {
    if game_time.time_synced {
        game_time.update();
    }
}

/// Update sun position based on game time
/// Sun rises in the east (x+), sets in the west (x-)
/// Noon (12:00) = sun directly overhead (south)
/// Midnight (0:00/24:00) = sun on opposite side (north, below horizon)
fn update_sun_position(
    game_time: Res<GameTime>,
    mut sun_light_query: Query<&mut Transform, (With<Sun>, Without<SunVisual>)>,
    mut sun_visual_query: Query<&mut Transform, (With<SunVisual>, Without<Sun>)>,
) {
    let hour = game_time.hour;
    
    // Convert hour to angle (0-24 hours → 0-360 degrees)
    // We want: 6:00 (sunrise) = East, 12:00 (noon) = overhead, 18:00 (sunset) = West
    let angle_degrees = (hour / 24.0) * 360.0;
    let angle_radians = angle_degrees.to_radians();

    // Calculate position on arc
    let radius = 100.0;
    
    // Create circular arc motion
    // At 12:00 (noon), angle = 180° → sun should be high up
    // At 0:00 (midnight), angle = 0° → sun below horizon
    let sun_x = angle_radians.sin() * radius;
    let sun_y = -angle_radians.cos() * radius; // Negative cos: high at 180° (noon)
    let sun_z = -30.0; // Constant offset toward south
    
    let new_position = Vec3::new(sun_x, sun_y, sun_z);
    
    // Update light position and direction
    if let Ok(mut sun_transform) = sun_light_query.get_single_mut() {
        sun_transform.translation = new_position;
        sun_transform.look_at(Vec3::ZERO, Vec3::Y);
    }
    
    // Update visual sun mesh position
    if let Ok(mut visual_transform) = sun_visual_query.get_single_mut() {
        visual_transform.translation = new_position;
    }
}

/// Update moon position (opposite of the sun)
fn update_moon_position(
    game_time: Res<GameTime>,
    mut moon_light_query: Query<&mut Transform, (With<Moon>, Without<MoonVisual>)>,
    mut moon_visual_query: Query<&mut Transform, (With<MoonVisual>, Without<Moon>)>,
) {
    let hour = game_time.hour;
    
    // Moon is on the opposite side of the sun
    // When sun angle is 0°, moon angle is 180°
    let angle_degrees = (hour / 24.0) * 360.0;
    let angle_radians = angle_degrees.to_radians();
    
    // Add 180° (π radians) to put moon on opposite side
    let moon_angle = angle_radians + std::f32::consts::PI;

    let radius = 100.0;
    let moon_x = moon_angle.sin() * radius;
    let moon_y = -moon_angle.cos() * radius;
    let moon_z = 30.0; // Opposite direction from sun
    
    let moon_position = Vec3::new(moon_x, moon_y, moon_z);
    
    // Update moon light position and direction
    if let Ok(mut moon_transform) = moon_light_query.get_single_mut() {
        moon_transform.translation = moon_position;
        moon_transform.look_at(Vec3::ZERO, Vec3::Y);
    }
    
    // Update moon visual position
    if let Ok(mut moon_transform) = moon_visual_query.get_single_mut() {
        moon_transform.translation = moon_position;
    }
}

/// Update ambient light and skybox color based on time of day
fn update_ambient_light(
    game_time: Res<GameTime>,
    mut sun_query: Query<&mut DirectionalLight, With<Sun>>,
    mut ambient_light: ResMut<AmbientLight>,
) {
    // Calculate light intensity and color based on time
    let hour = game_time.hour;
    
    // Time periods with dramatic lighting changes
    let (sun_intensity, sun_color, ambient_brightness) = if hour < 5.0 {
        // Deep night - moonlight but still visible
        (2000.0, Color::srgb(0.4, 0.5, 0.9), 200.0)
    } else if hour < 6.5 {
        // Early dawn (5-6.5) - Deep orange/red sunrise
        let t = (hour - 5.0) / 1.5; // 0.0 to 1.0
        let intensity = 2000.0 + (8000.0 * t);
        let color = Color::srgb(
            1.0,                      // Full red
            0.4 + (0.4 * t),         // Orange to yellow
            0.3 + (0.3 * t),         // Some blue
        );
        let ambient = 200.0 + (300.0 * t);
        (intensity, color, ambient)
    } else if hour < 8.0 {
        // Late dawn (6.5-8) - Orange to golden yellow
        let t = (hour - 6.5) / 1.5; // 0.0 to 1.0
        let intensity = 10000.0 + (10000.0 * t);
        let color = Color::srgb(
            1.0,                      // Red stays high
            0.8 + (0.18 * t),        // Yellow
            0.6 + (0.3 * t),         // Blue increases
        );
        let ambient = 500.0 + (300.0 * t);
        (intensity, color, ambient)
    } else if hour < 17.0 {
        // Full day - Bright golden sunlight
        (20000.0, Color::srgb(1.0, 0.98, 0.9), 800.0)
    } else if hour < 18.5 {
        // Early dusk (17-18.5) - Golden to orange
        let t = (hour - 17.0) / 1.5; // 0.0 to 1.0
        let intensity = 20000.0 - (10000.0 * t);
        let color = Color::srgb(
            1.0,                      // Red stays high
            0.98 - (0.18 * t),       // Less yellow
            0.9 - (0.3 * t),         // Less blue
        );
        let ambient = 800.0 - (300.0 * t);
        (intensity, color, ambient)
    } else if hour < 20.0 {
        // Late dusk (18.5-20) - Deep orange/red sunset
        let t = (hour - 18.5) / 1.5; // 0.0 to 1.0
        let intensity = 10000.0 - (8000.0 * t);
        let color = Color::srgb(
            1.0,                      // Full red
            0.8 - (0.3 * t),         // Orange fading
            0.6 - (0.3 * t),         // Blue disappearing
        );
        let ambient = 500.0 - (300.0 * t);
        (intensity, color, ambient)
    } else {
        // Night - moonlight but visible
        (2000.0, Color::srgb(0.4, 0.5, 0.9), 200.0)
    };

    // Update sun directional light
    if let Ok(mut sun) = sun_query.get_single_mut() {
        sun.illuminance = sun_intensity;
        sun.color = sun_color;
    }

    // Update ambient light (Resource)
    ambient_light.brightness = ambient_brightness;
    
    // Dynamic ambient light color based on time
    // Keep it relatively neutral so shadows aren't too colored
    if hour < 5.0 || hour >= 20.0 {
        // Night - slight blue tint
        ambient_light.color = Color::srgb(0.6, 0.65, 0.8);
    } else if hour < 7.0 {
        // Dawn - warm tint
        let t = (hour - 5.0) / 2.0;
        ambient_light.color = Color::srgb(
            0.75 + (0.15 * t),
            0.7 + (0.15 * t),
            0.75 + (0.15 * t),
        );
    } else if hour < 17.0 {
        // Day - neutral bright
        ambient_light.color = Color::srgb(0.85, 0.85, 0.9);
    } else if hour < 19.0 {
        // Dusk - warm tint
        let t = (hour - 17.0) / 2.0;
        ambient_light.color = Color::srgb(
            0.9 - (0.15 * t),
            0.85 - (0.15 * t),
            0.9 - (0.15 * t),
        );
    } else {
        // Evening to night transition
        let t = (hour - 19.0) / 1.0;
        ambient_light.color = Color::srgb(
            0.75 - (0.15 * t),
            0.7 - (0.05 * t),
            0.75 + (0.05 * t),
        );
    }
}

/// Update the visual sun's color and glow based on time of day
fn update_sun_visual_color(
    game_time: Res<GameTime>,
    mut sun_visual_query: Query<&Handle<StandardMaterial>, With<SunVisual>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    if let Ok(material_handle) = sun_visual_query.get_single_mut() {
        if let Some(material) = materials.get_mut(material_handle) {
            let hour = game_time.hour;
            
            // Calculate sun visual color based on time
            let (base_color, emissive) = if hour < 5.0 || hour >= 20.0 {
                // Night - dim pale moon-like
                (
                    Color::srgb(0.9, 0.9, 1.0),
                    LinearRgba::rgb(1.0, 1.0, 1.5),
                )
            } else if hour < 7.0 {
                // Dawn - orange/red
                let t = (hour - 5.0) / 2.0;
                (
                    Color::srgb(1.0, 0.6 + (0.3 * t), 0.3),
                    LinearRgba::rgb(8.0, 3.0 + (3.0 * t), 0.5),
                )
            } else if hour < 17.0 {
                // Full day - bright golden yellow
                (
                    Color::srgb(1.0, 0.95, 0.7),
                    LinearRgba::rgb(5.0, 4.5, 2.0),
                )
            } else if hour < 19.0 {
                // Dusk - yellow to orange
                let t = (hour - 17.0) / 2.0;
                (
                    Color::srgb(1.0, 0.9 - (0.3 * t), 0.7 - (0.4 * t)),
                    LinearRgba::rgb(8.0, 6.0 - (3.0 * t), 2.0 - (1.5 * t)),
                )
            } else {
                // Late dusk - orange/red
                (
                    Color::srgb(1.0, 0.6, 0.3),
                    LinearRgba::rgb(8.0, 3.0, 0.5),
                )
            };
            
            material.base_color = base_color;
            material.emissive = emissive;
        }
    }
}

/// Update moon visibility based on time of day
/// Moon is visible at night (18:00 - 06:00), hidden during day
fn update_moon_visibility(
    game_time: Res<GameTime>,
    mut moon_light_query: Query<&mut Visibility, (With<Moon>, Without<MoonVisual>)>,
    mut moon_visual_query: Query<&mut Visibility, (With<MoonVisual>, Without<Moon>)>,
    mut sun_light_query: Query<&mut Visibility, (With<Sun>, Without<SunVisual>, Without<Moon>, Without<MoonVisual>)>,
) {
    let hour = game_time.hour;
    
    // Moon visible at night: 18:00 (18.0) to 06:00 (6.0)
    // This means: hour >= 18.0 OR hour < 6.0
    let moon_visible = hour >= 18.0 || hour < 6.0;
    let sun_visible = !moon_visible; // Sun visible during day
    
    // Update moon light visibility
    if let Ok(mut visibility) = moon_light_query.get_single_mut() {
        *visibility = if moon_visible {
            Visibility::Visible
        } else {
            Visibility::Hidden
        };
    }
    
    // Update moon visual visibility
    if let Ok(mut visibility) = moon_visual_query.get_single_mut() {
        *visibility = if moon_visible {
            Visibility::Visible
        } else {
            Visibility::Hidden
        };
    }
    
    // Update sun light visibility (disable at night to save performance)
    if let Ok(mut visibility) = sun_light_query.get_single_mut() {
        *visibility = if sun_visible {
            Visibility::Visible
        } else {
            Visibility::Hidden
        };
    }
}

/// Update skybox color based on time of day
fn update_skybox_color(
    game_time: Res<GameTime>,
    skybox_query: Query<&Handle<StandardMaterial>, With<Skybox>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    if let Ok(material_handle) = skybox_query.get_single() {
        if let Some(material) = materials.get_mut(material_handle) {
            let hour = game_time.hour;
            
            // Calculate sky color based on time
            let sky_color = if hour < 5.0 {
                // Deep night - dark blue/black
                Color::srgb(0.02, 0.03, 0.08)
            } else if hour < 6.5 {
                // Early dawn - dark to purple
                let t = (hour - 5.0) / 1.5;
                Color::srgb(
                    0.02 + (0.28 * t),  // Dark to orange
                    0.03 + (0.17 * t),  // Dark to orange
                    0.08 + (0.32 * t),  // Dark blue to purple
                )
            } else if hour < 8.0 {
                // Late dawn - purple to light blue
                let t = (hour - 6.5) / 1.5;
                Color::srgb(
                    0.3 + (0.23 * t),   // Orange to light
                    0.2 + (0.61 * t),   // Orange to blue
                    0.4 + (0.52 * t),   // Purple to blue
                )
            } else if hour < 17.0 {
                // Day - light blue sky
                Color::srgb(0.53, 0.81, 0.92)
            } else if hour < 18.5 {
                // Early dusk - blue to orange
                let t = (hour - 17.0) / 1.5;
                Color::srgb(
                    0.53 - (0.23 * t),  // Light to orange
                    0.81 - (0.61 * t),  // Blue to orange
                    0.92 - (0.52 * t),  // Blue to purple
                )
            } else if hour < 20.0 {
                // Late dusk - orange to dark
                let t = (hour - 18.5) / 1.5;
                Color::srgb(
                    0.3 - (0.28 * t),   // Orange to dark
                    0.2 - (0.17 * t),   // Orange to dark
                    0.4 - (0.32 * t),   // Purple to dark blue
                )
            } else {
                // Night - dark blue/black
                Color::srgb(0.02, 0.03, 0.08)
            };
            
            material.base_color = sky_color;
        }
    }
}
