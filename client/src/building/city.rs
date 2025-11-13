use bevy::prelude::*;
use crate::player::GameWorld;
use super::medieval_kit::*;

/// Spawns all city buildings using the Medieval Village Kit
/// 
/// Complete city rebuild with modular medieval assets!
/// Layout: Central plaza (40x40m) with buildings around it
/// - Player spawns at (0,1,0)
/// - NPC at (5,1,5)
/// - Buildings arranged organically around plaza
pub fn spawn_city_buildings(
    commands: &mut Commands,
    asset_server: &AssetServer,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
) {
    info!("🏰 Building medieval city with kit assets...");
    
    // ==================== NORTH SIDE ====================
    
    // 1. Large Tavern/Inn - "The Golden Dragon"
    spawn_building_from_template(
        commands,
        asset_server,
        template_tavern(),
        Vec3::new(-8.0, 0.0, 28.0),
        0.15,
    );
    
    // 2. Smithy - "Ironforge"
    spawn_building_from_template(
        commands,
        asset_server,
        template_smithy(),
        Vec3::new(-28.0, 0.0, 28.0),
        -0.2,
    );
    
    // 3. Medium House North 1
    spawn_building_from_template(
        commands,
        asset_server,
        template_house_medium(),
        Vec3::new(24.0, 0.0, 30.0),
        0.3,
    );
    
    // 4. Small House North 2
    spawn_building_from_template(
        commands,
        asset_server,
        template_house_small(),
        Vec3::new(32.0, 0.0, 26.0),
        -0.4,
    );
    
    // 5. Tower - "Watchtower"
    spawn_building_from_template(
        commands,
        asset_server,
        template_tower(),
        Vec3::new(35.0, 0.0, 35.0),
        0.0,
    );
    
    // ==================== EAST SIDE ====================
    
    // 6. Medium House East 1
    spawn_building_from_template(
        commands,
        asset_server,
        template_house_medium(),
        Vec3::new(30.0, 0.0, 10.0),
        0.1,
    );
    
    // 7. Small House East 2
    spawn_building_from_template(
        commands,
        asset_server,
        template_house_small(),
        Vec3::new(31.0, 0.0, -8.0),
        -0.25,
    );
    
    // 8. Medium House East 3
    spawn_building_from_template(
        commands,
        asset_server,
        template_house_medium(),
        Vec3::new(27.0, 0.0, -20.0),
        0.5,
    );
    
    // ==================== SOUTH SIDE ====================
    
    // 9. Smithy South - "Blackforge"
    spawn_building_from_template(
        commands,
        asset_server,
        template_smithy(),
        Vec3::new(2.0, 0.0, -30.0),
        -0.1,
    );
    
    // 10. Small House South 1
    spawn_building_from_template(
        commands,
        asset_server,
        template_house_small(),
        Vec3::new(18.0, 0.0, -31.0),
        -0.3,
    );
    
    // 11. Small Cottage South 2
    spawn_building_from_template(
        commands,
        asset_server,
        template_house_small(),
        Vec3::new(30.0, 0.0, -27.0),
        0.2,
    );
    
    // ==================== WEST SIDE ====================
    
    // 12. Church/Cathedral - "St. Michael's"
    spawn_building_from_template(
        commands,
        asset_server,
        template_church(),
        Vec3::new(-32.0, 0.0, 8.0),
        0.08,
    );
    
    // 13. Medium House West 1
    spawn_building_from_template(
        commands,
        asset_server,
        template_house_medium(),
        Vec3::new(-30.0, 0.0, -16.0),
        -0.15,
    );
    
    // 14. Small House West 2
    spawn_building_from_template(
        commands,
        asset_server,
        template_house_small(),
        Vec3::new(-34.0, 0.0, -31.0),
        0.45,
    );
    
    // 15. Tavern West - "The Rusty Sword"
    spawn_building_from_template(
        commands,
        asset_server,
        template_tavern(),
        Vec3::new(-36.0, 0.0, 24.0),
        -0.3,
    );
    
    // ==================== PLAZA (Central Market) ====================
    
    // Market Stall 1 - East side
    spawn_building_from_template(
        commands,
        asset_server,
        template_market_stall(),
        Vec3::new(22.0, 0.0, 4.0),
        0.6,
    );
    
    // Market Stall 2 - East side
    spawn_building_from_template(
        commands,
        asset_server,
        template_market_stall(),
        Vec3::new(20.0, 0.0, -3.0),
        -0.4,
    );
    
    // Market Stall 3 - North side
    spawn_building_from_template(
        commands,
        asset_server,
        template_market_stall(),
        Vec3::new(18.5, 0.0, 10.0),
        0.2,
    );
    
    // ==================== DECORATIONS & PROPS ====================
    
    // Central Fountain (keep the existing fountain)
    let fountain_mesh = meshes.add(Mesh::from(Cylinder::new(2.5, 3.0)));
    let fountain_material = materials.add(StandardMaterial {
        base_color: Color::srgb(0.5, 0.5, 0.55),
        perceptual_roughness: 0.9,
        ..default()
    });
    commands.spawn((
        PbrBundle {
            mesh: fountain_mesh,
            material: fountain_material,
            transform: Transform::from_xyz(-6.0, 1.5, 6.0),
            ..default()
        },
        bevy_rapier3d::prelude::RigidBody::Fixed,
        bevy_rapier3d::prelude::Collider::cylinder(1.5, 2.5),
        GameWorld,
    ));
    
    // Crates scattered around plaza
    spawn_kit_model(
        commands,
        asset_server,
        "Prop_Crate",
        Vec3::new(12.0, 0.0, 8.0),
        Quat::from_rotation_y(0.5),
        Vec3::ONE,
    );
    
    spawn_kit_model(
        commands,
        asset_server,
        "Prop_Crate",
        Vec3::new(10.0, 0.0, -6.0),
        Quat::from_rotation_y(-0.3),
        Vec3::ONE,
    );
    
    spawn_kit_model(
        commands,
        asset_server,
        "Prop_Crate",
        Vec3::new(-14.0, 0.0, 12.0),
        Quat::from_rotation_y(0.8),
        Vec3::ONE,
    );
    
    // Wagon
    spawn_kit_model(
        commands,
        asset_server,
        "Prop_Wagon",
        Vec3::new(-10.0, 0.0, -10.0),
        Quat::from_rotation_y(0.4),
        Vec3::ONE,
    );
    
    // Wooden Fences around plaza edges
    spawn_kit_model(
        commands,
        asset_server,
        "Prop_WoodenFence_Single",
        Vec3::new(18.0, 0.0, 18.0),
        Quat::IDENTITY,
        Vec3::ONE,
    );
    
    spawn_kit_model(
        commands,
        asset_server,
        "Prop_WoodenFence_Single",
        Vec3::new(-18.0, 0.0, 18.0),
        Quat::from_rotation_y(std::f32::consts::PI / 2.0),
        Vec3::ONE,
    );
    
    // Vines on some buildings for atmosphere
    spawn_kit_model(
        commands,
        asset_server,
        "Prop_Vine1",
        Vec3::new(-8.0, 0.0, 32.0), // On tavern wall
        Quat::IDENTITY,
        Vec3::ONE,
    );
    
    spawn_kit_model(
        commands,
        asset_server,
        "Prop_Vine2",
        Vec3::new(-32.0, 0.0, 12.0), // On church wall
        Quat::from_rotation_y(std::f32::consts::PI / 2.0),
        Vec3::ONE,
    );
    
    info!("✅ Medieval city complete - {} buildings spawned!", 15 + 3); // 15 main buildings + 3 market stalls
}

// Note: Old procedural system functions removed
// The city now uses the Medieval Village Kit exclusively
// All buildings have automatic Rapier colliders
// Layout preserved from original design
