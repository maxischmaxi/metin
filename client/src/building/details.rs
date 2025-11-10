use bevy::prelude::*;
use crate::player::GameWorld;
use super::{materials, DecorationConfig};

/// Configuration for architectural details on buildings
#[derive(Clone, Debug)]
pub struct BuildingDetails {
    pub has_windows: bool,
    pub windows_per_floor_front: u32,
    pub windows_per_floor_side: u32,
    pub window_size: Vec2, // (width, height)
    pub has_door: bool,
    pub door_size: Vec2, // (width, height)
    pub has_fachwerk: bool,
    pub fachwerk_pattern: FachwerkPattern,
    // STEP 4: Material details
    pub     has_corner_stones: bool,      // Quoins at corners (stone buildings)
    pub has_window_trim: bool,        // Frames around windows
    pub has_door_trim: bool,          // Frame around door
    pub has_roof_ridge: bool,         // Ridge cap on gabled roofs
    // STEP 5: Decorations
    pub decorations: DecorationConfig, // Chimneys, lanterns, barrels, etc.
}

/// Patterns for half-timbering (Fachwerk)
#[derive(Clone, Debug, PartialEq)]
pub enum FachwerkPattern {
    None,
    Simple,      // Just vertical posts
    Cross,       // X pattern diagonal braces
    Traditional, // Full pattern with horizontals and diagonals
}

impl Default for BuildingDetails {
    fn default() -> Self {
        Self {
            has_windows: true,
            windows_per_floor_front: 2,
            windows_per_floor_side: 1,
            window_size: Vec2::new(1.0, 1.2),
            has_door: true,
            door_size: Vec2::new(1.0, 2.0),
            has_fachwerk: false,
            fachwerk_pattern: FachwerkPattern::None,
            has_corner_stones: false,
            has_window_trim: false,
            has_door_trim: false,
            has_roof_ridge: false,
            decorations: DecorationConfig::default(),
        }
    }
}

impl BuildingDetails {
    /// No details at all
    pub fn none() -> Self {
        Self {
            has_windows: false,
            has_door: false,
            has_fachwerk: false,
            ..Default::default()
        }
    }

    /// Simple house details
    pub fn simple_house() -> Self {
        Self {
            windows_per_floor_front: 2,
            windows_per_floor_side: 1,
            window_size: Vec2::new(0.9, 1.1),
            has_fachwerk: true,
            fachwerk_pattern: FachwerkPattern::Simple,
            has_window_trim: true,
            has_door_trim: true,
            has_roof_ridge: true,
            decorations: DecorationConfig::house(),
            ..Default::default()
        }
    }

    /// Townhouse with full Fachwerk
    pub fn townhouse() -> Self {
        Self {
            windows_per_floor_front: 3,
            windows_per_floor_side: 2,
            window_size: Vec2::new(0.8, 1.0),
            has_fachwerk: true,
            fachwerk_pattern: FachwerkPattern::Traditional,
            has_window_trim: true,
            has_door_trim: true,
            has_roof_ridge: true,
            decorations: DecorationConfig::house(),
            ..Default::default()
        }
    }

    /// Inn/Tavern with many windows
    pub fn inn() -> Self {
        Self {
            windows_per_floor_front: 4,
            windows_per_floor_side: 3,
            window_size: Vec2::new(1.0, 1.2),
            door_size: Vec2::new(1.5, 2.2), // Larger door
            has_fachwerk: true,
            fachwerk_pattern: FachwerkPattern::Cross,
            has_window_trim: true,
            has_door_trim: true,
            has_roof_ridge: true,
            decorations: DecorationConfig::inn(),
            ..Default::default()
        }
    }

    /// Stone building (castle, chapel, etc.) - no Fachwerk
    pub fn stone_building() -> Self {
        Self {
            windows_per_floor_front: 2,
            windows_per_floor_side: 1,
            window_size: Vec2::new(0.7, 1.5), // Tall narrow windows
            has_fachwerk: false,
            has_corner_stones: true, // Stone quoins at corners
            has_window_trim: true,   // Stone trim
            has_door_trim: true,
            has_roof_ridge: true,
            decorations: DecorationConfig::stone_building(),
            ..Default::default()
        }
    }

