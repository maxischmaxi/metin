use bevy::prelude::*;
use crate::player::GameWorld;
use super::{Building, BuildingType, FloorPlan, MaterialConfig, mesh_combiner::MeshBuilder, materials};

/// Optimized building spawner that combines meshes by material
/// This reduces entity count from ~100 to ~6 per building
/// Massive CPU and draw call reduction!
pub fn spawn_building_optimized(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
    floor_plan: FloorPlan,
    base_position: Vec3,
    rotation: f32,
    building_type: BuildingType,
) -> Entity {
    let quat_rotation = Quat::from_rotation_y(rotation);
    
    // ========== PHASE 1: BUILD COMBINED MESHES ==========
    
    let mut walls_builder = MeshBuilder::new();
    let mut trim_builder = MeshBuilder::new();
    let mut windows_builder = MeshBuilder::new();
    let mut fachwerk_builder = MeshBuilder::new();
    let mut decorations_builder = MeshBuilder::new();
    
    let trim_height = 0.3;
    let mut current_y = base_position.y;
    
    // Ground floor walls (LOCAL coordinates - relative to base_position)
    let ground_height = floor_plan.ground_floor.height;
    walls_builder.add_cuboid(
        quat_rotation * Vec3::new(0.0, current_y + ground_height / 2.0 - base_position.y, 0.0),
        Vec3::new(floor_plan.width, ground_height, floor_plan.depth),
        quat_rotation,
    );
    current_y += ground_height;
    
    // Mid floors walls + trim (LOCAL coordinates)
    for floor_config in &floor_plan.mid_floors {
        if floor_config.has_trim_below {
            trim_builder.add_cuboid(
                quat_rotation * Vec3::new(0.0, current_y + trim_height / 2.0 - base_position.y, 0.0),
                Vec3::new(floor_plan.width + 0.4, trim_height, floor_plan.depth + 0.4),
                quat_rotation,
            );
            current_y += trim_height;
        }
        
        walls_builder.add_cuboid(
            quat_rotation * Vec3::new(0.0, current_y + floor_config.height / 2.0 - base_position.y, 0.0),
            Vec3::new(floor_plan.width, floor_config.height, floor_plan.depth),
            quat_rotation,
        );
        current_y += floor_config.height;
    }
    
    // Top floor walls + trim (LOCAL coordinates)
    if floor_plan.top_floor.height > 0.0 {
        if floor_plan.top_floor.has_trim_below {
            trim_builder.add_cuboid(
                quat_rotation * Vec3::new(0.0, current_y + trim_height / 2.0 - base_position.y, 0.0),
                Vec3::new(floor_plan.width + 0.4, trim_height, floor_plan.depth + 0.4),
                quat_rotation,
            );
            current_y += trim_height;
        }
        
        walls_builder.add_cuboid(
            quat_rotation * Vec3::new(0.0, current_y + floor_plan.top_floor.height / 2.0 - base_position.y, 0.0),
            Vec3::new(floor_plan.width, floor_plan.top_floor.height, floor_plan.depth),
            quat_rotation,
        );
        current_y += floor_plan.top_floor.height;
    }
    
    // Add windows
    add_windows_to_builder(
        &mut windows_builder,
        &floor_plan,
        base_position,
        quat_rotation,
    );
    
    // Add fachwerk beams
    if floor_plan.details.has_fachwerk {
        add_fachwerk_to_builder(
            &mut fachwerk_builder,
            &floor_plan,
            base_position,
            quat_rotation,
        );
    }
    
    // Add door (LOCAL coordinates)
    if floor_plan.details.has_door {
        let door_size = floor_plan.details.door_size;
        let door_pos = quat_rotation * Vec3::new(
            0.0,
            door_size.y / 2.0,
            floor_plan.depth / 2.0 + 0.1,
        );
        decorations_builder.add_cuboid(
            door_pos,
            Vec3::new(door_size.x, door_size.y, 0.2),
            quat_rotation,
        );
    }
    
    // Add decorations (chimneys)
    if floor_plan.details.decorations.has_chimney {
        add_chimneys_to_builder(
            &mut decorations_builder,
            &floor_plan,
            base_position,
            quat_rotation,
            current_y,
        );
    }
    
    // ========== PHASE 2: SPAWN ENTITIES ==========
    
    let ground_entity = commands.spawn((
        Building,
        building_type,
        GameWorld,
        SpatialBundle::from_transform(Transform::from_translation(base_position)),
    )).id();
    
    // Spawn walls mesh (combined all floors)
    if !walls_builder.is_empty() {
        let walls_mesh = meshes.add(walls_builder.build());
        let walls_material = materials.add(floor_plan.ground_floor.material.to_standard_material());
        commands.spawn((
            PbrBundle {
                mesh: walls_mesh,
                material: walls_material,
                transform: Transform::from_translation(base_position).with_rotation(quat_rotation),
                ..default()
            },
            GameWorld,
        ));
    }
    
    // Spawn trim mesh (combined all trims)
    if !trim_builder.is_empty() {
        let trim_mesh = meshes.add(trim_builder.build());
        let trim_material = materials.add(materials::dark_wood().to_standard_material());
        commands.spawn((
            PbrBundle {
                mesh: trim_mesh,
                material: trim_material,
                transform: Transform::from_translation(base_position).with_rotation(quat_rotation),
                ..default()
            },
            GameWorld,
        ));
    }
    
    // Spawn windows mesh (combined all windows)
    if !windows_builder.is_empty() {
        let windows_mesh = meshes.add(windows_builder.build());
        let windows_material = materials.add(StandardMaterial {
            base_color: Color::srgb(0.1, 0.1, 0.15),
            perceptual_roughness: 0.3,
            metallic: 0.0,
            reflectance: 0.5,
            ..default()
        });
        commands.spawn((
            PbrBundle {
                mesh: windows_mesh,
                material: windows_material,
                transform: Transform::from_translation(base_position).with_rotation(quat_rotation),
                ..default()
            },
            GameWorld,
        ));
    }
    
    // Spawn fachwerk mesh (combined all beams)
    if !fachwerk_builder.is_empty() {
        let fachwerk_mesh = meshes.add(fachwerk_builder.build());
        let fachwerk_material = materials.add(materials::dark_wood().to_standard_material());
        commands.spawn((
            PbrBundle {
                mesh: fachwerk_mesh,
                material: fachwerk_material,
                transform: Transform::from_translation(base_position).with_rotation(quat_rotation),
                ..default()
            },
            GameWorld,
        ));
    }
    
    // Spawn decorations mesh (door + chimneys)
    if !decorations_builder.is_empty() {
        let decorations_mesh = meshes.add(decorations_builder.build());
        let decorations_material = materials.add(materials::dark_wood().to_standard_material());
        commands.spawn((
            PbrBundle {
                mesh: decorations_mesh,
                material: decorations_material,
                transform: Transform::from_translation(base_position).with_rotation(quat_rotation),
                ..default()
            },
            GameWorld,
        ));
    }
    
    // Spawn roof (separate entity, different material)
    spawn_roof(
        commands,
        meshes,
        materials,
        &floor_plan,
        base_position,
        quat_rotation,
        current_y,
    );
    
    // Spawn Rapier collider (one per building)
    let total_building_height = current_y - base_position.y + floor_plan.roof_height;
    commands.spawn((
        TransformBundle::from_transform(Transform::from_xyz(
            base_position.x,
            base_position.y + total_building_height / 2.0,
            base_position.z,
        ).with_rotation(quat_rotation)),
        bevy_rapier3d::prelude::RigidBody::Fixed,
        bevy_rapier3d::prelude::Collider::cuboid(
            floor_plan.width / 2.0,
            total_building_height / 2.0,
            floor_plan.depth / 2.0,
        ),
        GameWorld,
    ));
    
    ground_entity
}

