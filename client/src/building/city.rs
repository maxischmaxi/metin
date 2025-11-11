use bevy::prelude::*;
// Old collision system removed - now using bevy_rapier3d!
use crate::player::GameWorld;
use super::{Building, BuildingType, RoofType, MaterialConfig, materials, spawn_building_with_roof, FloorPlan, trim_material, create_prism_roof, create_pyramid_roof, floor_plans};
use super::details::{spawn_windows_for_floor, spawn_door, spawn_fachwerk_for_floor, spawn_corner_stones, spawn_door_trim, spawn_roof_ridge};
use super::decorations::{spawn_chimneys, spawn_lanterns, spawn_barrels, spawn_crates};
use super::optimized_spawner::spawn_building_optimized;

/// Spawns all city buildings with roofs and PBR materials
/// This is the STEP 1 implementation with roofs and better materials
pub fn spawn_city_buildings(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
) {
    // Helper macro to reduce boilerplate
    macro_rules! spawn_building {
        (
            $name:expr,
            $type:expr,
            $size:expr,
            $pos:expr,
            $rot:expr,
            $wall_mat:expr,
            $roof_type:expr,
            $roof_size:expr,
            $roof_mat:expr
        ) => {
            {
                let mesh = meshes.add(Cuboid::new($size.x, $size.y, $size.z));
                let entity = spawn_building_with_roof(
                    commands,
                    meshes,
                    materials,
                    mesh,
                    $wall_mat,
                    Transform::from_xyz($pos.x, $pos.y, $pos.z)
                        .with_rotation(Quat::from_rotation_y($rot)),
                    $roof_type,
                    $roof_size,
                    $roof_mat,
                );
                commands.entity(entity).insert((
                    Building,
                    $type,
        // OLD: AutoCollider removed - using Rapier physics
                    GameWorld,
                ));
            }
        };
    }

    // NORTH SIDE
    
    // 1. Inn/Tavern - NOW USING FLOOR SYSTEM (STEP 2)! 🏛️
    {
        let inn_plan = floor_plans::inn(15.0, 12.0);
        spawn_building_optimized(
            commands,
            meshes,
            materials,
            inn_plan,
            Vec3::new(-8.0, 0.0, 28.0), // Base at ground level
            0.15,
            BuildingType::Inn,
        );
    }

    // 2. Weapon Smith - OPTIMIZED ⚡
    {
        let plan = floor_plans::smithy(10.0, 9.0);
        spawn_building_optimized(
            commands,
            meshes,
            materials,
            plan,
            Vec3::new(-28.0, 0.0, 28.0),
            -0.2,
            BuildingType::Smithy,
        );
    }

    // 3. Townhouse North 1 - OPTIMIZED ⚡
    {
        let plan = floor_plans::townhouse_3_floors(7.0, 7.0);
        spawn_building_optimized(
            commands,
            meshes,
            materials,
            plan,
            Vec3::new(24.0, 0.0, 30.0),
            0.3,
            BuildingType::House,
        );
    }

    // 4. House North 2 - OPTIMIZED ⚡
    {
        let plan = floor_plans::house_2_floors(8.0, 8.0);
        spawn_building_optimized(
            commands,
            meshes,
            materials,
            plan,
            Vec3::new(32.0, 0.0, 26.0),
            -0.4,
            BuildingType::House,
        );
    }

    // 5. Guard Tower - OPTIMIZED ⚡
    {
        let plan = floor_plans::tower(6.0, 6.0, 5); // 5 floors
        spawn_building_optimized(
            commands,
            meshes,
            materials,
            plan,
            Vec3::new(35.0, 0.0, 35.0),
            0.0,
            BuildingType::Tower,
        );
    }

    // EAST SIDE

    // 6. Market Hall - OPTIMIZED ⚡
    {
        let plan = floor_plans::single_story(
            10.0, 18.0, 5.0,
            materials::brick()
        );
        spawn_building_optimized(
            commands,
            meshes,
            materials,
            plan,
            Vec3::new(30.0, 0.0, 10.0),
            0.1,
            BuildingType::Market,
        );
    }

    // 7. House East 1 - OPTIMIZED ⚡
    {
        let plan = floor_plans::house_2_floors(9.0, 8.0);
        spawn_building_optimized(
            commands,
            meshes,
            materials,
            plan,
            Vec3::new(31.0, 0.0, -8.0),
            -0.25,
            BuildingType::House,
        );
    }

    // 8. Warehouse - OPTIMIZED ⚡
    {
        let plan = floor_plans::warehouse(7.0, 10.0);
        spawn_building_optimized(
            commands,
            meshes,
            materials,
            plan,
            Vec3::new(27.0, 0.0, -20.0),
            0.5,
            BuildingType::Workshop,
        );
    }

    // SOUTH SIDE

    // 9. Blacksmith - OPTIMIZED ⚡
    {
        let plan = floor_plans::smithy(12.0, 10.0);
        spawn_building_optimized(
            commands,
            meshes,
            materials,
            plan,
            Vec3::new(2.0, 0.0, -30.0),
            -0.1,
            BuildingType::Smithy,
        );
    }

    // 10. Alchemist Tower - OPTIMIZED ⚡
    {
        let plan = floor_plans::tower(8.0, 8.0, 4); // 4-floor tower
        // But override with purple color
        let mut plan = plan;
        plan.ground_floor.material = MaterialConfig {
            base_color: Color::srgb(0.45, 0.25, 0.55), // Dark purple
            roughness: 0.85,
            metallic: 0.0,
            reflectance: 0.3,
        };
        for floor in &mut plan.mid_floors {
            floor.material = MaterialConfig {
                base_color: Color::srgb(0.5, 0.3, 0.6), // Medium purple
                roughness: 0.85,
                metallic: 0.0,
                reflectance: 0.3,
            };
        }
        plan.top_floor.material = MaterialConfig {
            base_color: Color::srgb(0.55, 0.35, 0.65), // Light purple
            roughness: 0.85,
            metallic: 0.0,
            reflectance: 0.3,
        };
        spawn_building_optimized(
            commands,
            meshes,
            materials,
            plan,
            Vec3::new(-20.0, 0.0, -28.0),
            0.35,
            BuildingType::Workshop,
        );
    }

    // 11. House South 1 - OPTIMIZED ⚡
    {
        let plan = floor_plans::house_2_floors(8.0, 9.0);
        spawn_building_optimized(
            commands,
            meshes,
            materials,
            plan,
            Vec3::new(18.0, 0.0, -31.0),
            -0.3,
            BuildingType::House,
        );
    }

    // 12. Cottage South 2 - OPTIMIZED ⚡
    {
        let plan = floor_plans::house_2_floors(7.0, 7.0);
        spawn_building_optimized(
            commands,
            meshes,
            materials,
            plan,
            Vec3::new(30.0, 0.0, -27.0),
            0.2,
            BuildingType::House,
        );
    }

    // WEST SIDE

    // 13. Cathedral/Temple - OPTIMIZED ⚡
    {
        let plan = floor_plans::cathedral(14.0, 20.0);
        spawn_building_optimized(
            commands,
            meshes,
            materials,
            plan,
            Vec3::new(-32.0, 0.0, 8.0),
            0.08,
            BuildingType::Temple,
        );
    }

    // 14. Library - OPTIMIZED ⚡
    {
        let plan = floor_plans::library(11.0, 11.0);
        spawn_building_optimized(
            commands,
            meshes,
            materials,
            plan,
            Vec3::new(-30.0, 0.0, -16.0),
            -0.15,
            BuildingType::Library,
        );
    }

    // 15. House West 1 - OPTIMIZED ⚡
    {
        let plan = floor_plans::house_2_floors(8.0, 8.0);
        spawn_building_optimized(
            commands,
            meshes,
            materials,
            plan,
            Vec3::new(-34.0, 0.0, -31.0),
            0.45,
            BuildingType::House,
        );
    }

    // 16. Workshop - OPTIMIZED ⚡
    {
        let plan = floor_plans::workshop(10.0, 8.0);
        spawn_building_optimized(
            commands,
            meshes,
            materials,
            plan,
            Vec3::new(-36.0, 0.0, 24.0),
            -0.3,
            BuildingType::Workshop,
        );
    }

    // 17. Chapel - OPTIMIZED ⚡
    {
        let plan = floor_plans::chapel(8.0, 7.0);
        spawn_building_optimized(
            commands,
            meshes,
            materials,
            plan,
            Vec3::new(-38.0, 0.0, -36.0),
            0.25,
            BuildingType::Chapel,
        );
    }

    // PLAZA DECORATIONS
    
    // Central Fountain
    let fountain_mesh = meshes.add(Cylinder::new(2.5, 3.0));
    let fountain_material = materials.add(materials::stone().to_standard_material());
    commands.spawn((
        PbrBundle {
            mesh: fountain_mesh,
            material: fountain_material,
            transform: Transform::from_xyz(-6.0, 1.5, 6.0),
            ..default()
        },
        bevy_rapier3d::prelude::RigidBody::Fixed,
        bevy_rapier3d::prelude::Collider::cylinder(1.5, 2.5),  // half_height, radius
        GameWorld,
    ));

    // Statue/Monument
    let statue_mesh = meshes.add(Cuboid::new(2.0, 5.0, 2.0));
    let statue_material = materials.add(materials::stone().to_standard_material());
    commands.spawn((
        PbrBundle {
            mesh: statue_mesh,
            material: statue_material,
            transform: Transform::from_xyz(8.0, 2.5, -8.0),
            ..default()
        },
        bevy_rapier3d::prelude::RigidBody::Fixed,
        bevy_rapier3d::prelude::Collider::cuboid(1.0, 2.5, 1.0),  // half-extents
        GameWorld,
    ));

    // Market Stall 1
    spawn_building!(
        "Market Stall 1",
        BuildingType::Market,
        Vec3::new(4.0, 3.0, 3.5),
        Vec3::new(22.0, 1.5, 4.0),
        0.6,
        materials::wood(),
        RoofType::Flat,
        Vec3::new(4.2, 0.3, 3.7),
        materials::roof_wood()
    );

    // Market Stall 2
    spawn_building!(
        "Market Stall 2",
        BuildingType::Market,
        Vec3::new(3.5, 3.0, 4.0),
        Vec3::new(20.0, 1.5, -3.0),
        -0.4,
        materials::wood(),
        RoofType::Flat,
        Vec3::new(3.7, 0.3, 4.2),
        materials::roof_wood()
    );

    // Market Stall 3
    spawn_building!(
        "Market Stall 3",
        BuildingType::Market,
        Vec3::new(4.0, 3.0, 3.0),
        Vec3::new(18.5, 1.5, 10.0),
        0.2,
        materials::wood(),
        RoofType::Flat,
        Vec3::new(4.2, 0.3, 3.2),
        materials::roof_wood()
    );
}