    /// Workshop/Smithy - fewer windows
    pub fn workshop() -> Self {
        Self {
            windows_per_floor_front: 1,
            windows_per_floor_side: 1,
            window_size: Vec2::new(0.8, 0.8), // Square windows
            door_size: Vec2::new(1.5, 2.5), // Large door for goods
            has_fachwerk: false,
            has_door_trim: true,
            has_roof_ridge: true,
            decorations: DecorationConfig::workshop(),
            ..Default::default()
        }
    }
}

/// Spawns windows on a building floor
/// 
/// # Arguments
/// * `commands` - Bevy commands
/// * `meshes` - Mesh assets
/// * `materials` - Material assets
/// * `building_pos` - Position of the building center
/// * `building_rotation` - Building's Y-axis rotation
/// * `width` - Building width (X axis)
/// * `depth` - Building depth (Z axis)
/// * `floor_y` - Y position of this floor's center
/// * `floor_height` - Height of this floor
/// * `details` - Building detail configuration
pub fn spawn_windows_for_floor(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
    building_pos: Vec3,
    building_rotation: f32,
    width: f32,
    depth: f32,
    floor_y: f32,
    floor_height: f32,
    details: &BuildingDetails,
) {
    if !details.has_windows {
        return;
    }

    let window_material = materials.add(StandardMaterial {
        base_color: Color::srgb(0.1, 0.1, 0.15), // Very dark blue-gray (glass/opening)
        perceptual_roughness: 0.3,
        metallic: 0.0,
        reflectance: 0.5,
        ..default()
    });

    let window_depth = 0.15; // How much window protrudes/insets
    let window_y = floor_y + floor_height * 0.15; // Position window in upper part of floor

    // FRONT FACE (Z+)
    if details.windows_per_floor_front > 0 {
        let spacing = width / (details.windows_per_floor_front + 1) as f32;
        for i in 0..details.windows_per_floor_front {
            let x_offset = -width / 2.0 + spacing * (i + 1) as f32;
            let z_offset = depth / 2.0 + window_depth / 2.0;
            
            let window_pos = spawn_window(
                commands,
                meshes,
                window_material.clone(),
                building_pos,
                building_rotation,
                Vec3::new(x_offset, window_y, z_offset),
                details.window_size,
                0.0, // No rotation (facing +Z)
            );
            
            // Add window trim if enabled
            if details.has_window_trim {
                let window_rotation = Quat::from_rotation_y(building_rotation);
                spawn_window_trim(commands, meshes, materials, window_pos, window_rotation, details.window_size);
            }
        }
    }

    // BACK FACE (Z-)
    if details.windows_per_floor_front > 0 {
        let spacing = width / (details.windows_per_floor_front + 1) as f32;
        for i in 0..details.windows_per_floor_front {
            let x_offset = -width / 2.0 + spacing * (i + 1) as f32;
            let z_offset = -depth / 2.0 - window_depth / 2.0;
            
            let window_pos = spawn_window(
                commands,
                meshes,
                window_material.clone(),
                building_pos,
                building_rotation,
                Vec3::new(x_offset, window_y, z_offset),
                details.window_size,
                std::f32::consts::PI, // Facing -Z
            );
            
            if details.has_window_trim {
                let window_rotation = Quat::from_rotation_y(building_rotation + std::f32::consts::PI);
                spawn_window_trim(commands, meshes, materials, window_pos, window_rotation, details.window_size);
            }
        }
    }

    // LEFT FACE (X-)
    if details.windows_per_floor_side > 0 {
        let spacing = depth / (details.windows_per_floor_side + 1) as f32;
        for i in 0..details.windows_per_floor_side {
            let z_offset = -depth / 2.0 + spacing * (i + 1) as f32;
            let x_offset = -width / 2.0 - window_depth / 2.0;
            
            let window_pos = spawn_window(
                commands,
                meshes,
                window_material.clone(),
                building_pos,
                building_rotation,
                Vec3::new(x_offset, window_y, z_offset),
                details.window_size,
                -std::f32::consts::PI / 2.0, // Facing -X
            );
            
            if details.has_window_trim {
                let window_rotation = Quat::from_rotation_y(building_rotation - std::f32::consts::PI / 2.0);
                spawn_window_trim(commands, meshes, materials, window_pos, window_rotation, details.window_size);
            }
        }
    }

    // RIGHT FACE (X+)
    if details.windows_per_floor_side > 0 {
        let spacing = depth / (details.windows_per_floor_side + 1) as f32;
        for i in 0..details.windows_per_floor_side {
            let z_offset = -depth / 2.0 + spacing * (i + 1) as f32;
            let x_offset = width / 2.0 + window_depth / 2.0;
            
            let window_pos = spawn_window(
                commands,
                meshes,
                window_material.clone(),
                building_pos,
                building_rotation,
                Vec3::new(x_offset, window_y, z_offset),
                details.window_size,
                std::f32::consts::PI / 2.0, // Facing +X
            );
            
            if details.has_window_trim {
                let window_rotation = Quat::from_rotation_y(building_rotation + std::f32::consts::PI / 2.0);
                spawn_window_trim(commands, meshes, materials, window_pos, window_rotation, details.window_size);
            }
        }
    }
}