/// Adds all windows to the mesh builder (LOCAL coordinates)
fn add_windows_to_builder(
    builder: &mut MeshBuilder,
    floor_plan: &FloorPlan,
    base_position: Vec3,
    rotation: Quat,
) {
    if !floor_plan.details.has_windows {
        return;
    }
    
    let window_depth = 0.15;
    let window_size = floor_plan.details.window_size;
    
    // Start at ground floor height (local Y)
    let mut current_y = floor_plan.ground_floor.height * 0.6;
    
    // Helper to add windows for one floor (LOCAL coordinates)
    let add_floor_windows = |builder: &mut MeshBuilder, y: f32| {
        // Front face windows
        if floor_plan.details.windows_per_floor_front > 0 {
            let spacing = floor_plan.width / (floor_plan.details.windows_per_floor_front + 1) as f32;
            for i in 0..floor_plan.details.windows_per_floor_front {
                let x_offset = -floor_plan.width / 2.0 + spacing * (i + 1) as f32;
                let pos = rotation * Vec3::new(x_offset, y, floor_plan.depth / 2.0 + window_depth / 2.0);
                builder.add_cuboid(pos, Vec3::new(window_size.x, window_size.y, window_depth), rotation);
            }
        }
        
        // Back face windows
        if floor_plan.details.windows_per_floor_front > 0 {
            let spacing = floor_plan.width / (floor_plan.details.windows_per_floor_front + 1) as f32;
            for i in 0..floor_plan.details.windows_per_floor_front {
                let x_offset = -floor_plan.width / 2.0 + spacing * (i + 1) as f32;
                let pos = rotation * Vec3::new(x_offset, y, -floor_plan.depth / 2.0 - window_depth / 2.0);
                builder.add_cuboid(pos, Vec3::new(window_size.x, window_size.y, window_depth), rotation);
            }
        }
        
        // Side windows
        if floor_plan.details.windows_per_floor_side > 0 {
            let spacing = floor_plan.depth / (floor_plan.details.windows_per_floor_side + 1) as f32;
            for i in 0..floor_plan.details.windows_per_floor_side {
                let z_offset = -floor_plan.depth / 2.0 + spacing * (i + 1) as f32;
                
                // Left side
                let pos_left = rotation * Vec3::new(-floor_plan.width / 2.0 - window_depth / 2.0, y, z_offset);
                builder.add_cuboid(pos_left, Vec3::new(window_depth, window_size.y, window_size.x), rotation);
                
                // Right side
                let pos_right = rotation * Vec3::new(floor_plan.width / 2.0 + window_depth / 2.0, y, z_offset);
                builder.add_cuboid(pos_right, Vec3::new(window_depth, window_size.y, window_size.x), rotation);
            }
        }
    };
    
    // Ground floor
    add_floor_windows(builder, current_y);
    current_y += floor_plan.ground_floor.height;
    
    // Mid floors
    for floor_config in &floor_plan.mid_floors {
        current_y += if floor_config.has_trim_below { 0.3 } else { 0.0 };
        add_floor_windows(builder, current_y + floor_config.height * 0.5);
        current_y += floor_config.height;
    }
    
    // Top floor
    if floor_plan.top_floor.height > 0.0 {
        current_y += if floor_plan.top_floor.has_trim_below { 0.3 } else { 0.0 };
        add_floor_windows(builder, current_y + floor_plan.top_floor.height * 0.5);
    }
}

