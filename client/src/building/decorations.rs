use bevy::prelude::*;
use crate::player::GameWorld;
use super::materials;

/// Configuration for decorative elements on buildings
#[derive(Clone, Debug)]
pub struct DecorationConfig {
    pub has_chimney: bool,
    pub chimney_count: u32,
    pub has_lanterns: bool,
    pub lantern_count: u32,
    pub has_barrels: bool,
    pub barrel_count: u32,
    pub has_crates: bool,
    pub crate_count: u32,
}

impl Default for DecorationConfig {
    fn default() -> Self {
        Self {
            has_chimney: false,
            chimney_count: 0,
            has_lanterns: false,
            lantern_count: 0,
            has_barrels: false,
            barrel_count: 0,
            has_crates: false,
            crate_count: 0,
        }
    }
}

impl DecorationConfig {
    /// No decorations
    pub fn none() -> Self {
        Self::default()
    }

    /// House decorations (chimney, simple)
    pub fn house() -> Self {
        Self {
            has_chimney: true,
            chimney_count: 1,
            has_lanterns: true,
            lantern_count: 1,
            ..Default::default()
        }
    }

    /// Inn/Tavern decorations (chimney, lanterns, barrels)
    pub fn inn() -> Self {
        Self {
            has_chimney: true,
            chimney_count: 2,
            has_lanterns: true,
            lantern_count: 2,
            has_barrels: true,
            barrel_count: 4,
            ..Default::default()
        }
    }

    /// Workshop decorations (chimney, crates)
    pub fn workshop() -> Self {
        Self {
            has_chimney: true,
            chimney_count: 1,
            has_lanterns: true,
            lantern_count: 1,
            has_crates: true,
            crate_count: 3,
            ..Default::default()
        }
    }

    /// Warehouse decorations (lots of crates)
    pub fn warehouse() -> Self {
        Self {
            has_lanterns: true,
            lantern_count: 1,
            has_crates: true,
            crate_count: 8,
            has_barrels: true,
            barrel_count: 6,
            ..Default::default()
        }
    }

    /// Stone building decorations (minimal, just lanterns)
    pub fn stone_building() -> Self {
        Self {
            has_lanterns: true,
            lantern_count: 2,
            ..Default::default()
        }
    }
}

/// Spawns chimneys on a building roof
pub fn spawn_chimneys(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
    building_pos: Vec3,
    building_rotation: f32,
    width: f32,
    depth: f32,
    building_height: f32,
    config: &DecorationConfig,
) {
    if !config.has_chimney || config.chimney_count == 0 {
        return;
    }

    let chimney_material = materials.add(materials::brick().to_standard_material());
    
    let chimney_width = 0.6;
    let chimney_depth = 0.6;
    let chimney_height = 2.5;
    
    let chimney_mesh = meshes.add(Cuboid::new(chimney_width, chimney_height, chimney_depth));
    
    let rotation = Quat::from_rotation_y(building_rotation);
    
    // Position chimneys on roof
    let chimney_y = building_height + chimney_height / 2.0;
    
    // Distribute chimneys evenly if multiple
    for i in 0..config.chimney_count {
        let offset_x = if config.chimney_count == 1 {
            0.0
        } else {
            -width / 3.0 + (i as f32) * (2.0 * width / 3.0) / (config.chimney_count - 1) as f32
        };
        
        let offset_z = -depth / 4.0; // Slightly back from center
        
        let local_pos = Vec3::new(offset_x, chimney_y, offset_z);
        let world_pos = building_pos + rotation * local_pos;
        
        commands.spawn((
            PbrBundle {
                mesh: chimney_mesh.clone(),
                material: chimney_material.clone(),
                transform: Transform::from_translation(world_pos)
                    .with_rotation(rotation),
                ..default()
            },
            GameWorld,
        ));
    }
}

/// Spawns lanterns near the entrance
pub fn spawn_lanterns(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
    building_pos: Vec3,
    building_rotation: f32,
    width: f32,
    depth: f32,
    config: &DecorationConfig,
) {
    if !config.has_lanterns || config.lantern_count == 0 {
        return;
    }

    let pole_material = materials.add(materials::dark_wood().to_standard_material());
    let light_material = materials.add(StandardMaterial {
        base_color: Color::srgb(1.0, 0.8, 0.4), // Warm yellow
        emissive: Color::srgb(4.0, 3.2, 1.6).into(), // Glowing effect (bright)
        perceptual_roughness: 0.5,
        metallic: 0.0,
        ..default()
    });

    let pole_mesh = meshes.add(Cylinder::new(0.08, 2.5));
    let light_mesh = meshes.add(Cuboid::new(0.3, 0.4, 0.3));
    
    let rotation = Quat::from_rotation_y(building_rotation);
    
    // Position lanterns beside door
    let lantern_positions = if config.lantern_count == 1 {
        vec![Vec3::new(width / 3.0, 1.25, depth / 2.0 + 0.3)]
    } else {
        vec![
            Vec3::new(-width / 3.0, 1.25, depth / 2.0 + 0.3), // Left of door
            Vec3::new(width / 3.0, 1.25, depth / 2.0 + 0.3),  // Right of door
        ]
    };
    
    for local_pos in lantern_positions.iter().take(config.lantern_count as usize) {
        let world_pos = building_pos + rotation * *local_pos;
        
        // Spawn pole
        commands.spawn((
            PbrBundle {
                mesh: pole_mesh.clone(),
                material: pole_material.clone(),
                transform: Transform::from_translation(world_pos)
                    .with_rotation(rotation),
                ..default()
            },
            GameWorld,
        ));
        
        // Spawn light on top
        let light_world_pos = world_pos + Vec3::new(0.0, 1.5, 0.0);
        commands.spawn((
            PbrBundle {
                mesh: light_mesh.clone(),
                material: light_material.clone(),
                transform: Transform::from_translation(light_world_pos)
                    .with_rotation(rotation),
                ..default()
            },
            GameWorld,
        ));
        
        // Add point light for actual lighting
        commands.spawn((
            PointLightBundle {
                point_light: PointLight {
                    color: Color::srgb(1.0, 0.8, 0.4),
                    intensity: 300.0,
                    radius: 8.0,
                    range: 8.0,
                    shadows_enabled: false, // Performance
                    ..default()
                },
                transform: Transform::from_translation(light_world_pos),
                ..default()
            },
            GameWorld,
        ));
    }
}