/// Spawns a single window and returns its world position
fn spawn_window(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    material: Handle<StandardMaterial>,
    building_pos: Vec3,
    building_rotation: f32,
    local_pos: Vec3,
    size: Vec2,
    face_rotation: f32,
) -> Vec3 {
    let window_mesh = meshes.add(Cuboid::new(size.x, size.y, 0.15));
    
    // Rotate local position by building rotation
    let rotation = Quat::from_rotation_y(building_rotation);
    let rotated_local_pos = rotation * local_pos;
    let world_pos = building_pos + rotated_local_pos;
    
    let total_rotation = Quat::from_rotation_y(building_rotation + face_rotation);
    
    commands.spawn((
        PbrBundle {
            mesh: window_mesh,
            material,
            transform: Transform::from_translation(world_pos)
                .with_rotation(total_rotation),
            ..default()
        },
        GameWorld,
    ));
    
    world_pos // Return position for trim placement
}

/// Spawns a door on the front face of a building
pub fn spawn_door(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
    building_pos: Vec3,
    building_rotation: f32,
    width: f32,
    depth: f32,
    ground_floor_height: f32,
    details: &BuildingDetails,
) {
    if !details.has_door {
        return;
    }

    let door_material = materials.add(materials::dark_wood().to_standard_material());
    let door_mesh = meshes.add(Cuboid::new(
        details.door_size.x,
        details.door_size.y,
        0.2,
    ));

    // Position door at center of front face, bottom at ground level
    let local_pos = Vec3::new(
        0.0, // Center of front face
        details.door_size.y / 2.0, // Bottom at Y=0
        depth / 2.0 + 0.1, // Just in front of wall
    );

    let rotation = Quat::from_rotation_y(building_rotation);
    let rotated_local_pos = rotation * local_pos;
    let world_pos = building_pos + rotated_local_pos;

    commands.spawn((
        PbrBundle {
            mesh: door_mesh,
            material: door_material,
            transform: Transform::from_translation(world_pos)
                .with_rotation(rotation),
            ..default()
        },
        GameWorld,
    ));
}

// ==================== STEP 4: MATERIAL DETAILS ====================

/// Spawns corner stones (quoins) at the four corners of a building
/// These are decorative stone blocks that reinforce corners visually
pub fn spawn_corner_stones(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
    building_pos: Vec3,
    building_rotation: f32,
    width: f32,
    depth: f32,
    total_height: f32,
    details: &BuildingDetails,
) {
    if !details.has_corner_stones {
        return;
    }

    // Corner stones are lighter than the wall material
    let quoin_material = materials.add(StandardMaterial {
        base_color: Color::srgb(0.92, 0.92, 0.88), // Light cream stone
        perceptual_roughness: 0.85,
        metallic: 0.0,
        reflectance: 0.35,
        ..default()
    });

    let stone_size = Vec3::new(0.4, 0.6, 0.4); // Width, height, depth of each stone
    let stone_mesh = meshes.add(Cuboid::new(stone_size.x, stone_size.y, stone_size.z));
    
    // Number of stones stacked vertically
    let num_stones = (total_height / stone_size.y).ceil() as i32;
    
    let rotation = Quat::from_rotation_y(building_rotation);
    
    // Four corners
    let corners = [
        Vec3::new(-width / 2.0, 0.0, depth / 2.0),   // Front-left
        Vec3::new(width / 2.0, 0.0, depth / 2.0),    // Front-right
        Vec3::new(-width / 2.0, 0.0, -depth / 2.0),  // Back-left
        Vec3::new(width / 2.0, 0.0, -depth / 2.0),   // Back-right
    ];
    
    for corner in corners.iter() {
        for i in 0..num_stones {
            let y_offset = stone_size.y / 2.0 + (i as f32) * stone_size.y;
            let local_pos = *corner + Vec3::new(0.0, y_offset, 0.0);
            let world_pos = building_pos + rotation * local_pos;
            
            commands.spawn((
                PbrBundle {
                    mesh: stone_mesh.clone(),
                    material: quoin_material.clone(),
                    transform: Transform::from_translation(world_pos)
                        .with_rotation(rotation),
                    ..default()
                },
                GameWorld,
            ));
        }
    }
}