/// Spawns a multi-floor building with separate floors, trim, and color gradients
/// 
/// # OPTIMIZED MESH BATCHING
/// This creates visually distinct floors with:
/// - Separate entities for each floor
/// - Trim/molding between floors (thin dark wood strips)
/// - Color gradients (darker base → lighter top)
/// - Proper roof placement based on total height
/// 
/// # Arguments
/// * `floor_plan` - Complete floor configuration
/// * `base_position` - Ground position (Y coordinate is ground level)
/// * `rotation` - Y-axis rotation in radians
/// * `building_type` - Type tag for the building
/// 
/// # Returns
/// Entity ID of the root building entity (ground floor)
pub fn spawn_building_with_floors(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
    floor_plan: FloorPlan,
    base_position: Vec3,
    rotation: f32,
    building_type: BuildingType,
) -> Entity {
    let trim_height = 0.3; // Height of trim strips
    let trim_material_handle = materials.add(trim_material().to_standard_material());
    
    let mut current_y = base_position.y;
    
    // Spawn GROUND FLOOR
    let ground_height = floor_plan.ground_floor.height;
    let ground_mesh = meshes.add(Cuboid::new(
        floor_plan.width,
        ground_height,
        floor_plan.depth,
    ));
    let ground_material = materials.add(floor_plan.ground_floor.material.to_standard_material());
    
    let ground_entity = commands.spawn((
        PbrBundle {
            mesh: ground_mesh,
            material: ground_material,
            transform: Transform::from_xyz(
                base_position.x,
                current_y + ground_height / 2.0,
                base_position.z,
            ).with_rotation(Quat::from_rotation_y(rotation)),
            ..default()
        },
        Building,
        building_type,
        // OLD: AutoCollider removed - using Rapier physics
        GameWorld,
    )).id();
    
    current_y += ground_height;
    
    // Spawn MID FLOORS (if any)
    for floor_config in floor_plan.mid_floors.iter() {
        // Spawn trim below this floor if requested
        if floor_config.has_trim_below {
            let trim_mesh = meshes.add(Cuboid::new(
                floor_plan.width + 0.4, // Slightly wider
                trim_height,
                floor_plan.depth + 0.4,
            ));
            
            commands.spawn((
                PbrBundle {
                    mesh: trim_mesh,
                    material: trim_material_handle.clone(),
                    transform: Transform::from_xyz(
                        base_position.x,
                        current_y + trim_height / 2.0,
                        base_position.z,
                    ).with_rotation(Quat::from_rotation_y(rotation)),
                    ..default()
                },
                GameWorld,
            ));
            
            current_y += trim_height;
        }
        
        // Spawn the floor itself
        let floor_mesh = meshes.add(Cuboid::new(
            floor_plan.width,
            floor_config.height,
            floor_plan.depth,
        ));
        let floor_material = materials.add(floor_config.material.to_standard_material());
        
        commands.spawn((
            PbrBundle {
                mesh: floor_mesh,
                material: floor_material,
                transform: Transform::from_xyz(
                    base_position.x,
                    current_y + floor_config.height / 2.0,
                    base_position.z,
                ).with_rotation(Quat::from_rotation_y(rotation)),
                ..default()
            },
        // OLD: AutoCollider removed - using Rapier physics
            GameWorld,
        ));
        
        current_y += floor_config.height;
    }
    
    // Spawn TOP FLOOR (if height > 0)
    if floor_plan.top_floor.height > 0.0 {
        // Spawn trim if requested
        if floor_plan.top_floor.has_trim_below {
            let trim_mesh = meshes.add(Cuboid::new(
                floor_plan.width + 0.4,
                trim_height,
                floor_plan.depth + 0.4,
            ));
            
            commands.spawn((
                PbrBundle {
                    mesh: trim_mesh,
                    material: trim_material_handle.clone(),
                    transform: Transform::from_xyz(
                        base_position.x,
                        current_y + trim_height / 2.0,
                        base_position.z,
                    ).with_rotation(Quat::from_rotation_y(rotation)),
                    ..default()
                },
                GameWorld,
            ));
            
            current_y += trim_height;
        }
        
        // Spawn top floor
        let top_mesh = meshes.add(Cuboid::new(
            floor_plan.width,
            floor_plan.top_floor.height,
            floor_plan.depth,
        ));
        let top_material = materials.add(floor_plan.top_floor.material.to_standard_material());
        
        commands.spawn((
            PbrBundle {
                mesh: top_mesh,
                material: top_material,
                transform: Transform::from_xyz(
                    base_position.x,
                    current_y + floor_plan.top_floor.height / 2.0,
                    base_position.z,
                ).with_rotation(Quat::from_rotation_y(rotation)),
                ..default()
            },
        // OLD: AutoCollider removed - using Rapier physics
            GameWorld,
        ));
        
        current_y += floor_plan.top_floor.height;
    }
    
    // Spawn ROOF
    let roof_y = current_y + floor_plan.roof_height / 2.0;
    let roof_material_handle = materials.add(floor_plan.roof_material.to_standard_material());
    
    let roof_mesh = match floor_plan.roof_type {
        RoofType::Gabled => meshes.add(create_prism_roof(
            floor_plan.width + 0.5,
            floor_plan.depth + 0.5,
            floor_plan.roof_height,
        )),
        RoofType::Pyramid => meshes.add(create_pyramid_roof(
            floor_plan.width + 0.5,
            floor_plan.depth + 0.5,
            floor_plan.roof_height,
        )),
        RoofType::Flat => meshes.add(Cuboid::new(
            floor_plan.width,
            0.3,
            floor_plan.depth,
        )),
    };
    
    commands.spawn((
        PbrBundle {
            mesh: roof_mesh,
            material: roof_material_handle,
            transform: Transform::from_xyz(
                base_position.x,
                roof_y,
                base_position.z,
            ).with_rotation(Quat::from_rotation_y(rotation)),
            ..default()
        },
        GameWorld,
    ));
    
    // ===== STEP 3: ADD WINDOWS, DOORS, AND FACHWERK ===== 
    
    // Add door to ground floor
    spawn_door(
        commands,
        meshes,
        materials,
        base_position,
        rotation,
        floor_plan.width,
        floor_plan.depth,
        floor_plan.ground_floor.height,
        &floor_plan.details,
    );
    
    // Add windows and Fachwerk to each floor
    let mut floor_y = base_position.y + floor_plan.ground_floor.height / 2.0;
    
    // Ground floor details
    spawn_windows_for_floor(
        commands,
        meshes,
        materials,
        base_position,
        rotation,
        floor_plan.width,
        floor_plan.depth,
        floor_y,
        floor_plan.ground_floor.height,
        &floor_plan.details,
    );
    spawn_fachwerk_for_floor(
        commands,
        meshes,
        materials,
        base_position,
        rotation,
        floor_plan.width,
        floor_plan.depth,
        floor_y,
        floor_plan.ground_floor.height,
        &floor_plan.details,
    );
    
    floor_y = base_position.y + floor_plan.ground_floor.height;
    
    // Mid floors details
    for floor_config in &floor_plan.mid_floors {
        if floor_config.has_trim_below {
            floor_y += trim_height;
        }
        
        let this_floor_y = floor_y + floor_config.height / 2.0;
        
        spawn_windows_for_floor(
            commands,
            meshes,
            materials,
            base_position,
            rotation,
            floor_plan.width,
            floor_plan.depth,
            this_floor_y,
            floor_config.height,
            &floor_plan.details,
        );
        spawn_fachwerk_for_floor(
            commands,
            meshes,
            materials,
            base_position,
            rotation,
            floor_plan.width,
            floor_plan.depth,
            this_floor_y,
            floor_config.height,
            &floor_plan.details,
        );
        
        floor_y += floor_config.height;
    }
    
    // Top floor details (if exists)
    if floor_plan.top_floor.height > 0.0 {
        if floor_plan.top_floor.has_trim_below {
            floor_y += trim_height;
        }
        
        let this_floor_y = floor_y + floor_plan.top_floor.height / 2.0;
        
        spawn_windows_for_floor(
            commands,
            meshes,
            materials,
            base_position,
            rotation,
            floor_plan.width,
            floor_plan.depth,
            this_floor_y,
            floor_plan.top_floor.height,
            &floor_plan.details,
        );
        spawn_fachwerk_for_floor(
            commands,
            meshes,
            materials,
            base_position,
            rotation,
            floor_plan.width,
            floor_plan.depth,
            this_floor_y,
            floor_plan.top_floor.height,
            &floor_plan.details,
        );
    }
    
    // ===== STEP 4: ADD MATERIAL DETAILS =====
    
    // Add corner stones (quoins) to stone buildings
    let total_height = floor_plan.total_height();
    spawn_corner_stones(
        commands,
        meshes,
        materials,
        base_position,
        rotation,
        floor_plan.width,
        floor_plan.depth,
        total_height,
        &floor_plan.details,
    );
    
    // Add door trim if enabled
    if floor_plan.details.has_door_trim && floor_plan.details.has_door {
        let door_pos = base_position + Quat::from_rotation_y(rotation) * Vec3::new(
            0.0,
            floor_plan.details.door_size.y / 2.0,
            floor_plan.depth / 2.0 + 0.1,
        );
        let door_rotation = Quat::from_rotation_y(rotation);
        spawn_door_trim(
            commands,
            meshes,
            materials,
            door_pos,
            door_rotation,
            floor_plan.details.door_size,
        );
    }
    
    // Add roof ridge for gabled roofs
    spawn_roof_ridge(
        commands,
        meshes,
        materials,
        base_position + Vec3::new(0.0, total_height, 0.0),
        rotation,
        floor_plan.roof_type,
        floor_plan.width,
        floor_plan.depth,
        floor_plan.roof_height,
        &floor_plan.details,
    );
    
    // ===== STEP 5: ADD DECORATIONS (CHIMNEYS, LANTERNS, ETC.) =====
    
    // Add chimneys on roof
    spawn_chimneys(
        commands,
        meshes,
        materials,
        base_position,
        rotation,
        floor_plan.width,
        floor_plan.depth,
        total_height,
        &floor_plan.details.decorations,
    );
    
    // Add lanterns near entrance
    spawn_lanterns(
        commands,
        meshes,
        materials,
        base_position,
        rotation,
        floor_plan.width,
        floor_plan.depth,
        &floor_plan.details.decorations,
    );
    
    // Add barrels around building
    spawn_barrels(
        commands,
        meshes,
        materials,
        base_position,
        rotation,
        floor_plan.width,
        floor_plan.depth,
        &floor_plan.details.decorations,
    );
    
    // Add crates around building
    spawn_crates(
        commands,
        meshes,
        materials,
        base_position,
        rotation,
        floor_plan.width,
        floor_plan.depth,
        &floor_plan.details.decorations,
    );
    
    ground_entity
}