/// Spawns barrels around a building
pub fn spawn_barrels(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
    building_pos: Vec3,
    building_rotation: f32,
    width: f32,
    depth: f32,
    config: &DecorationConfig,
) {
    if !config.has_barrels || config.barrel_count == 0 {
        return;
    }

    let barrel_material = materials.add(StandardMaterial {
        base_color: Color::srgb(0.5, 0.35, 0.2), // Brown wood
        perceptual_roughness: 0.85,
        metallic: 0.0,
        reflectance: 0.25,
        ..default()
    });

    let barrel_mesh = meshes.add(Cylinder::new(0.4, 0.8));
    
    let rotation = Quat::from_rotation_y(building_rotation);
    
    // Place barrels near the side of the building
    let positions = vec![
        Vec3::new(-width / 2.0 - 0.6, 0.4, depth / 3.0),
        Vec3::new(-width / 2.0 - 0.6, 0.4, -depth / 3.0),
        Vec3::new(width / 2.0 + 0.6, 0.4, depth / 3.0),
        Vec3::new(width / 2.0 + 0.6, 0.4, -depth / 3.0),
        Vec3::new(-width / 3.0, 0.4, -depth / 2.0 - 0.6),
        Vec3::new(width / 3.0, 0.4, -depth / 2.0 - 0.6),
        Vec3::new(0.0, 0.4, -depth / 2.0 - 0.6),
        Vec3::new(width / 4.0, 0.4, depth / 2.0 + 0.6),
    ];
    
    for local_pos in positions.iter().take(config.barrel_count as usize) {
        let world_pos = building_pos + rotation * *local_pos;
        
        commands.spawn((
            PbrBundle {
                mesh: barrel_mesh.clone(),
                material: barrel_material.clone(),
                transform: Transform::from_translation(world_pos)
                    .with_rotation(rotation),
                ..default()
            },
            GameWorld,
        ));
    }
}

/// Spawns crates around a building
pub fn spawn_crates(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
    building_pos: Vec3,
    building_rotation: f32,
    width: f32,
    depth: f32,
    config: &DecorationConfig,
) {
    if !config.has_crates || config.crate_count == 0 {
        return;
    }

    let crate_material = materials.add(StandardMaterial {
        base_color: Color::srgb(0.6, 0.5, 0.3), // Light wood
        perceptual_roughness: 0.9,
        metallic: 0.0,
        reflectance: 0.2,
        ..default()
    });

    let crate_mesh = meshes.add(Cuboid::new(0.7, 0.7, 0.7));
    
    let rotation = Quat::from_rotation_y(building_rotation);
    
    // Place crates near corners and sides
    let positions = vec![
        Vec3::new(-width / 2.0 - 0.5, 0.35, depth / 2.0 + 0.5),
        Vec3::new(width / 2.0 + 0.5, 0.35, depth / 2.0 + 0.5),
        Vec3::new(-width / 2.0 - 0.5, 0.35, -depth / 2.0 - 0.5),
        Vec3::new(width / 2.0 + 0.5, 0.35, -depth / 2.0 - 0.5),
        Vec3::new(-width / 3.0, 0.35, -depth / 2.0 - 0.8),
        Vec3::new(width / 3.0, 0.35, -depth / 2.0 - 0.8),
        Vec3::new(0.0, 0.35, depth / 2.0 + 0.8),
        Vec3::new(-width / 2.0 - 0.8, 0.35, 0.0),
    ];
    
    for (i, local_pos) in positions.iter().take(config.crate_count as usize).enumerate() {
        let world_pos = building_pos + rotation * *local_pos;
        
        // Vary crate rotation slightly for natural look
        let crate_rotation = rotation * Quat::from_rotation_y((i as f32) * 0.3);
        
        commands.spawn((
            PbrBundle {
                mesh: crate_mesh.clone(),
                material: crate_material.clone(),
                transform: Transform::from_translation(world_pos)
                    .with_rotation(crate_rotation),
                ..default()
            },
            GameWorld,
        ));
    }
}

/// Applies weathering effect to a color based on height
/// Height ratio: 0.0 = ground (darker), 1.0 = top (lighter)
pub fn apply_weathering(base_color: Color, height_ratio: f32) -> Color {
    let srgba = base_color.to_srgba();
    
    // Ground is darker (dirt, moisture), top is lighter (sun exposure)
    let brightness_factor = 0.92 + (height_ratio * 0.16); // 0.92 to 1.08
    
    // Add slight random variation (simulated by height ratio wobble)
    let variation = ((height_ratio * 13.7).sin() * 0.03).abs();
    let final_factor = brightness_factor + variation;
    
    Color::srgba(
        srgba.red * final_factor,
        srgba.green * final_factor,
        srgba.blue * final_factor,
        srgba.alpha,
    )
}