/// Spawns trim around a window
pub fn spawn_window_trim(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
    window_pos: Vec3,
    window_rotation: Quat,
    window_size: Vec2,
) {
    let trim_material = materials.add(StandardMaterial {
        base_color: Color::srgb(0.85, 0.82, 0.75), // Light wood/stone
        perceptual_roughness: 0.8,
        metallic: 0.0,
        reflectance: 0.3,
        ..default()
    });

    let trim_thickness = 0.08;
    let trim_depth = 0.05;
    
    // Top trim
    let top_trim = meshes.add(Cuboid::new(
        window_size.x + trim_thickness * 2.0,
        trim_thickness,
        trim_depth,
    ));
    commands.spawn((
        PbrBundle {
            mesh: top_trim,
            material: trim_material.clone(),
            transform: Transform::from_translation(
                window_pos + window_rotation * Vec3::new(0.0, window_size.y / 2.0 + trim_thickness / 2.0, 0.0)
            ).with_rotation(window_rotation),
            ..default()
        },
        GameWorld,
    ));
    
    // Bottom trim
    let bottom_trim = meshes.add(Cuboid::new(
        window_size.x + trim_thickness * 2.0,
        trim_thickness,
        trim_depth,
    ));
    commands.spawn((
        PbrBundle {
            mesh: bottom_trim,
            material: trim_material.clone(),
            transform: Transform::from_translation(
                window_pos + window_rotation * Vec3::new(0.0, -window_size.y / 2.0 - trim_thickness / 2.0, 0.0)
            ).with_rotation(window_rotation),
            ..default()
        },
        GameWorld,
    ));
    
    // Left trim
    let side_trim = meshes.add(Cuboid::new(
        trim_thickness,
        window_size.y,
        trim_depth,
    ));
    commands.spawn((
        PbrBundle {
            mesh: side_trim.clone(),
            material: trim_material.clone(),
            transform: Transform::from_translation(
                window_pos + window_rotation * Vec3::new(-window_size.x / 2.0 - trim_thickness / 2.0, 0.0, 0.0)
            ).with_rotation(window_rotation),
            ..default()
        },
        GameWorld,
    ));
    
    // Right trim
    commands.spawn((
        PbrBundle {
            mesh: side_trim,
            material: trim_material,
            transform: Transform::from_translation(
                window_pos + window_rotation * Vec3::new(window_size.x / 2.0 + trim_thickness / 2.0, 0.0, 0.0)
            ).with_rotation(window_rotation),
            ..default()
        },
        GameWorld,
    ));
}