/// Adds fachwerk beams to the mesh builder (LOCAL coordinates)
fn add_fachwerk_to_builder(
    builder: &mut MeshBuilder,
    floor_plan: &FloorPlan,
    base_position: Vec3,
    rotation: Quat,
) {
    let beam_thickness = 0.15;
    let beam_protrusion = 0.05;
    
    let mut current_y = 0.0;  // LOCAL Y coordinate
    
    let add_floor_posts = |builder: &mut MeshBuilder, y: f32, height: f32| {
        let num_posts = (floor_plan.width / 2.5).ceil() as i32;
        for i in 0..=num_posts {
            let x_offset = -floor_plan.width / 2.0 + (i as f32) * (floor_plan.width / num_posts as f32);
            let pos = rotation * Vec3::new(x_offset, y + height / 2.0, floor_plan.depth / 2.0 + beam_protrusion);
            builder.add_cuboid(pos, Vec3::new(beam_thickness, height, beam_thickness), rotation);
        }
    };
    
    // Ground floor
    add_floor_posts(builder, current_y, floor_plan.ground_floor.height);
    current_y += floor_plan.ground_floor.height;
    
    // Mid floors
    for floor_config in &floor_plan.mid_floors {
        current_y += if floor_config.has_trim_below { 0.3 } else { 0.0 };
        add_floor_posts(builder, current_y, floor_config.height);
        current_y += floor_config.height;
    }
}

/// Adds chimneys to the decorations builder (LOCAL coordinates)
fn add_chimneys_to_builder(
    builder: &mut MeshBuilder,
    floor_plan: &FloorPlan,
    base_position: Vec3,
    rotation: Quat,
    building_height: f32,
) {
    let chimney_size = Vec3::new(0.6, 2.5, 0.6);
    let chimney_count = floor_plan.details.decorations.chimney_count;
    
    for i in 0..chimney_count {
        let offset_x = if chimney_count == 1 {
            0.0
        } else {
            -floor_plan.width / 3.0 + (i as f32) * (2.0 * floor_plan.width / 3.0) / (chimney_count - 1) as f32
        };
        
        let offset_z = -floor_plan.depth / 4.0;
        let chimney_y = (building_height - base_position.y) + chimney_size.y / 2.0;
        
        let pos = rotation * Vec3::new(offset_x, chimney_y, offset_z);
        builder.add_cuboid(pos, chimney_size, rotation);
    }
}

/// Spawns the roof as a separate entity
fn spawn_roof(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
    floor_plan: &FloorPlan,
    base_position: Vec3,
    rotation: Quat,
    building_top_y: f32,
) {
    let roof_y = building_top_y + floor_plan.roof_height / 2.0;
    let roof_material = materials.add(floor_plan.roof_material.to_standard_material());
    
    let roof_mesh = match floor_plan.roof_type {
        super::RoofType::Gabled => meshes.add(super::meshes::create_prism_roof(
            floor_plan.width + 0.5,
            floor_plan.depth + 0.5,
            floor_plan.roof_height,
        )),
        super::RoofType::Pyramid => meshes.add(super::meshes::create_pyramid_roof(
            floor_plan.width + 0.5,
            floor_plan.depth + 0.5,
            floor_plan.roof_height,
        )),
        super::RoofType::Flat => meshes.add(Cuboid::new(
            floor_plan.width,
            0.3,
            floor_plan.depth,
        )),
    };
    
    commands.spawn((
        PbrBundle {
            mesh: roof_mesh,
            material: roof_material,
            transform: Transform::from_translation(Vec3::new(base_position.x, roof_y, base_position.z))
                .with_rotation(rotation),
            ..default()
        },
        GameWorld,
    ));
}