/// Spawns trim around a door
pub fn spawn_door_trim(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
    door_pos: Vec3,
    door_rotation: Quat,
    door_size: Vec2,
) {
    let trim_material = materials.add(StandardMaterial {
        base_color: Color::srgb(0.35, 0.25, 0.18), // Dark wood for door trim
        perceptual_roughness: 0.85,
        metallic: 0.0,
        reflectance: 0.25,
        ..default()
    });

    let trim_thickness = 0.12;
    let trim_depth = 0.08;
    
    // Top trim (lintel)
    let top_trim = meshes.add(Cuboid::new(
        door_size.x + trim_thickness * 2.0,
        trim_thickness * 1.5, // Slightly thicker lintel
        trim_depth,
    ));
    commands.spawn((
        PbrBundle {
            mesh: top_trim,
            material: trim_material.clone(),
            transform: Transform::from_translation(
                door_pos + door_rotation * Vec3::new(0.0, door_size.y / 2.0 + trim_thickness * 0.75, 0.0)
            ).with_rotation(door_rotation),
            ..default()
        },
        GameWorld,
    ));
    
    // Left trim
    let side_trim = meshes.add(Cuboid::new(
        trim_thickness,
        door_size.y + trim_thickness * 1.5,
        trim_depth,
    ));
    commands.spawn((
        PbrBundle {
            mesh: side_trim.clone(),
            material: trim_material.clone(),
            transform: Transform::from_translation(
                door_pos + door_rotation * Vec3::new(-door_size.x / 2.0 - trim_thickness / 2.0, 0.0, 0.0)
            ).with_rotation(door_rotation),
            ..default()
        },
        GameWorld,
    ));
    
    // Right trim
    commands.spawn((
        PbrBundle {
            mesh: side_trim,
            material: trim_material,
            transform: Transform::from_translation(
                door_pos + door_rotation * Vec3::new(door_size.x / 2.0 + trim_thickness / 2.0, 0.0, 0.0)
            ).with_rotation(door_rotation),
            ..default()
        },
        GameWorld,
    ));
}

/// Spawns a ridge cap on a gabled roof
pub fn spawn_roof_ridge(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
    building_pos: Vec3,
    building_rotation: f32,
    roof_type: super::RoofType,
    roof_width: f32,
    roof_depth: f32,
    roof_height: f32,
    details: &BuildingDetails,
) {
    if !details.has_roof_ridge || roof_type != super::RoofType::Gabled {
        return;
    }

    // Ridge cap material - slightly different from roof tiles
    let ridge_material = materials.add(StandardMaterial {
        base_color: Color::srgb(0.7, 0.25, 0.18), // Slightly darker/redder
        perceptual_roughness: 0.75,
        metallic: 0.0,
        reflectance: 0.35,
        ..default()
    });

    // Ridge cap is a thin cuboid along the peak of the roof
    let ridge_mesh = meshes.add(Cuboid::new(0.25, 0.15, roof_depth + 0.2));
    
    let rotation = Quat::from_rotation_y(building_rotation);
    let ridge_y = building_pos.y + roof_height; // At the peak
    let world_pos = building_pos + Vec3::new(0.0, ridge_y, 0.0);
    
    commands.spawn((
        PbrBundle {
            mesh: ridge_mesh,
            material: ridge_material,
            transform: Transform::from_translation(world_pos)
                .with_rotation(rotation),
            ..default()
        },
        GameWorld,
    ));
}

/// Spawns Fachwerk (half-timbering) beams on a building floor
pub fn spawn_fachwerk_for_floor(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
    building_pos: Vec3,
    building_rotation: f32,
    width: f32,
    depth: f32,
    floor_y: f32,
    floor_height: f32,
    details: &BuildingDetails,
) {
    if !details.has_fachwerk || details.fachwerk_pattern == FachwerkPattern::None {
        return;
    }

    let beam_material = materials.add(materials::dark_wood().to_standard_material());
    let beam_thickness = 0.15;
    let beam_protrusion = 0.05; // How much beams stick out from wall

    match details.fachwerk_pattern {
        FachwerkPattern::None => {},
        
        FachwerkPattern::Simple => {
            // Just vertical posts every 2.5m
            spawn_vertical_posts(
                commands, meshes, beam_material, building_pos, building_rotation,
                width, depth, floor_y, floor_height, beam_thickness, beam_protrusion,
            );
        },
        
        FachwerkPattern::Cross => {
            // Vertical posts + horizontal beams + X-pattern diagonals
            spawn_vertical_posts(
                commands, meshes, beam_material.clone(), building_pos, building_rotation,
                width, depth, floor_y, floor_height, beam_thickness, beam_protrusion,
            );
            spawn_horizontal_beams(
                commands, meshes, beam_material.clone(), building_pos, building_rotation,
                width, depth, floor_y, floor_height, beam_thickness, beam_protrusion,
            );
            spawn_diagonal_braces(
                commands, meshes, beam_material, building_pos, building_rotation,
                width, depth, floor_y, floor_height, beam_thickness, beam_protrusion,
            );
        },
        
        FachwerkPattern::Traditional => {
            // Full pattern: posts, horizontals, and complex diagonal bracing
            spawn_vertical_posts(
                commands, meshes, beam_material.clone(), building_pos, building_rotation,
                width, depth, floor_y, floor_height, beam_thickness, beam_protrusion,
            );
            spawn_horizontal_beams(
                commands, meshes, beam_material.clone(), building_pos, building_rotation,
                width, depth, floor_y, floor_height, beam_thickness, beam_protrusion,
            );
            spawn_diagonal_braces(
                commands, meshes, beam_material, building_pos, building_rotation,
                width, depth, floor_y, floor_height, beam_thickness, beam_protrusion,
            );
        },
    }
}

fn spawn_vertical_posts(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    material: Handle<StandardMaterial>,
    building_pos: Vec3,
    building_rotation: f32,
    width: f32,
    depth: f32,
    floor_y: f32,
    floor_height: f32,
    thickness: f32,
    protrusion: f32,
) {
    let post_mesh = meshes.add(Cuboid::new(thickness, floor_height, thickness));
    let rotation = Quat::from_rotation_y(building_rotation);

    // Front face posts
    let num_posts = (width / 2.5).ceil() as i32;
    for i in 0..=num_posts {
        let x_offset = -width / 2.0 + (i as f32) * (width / num_posts as f32);
        let local_pos = Vec3::new(x_offset, floor_y, depth / 2.0 + protrusion);
        let world_pos = building_pos + rotation * local_pos;
        
        commands.spawn((
            PbrBundle {
                mesh: post_mesh.clone(),
                material: material.clone(),
                transform: Transform::from_translation(world_pos)
                    .with_rotation(rotation),
                ..default()
            },
            GameWorld,
        ));
    }
}

fn spawn_horizontal_beams(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    material: Handle<StandardMaterial>,
    building_pos: Vec3,
    building_rotation: f32,
    width: f32,
    depth: f32,
    floor_y: f32,
    floor_height: f32,
    thickness: f32,
    protrusion: f32,
) {
    let beam_mesh = meshes.add(Cuboid::new(width, thickness, thickness));
    let rotation = Quat::from_rotation_y(building_rotation);

    // Horizontal beam at mid-floor
    let local_pos = Vec3::new(0.0, floor_y, depth / 2.0 + protrusion);
    let world_pos = building_pos + rotation * local_pos;
    
    commands.spawn((
        PbrBundle {
            mesh: beam_mesh,
            material,
            transform: Transform::from_translation(world_pos)
                .with_rotation(rotation),
            ..default()
        },
        GameWorld,
    ));
}

fn spawn_diagonal_braces(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    material: Handle<StandardMaterial>,
    building_pos: Vec3,
    building_rotation: f32,
    width: f32,
    depth: f32,
    floor_y: f32,
    floor_height: f32,
    thickness: f32,
    protrusion: f32,
) {
    // Create X-pattern diagonal braces between posts
    // This is simplified - in reality would calculate exact diagonal length
    let num_sections = (width / 2.5).ceil() as i32;
    let section_width = width / num_sections as f32;
    
    for i in 0..num_sections {
        let x_start = -width / 2.0 + (i as f32) * section_width;
        let x_mid = x_start + section_width / 2.0;
        
        // Simplified: just add small diagonal indicators
        // Full implementation would calculate proper diagonal beam rotation and length
        let diagonal_length = (section_width.powi(2) + floor_height.powi(2)).sqrt();
        let diagonal_mesh = meshes.add(Cuboid::new(thickness, diagonal_length * 0.4, thickness));
        
        let rotation = Quat::from_rotation_y(building_rotation);
        let local_pos = Vec3::new(x_mid, floor_y, depth / 2.0 + protrusion);
        let world_pos = building_pos + rotation * local_pos;
        
        commands.spawn((
            PbrBundle {
                mesh: diagonal_mesh.clone(),
                material: material.clone(),
                transform: Transform::from_translation(world_pos)
                    .with_rotation(rotation * Quat::from_rotation_z(std::f32::consts::PI / 4.0)),
                ..default()
            },
            GameWorld,
        ));
    }
}
