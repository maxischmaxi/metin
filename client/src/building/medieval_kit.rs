/// Medieval Village Kit Integration - CORRECTED VERSION
/// 
/// Model Dimensions (from OBJ analysis):
/// - Walls: 2m wide × 3m tall × 0.2m deep, centered at (0, 1.5, 0)
/// - Floors: 2m × 2m tiles, centered at (0, 0, 0)
/// - Roofs: Size varies, positioned to cover building
/// 
/// The kit uses 2m modular units!

use bevy::prelude::*;
use crate::player::GameWorld;

/// Base path for medieval village models
const KIT_BASE: &str = "models/medieval_village/glTF/";

/// Helper to create full asset path for a model
pub fn kit_path(model_name: &str) -> String {
    format!("{}{}#Scene0", KIT_BASE, model_name)
}

/// Component to mark buildings as medieval kit models
#[derive(Component)]
pub struct MedievalKitBuilding;

/// Building template structure
pub struct BuildingTemplate {
    pub name: String,
    pub components: Vec<BuildingComponent>,
    /// Approximate collider size (half-extents) for Rapier
    pub collider_size: Vec3,
    /// Collider offset from root position
    pub collider_offset: Vec3,
}

/// A single component of a building (wall, roof, floor, etc.)
pub struct BuildingComponent {
    pub model_name: String,
    pub offset: Vec3,
    pub rotation: Quat,
    pub scale: Vec3,
}

impl BuildingComponent {
    pub fn new(model_name: &str) -> Self {
        Self {
            model_name: model_name.to_string(),
            offset: Vec3::ZERO,
            rotation: Quat::IDENTITY,
            scale: Vec3::ONE,
        }
    }

    pub fn with_offset(mut self, offset: Vec3) -> Self {
        self.offset = offset;
        self
    }

    pub fn with_rotation(mut self, rotation: Quat) -> Self {
        self.rotation = rotation;
        self
    }

    pub fn with_scale(mut self, scale: Vec3) -> Self {
        self.scale = scale;
        self
    }
}

/// Spawns a complete building from a template
pub fn spawn_building_from_template(
    commands: &mut Commands,
    asset_server: &AssetServer,
    template: BuildingTemplate,
    position: Vec3,
    rotation: f32,
) -> Entity {
    info!("Spawning medieval kit building: {} at {:?}", template.name, position);
    
    let base_rotation = Quat::from_rotation_y(rotation);
    
    // Create root entity with Rapier collider
    let root = commands.spawn((
        SpatialBundle {
            transform: Transform::from_translation(position)
                .with_rotation(base_rotation),
            ..default()
        },
        MedievalKitBuilding,
        GameWorld,
        Name::new(template.name.clone()),
        // Rapier physics - Fixed body for buildings
        bevy_rapier3d::prelude::RigidBody::Fixed,
    )).id();
    
    // Add collider as child entity
    commands.entity(root).with_children(|parent| {
        parent.spawn((
            TransformBundle::from(Transform::from_translation(template.collider_offset)),
            bevy_rapier3d::prelude::Collider::cuboid(
                template.collider_size.x,
                template.collider_size.y,
                template.collider_size.z,
            ),
        ));
    });
    
    // Spawn all visual components as children
    for component in template.components {
        let scene_path = kit_path(&component.model_name);
        let scene: Handle<Scene> = asset_server.load(&scene_path);
        
        commands.entity(root).with_children(|parent| {
            parent.spawn((
                SceneBundle {
                    scene,
                    transform: Transform {
                        translation: component.offset,
                        rotation: component.rotation,
                        scale: component.scale,
                    },
                    ..default()
                },
                GameWorld,
            ));
        });
    }
    
    root
}

/// Simple helper to spawn a single kit model
pub fn spawn_kit_model(
    commands: &mut Commands,
    asset_server: &AssetServer,
    model_name: &str,
    position: Vec3,
    rotation: Quat,
    scale: Vec3,
) -> Entity {
    let scene_path = kit_path(model_name);
    let scene: Handle<Scene> = asset_server.load(&scene_path);
    
    commands.spawn((
        SceneBundle {
            scene,
            transform: Transform {
                translation: position,
                rotation,
                scale,
            },
            ..default()
        },
        GameWorld,
        Name::new(model_name.to_string()),
    )).id()
}

// ==================== BUILDING TEMPLATES (CORRECTED) ====================

/// Small house - Single 2x2m room
/// Floor is 2m×2m, walls are 2m wide, so we need 1 wall per side
pub fn template_house_small() -> BuildingTemplate {
    // Wall dimensions: 2m wide, 3m tall, center at (0, 1.5, 0)
    // Floor dimensions: 2m×2m, center at (0, 0, 0)
    // We need walls at the edges: distance = floor_radius + wall_thickness/2 = 1.0 + 0.1 = 1.1
    
    BuildingTemplate {
        name: "Small House".to_string(),
        components: vec![
            // Floor (2m×2m)
            BuildingComponent::new("Floor_WoodDark.gltf")
                .with_offset(Vec3::new(0.0, 0.0, 0.0)),
            
            // North wall (front with door)
            BuildingComponent::new("Wall_Plaster_Door_Round.gltf")
                .with_offset(Vec3::new(0.0, 1.5, 1.1))
                .with_rotation(Quat::IDENTITY), // Facing south (door faces 0,0,0)
            
            // South wall (back)
            BuildingComponent::new("Wall_Plaster_Straight.gltf")
                .with_offset(Vec3::new(0.0, 1.5, -1.1))
                .with_rotation(Quat::from_rotation_y(std::f32::consts::PI)),
            
            // East wall (right with window)
            BuildingComponent::new("Wall_Plaster_Window_Wide_Round.gltf")
                .with_offset(Vec3::new(1.1, 1.5, 0.0))
                .with_rotation(Quat::from_rotation_y(-std::f32::consts::PI / 2.0)),
            
            // West wall (left with window)
            BuildingComponent::new("Wall_Plaster_Window_Wide_Round.gltf")
                .with_offset(Vec3::new(-1.1, 1.5, 0.0))
                .with_rotation(Quat::from_rotation_y(std::f32::consts::PI / 2.0)),
            
            // Roof (4x4 covers a 2x2m building with overhang)
            BuildingComponent::new("Roof_RoundTiles_4x4.gltf")
                .with_offset(Vec3::new(0.0, 3.0, 0.0)),
        ],
        // Collider: 2m×3m×2m building
        collider_size: Vec3::new(1.2, 1.5, 1.2), // Slightly larger than 2x3x2
        collider_offset: Vec3::new(0.0, 1.5, 0.0), // Center at half height
    }
}

/// Medium house - 4x4m (2×2 floor tiles)
pub fn template_house_medium() -> BuildingTemplate {
    // For 4x4m we need 2 floor tiles and 2 walls per side
    
    BuildingTemplate {
        name: "Medium House".to_string(),
        components: vec![
            // Floors (4 tiles for 4x4m)
            BuildingComponent::new("Floor_RedBrick.gltf")
                .with_offset(Vec3::new(-1.0, 0.0, -1.0)),
            BuildingComponent::new("Floor_RedBrick.gltf")
                .with_offset(Vec3::new(1.0, 0.0, -1.0)),
            BuildingComponent::new("Floor_RedBrick.gltf")
                .with_offset(Vec3::new(-1.0, 0.0, 1.0)),
            BuildingComponent::new("Floor_RedBrick.gltf")
                .with_offset(Vec3::new(1.0, 0.0, 1.0)),
            
            // North walls (2 walls = 4m)
            BuildingComponent::new("Wall_UnevenBrick_Door_Flat.gltf")
                .with_offset(Vec3::new(-1.0, 1.5, 2.1))
                .with_rotation(Quat::IDENTITY),
            BuildingComponent::new("Wall_UnevenBrick_Window_Wide_Flat.gltf")
                .with_offset(Vec3::new(1.0, 1.5, 2.1))
                .with_rotation(Quat::IDENTITY),
            
            // South walls
            BuildingComponent::new("Wall_UnevenBrick_Straight.gltf")
                .with_offset(Vec3::new(-1.0, 1.5, -2.1))
                .with_rotation(Quat::from_rotation_y(std::f32::consts::PI)),
            BuildingComponent::new("Wall_UnevenBrick_Window_Wide_Round.gltf")
                .with_offset(Vec3::new(1.0, 1.5, -2.1))
                .with_rotation(Quat::from_rotation_y(std::f32::consts::PI)),
            
            // East walls
            BuildingComponent::new("Wall_UnevenBrick_Window_Thin_Round.gltf")
                .with_offset(Vec3::new(2.1, 1.5, -1.0))
                .with_rotation(Quat::from_rotation_y(-std::f32::consts::PI / 2.0)),
            BuildingComponent::new("Wall_UnevenBrick_Straight.gltf")
                .with_offset(Vec3::new(2.1, 1.5, 1.0))
                .with_rotation(Quat::from_rotation_y(-std::f32::consts::PI / 2.0)),
            
            // West walls
            BuildingComponent::new("Wall_UnevenBrick_Window_Thin_Round.gltf")
                .with_offset(Vec3::new(-2.1, 1.5, -1.0))
                .with_rotation(Quat::from_rotation_y(std::f32::consts::PI / 2.0)),
            BuildingComponent::new("Wall_UnevenBrick_Straight.gltf")
                .with_offset(Vec3::new(-2.1, 1.5, 1.0))
                .with_rotation(Quat::from_rotation_y(std::f32::consts::PI / 2.0)),
            
            // Roof
            BuildingComponent::new("Roof_RoundTiles_6x6.gltf")
                .with_offset(Vec3::new(0.0, 3.0, 0.0)),
        ],
        collider_size: Vec3::new(2.2, 1.5, 2.2),
        collider_offset: Vec3::new(0.0, 1.5, 0.0),
    }
}

/// Tavern - Large 6x4m building
pub fn template_tavern() -> BuildingTemplate {
    BuildingTemplate {
        name: "Tavern".to_string(),
        components: vec![
            // Floor 6x4m (3×2 tiles)
            BuildingComponent::new("Floor_WoodLight.gltf")
                .with_offset(Vec3::new(-2.0, 0.0, -1.0)),
            BuildingComponent::new("Floor_WoodLight.gltf")
                .with_offset(Vec3::new(0.0, 0.0, -1.0)),
            BuildingComponent::new("Floor_WoodLight.gltf")
                .with_offset(Vec3::new(2.0, 0.0, -1.0)),
            BuildingComponent::new("Floor_WoodLight.gltf")
                .with_offset(Vec3::new(-2.0, 0.0, 1.0)),
            BuildingComponent::new("Floor_WoodLight.gltf")
                .with_offset(Vec3::new(0.0, 0.0, 1.0)),
            BuildingComponent::new("Floor_WoodLight.gltf")
                .with_offset(Vec3::new(2.0, 0.0, 1.0)),
            
            // Front (North) - 3 walls
            BuildingComponent::new("Wall_Plaster_Door_Round.gltf")
                .with_offset(Vec3::new(0.0, 1.5, 2.1)),
            BuildingComponent::new("Wall_Plaster_Window_Wide_Flat.gltf")
                .with_offset(Vec3::new(-2.0, 1.5, 2.1)),
            BuildingComponent::new("Wall_Plaster_Window_Wide_Flat.gltf")
                .with_offset(Vec3::new(2.0, 1.5, 2.1)),
            
            // Back (South) - 3 walls
            BuildingComponent::new("Wall_Plaster_Straight.gltf")
                .with_offset(Vec3::new(0.0, 1.5, -2.1))
                .with_rotation(Quat::from_rotation_y(std::f32::consts::PI)),
            BuildingComponent::new("Wall_Plaster_Straight.gltf")
                .with_offset(Vec3::new(-2.0, 1.5, -2.1))
                .with_rotation(Quat::from_rotation_y(std::f32::consts::PI)),
            BuildingComponent::new("Wall_Plaster_Window_Wide_Round.gltf")
                .with_offset(Vec3::new(2.0, 1.5, -2.1))
                .with_rotation(Quat::from_rotation_y(std::f32::consts::PI)),
            
            // Sides - 2 walls each
            BuildingComponent::new("Wall_Plaster_Window_Wide_Round.gltf")
                .with_offset(Vec3::new(3.1, 1.5, -1.0))
                .with_rotation(Quat::from_rotation_y(-std::f32::consts::PI / 2.0)),
            BuildingComponent::new("Wall_Plaster_Window_Wide_Round.gltf")
                .with_offset(Vec3::new(3.1, 1.5, 1.0))
                .with_rotation(Quat::from_rotation_y(-std::f32::consts::PI / 2.0)),
            BuildingComponent::new("Wall_Plaster_Straight.gltf")
                .with_offset(Vec3::new(-3.1, 1.5, -1.0))
                .with_rotation(Quat::from_rotation_y(std::f32::consts::PI / 2.0)),
            BuildingComponent::new("Wall_Plaster_Straight.gltf")
                .with_offset(Vec3::new(-3.1, 1.5, 1.0))
                .with_rotation(Quat::from_rotation_y(std::f32::consts::PI / 2.0)),
            
            // Roof
            BuildingComponent::new("Roof_RoundTiles_6x8.gltf")
                .with_offset(Vec3::new(0.0, 3.0, 0.0)),
        ],
        collider_size: Vec3::new(3.2, 1.5, 2.2),
        collider_offset: Vec3::new(0.0, 1.5, 0.0),
    }
}

/// Church - Large 6x6m building with tall windows
pub fn template_church() -> BuildingTemplate {
    BuildingTemplate {
        name: "Church".to_string(),
        components: vec![
            // Floor 6x6m (3×3 tiles)
            BuildingComponent::new("Floor_Brick.gltf")
                .with_offset(Vec3::new(-2.0, 0.0, -2.0)),
            BuildingComponent::new("Floor_Brick.gltf")
                .with_offset(Vec3::new(0.0, 0.0, -2.0)),
            BuildingComponent::new("Floor_Brick.gltf")
                .with_offset(Vec3::new(2.0, 0.0, -2.0)),
            BuildingComponent::new("Floor_Brick.gltf")
                .with_offset(Vec3::new(-2.0, 0.0, 0.0)),
            BuildingComponent::new("Floor_Brick.gltf")
                .with_offset(Vec3::new(0.0, 0.0, 0.0)),
            BuildingComponent::new("Floor_Brick.gltf")
                .with_offset(Vec3::new(2.0, 0.0, 0.0)),
            BuildingComponent::new("Floor_Brick.gltf")
                .with_offset(Vec3::new(-2.0, 0.0, 2.0)),
            BuildingComponent::new("Floor_Brick.gltf")
                .with_offset(Vec3::new(0.0, 0.0, 2.0)),
            BuildingComponent::new("Floor_Brick.gltf")
                .with_offset(Vec3::new(2.0, 0.0, 2.0)),
            
            // Front (North) - arched door
            BuildingComponent::new("Wall_Plaster_Door_RoundInset.gltf")
                .with_offset(Vec3::new(0.0, 1.5, 3.1)),
            BuildingComponent::new("Wall_Plaster_Straight.gltf")
                .with_offset(Vec3::new(-2.0, 1.5, 3.1)),
            BuildingComponent::new("Wall_Plaster_Straight.gltf")
                .with_offset(Vec3::new(2.0, 1.5, 3.1)),
            
            // Sides with tall windows (3 walls per side)
            BuildingComponent::new("Wall_Plaster_Window_Thin_Round.gltf")
                .with_offset(Vec3::new(3.1, 1.5, -2.0))
                .with_rotation(Quat::from_rotation_y(-std::f32::consts::PI / 2.0)),
            BuildingComponent::new("Wall_Plaster_Window_Thin_Round.gltf")
                .with_offset(Vec3::new(3.1, 1.5, 0.0))
                .with_rotation(Quat::from_rotation_y(-std::f32::consts::PI / 2.0)),
            BuildingComponent::new("Wall_Plaster_Window_Thin_Round.gltf")
                .with_offset(Vec3::new(3.1, 1.5, 2.0))
                .with_rotation(Quat::from_rotation_y(-std::f32::consts::PI / 2.0)),
            
            BuildingComponent::new("Wall_Plaster_Window_Thin_Round.gltf")
                .with_offset(Vec3::new(-3.1, 1.5, -2.0))
                .with_rotation(Quat::from_rotation_y(std::f32::consts::PI / 2.0)),
            BuildingComponent::new("Wall_Plaster_Window_Thin_Round.gltf")
                .with_offset(Vec3::new(-3.1, 1.5, 0.0))
                .with_rotation(Quat::from_rotation_y(std::f32::consts::PI / 2.0)),
            BuildingComponent::new("Wall_Plaster_Window_Thin_Round.gltf")
                .with_offset(Vec3::new(-3.1, 1.5, 2.0))
                .with_rotation(Quat::from_rotation_y(std::f32::consts::PI / 2.0)),
            
            // Back wall
            BuildingComponent::new("Wall_Plaster_Straight.gltf")
                .with_offset(Vec3::new(-2.0, 1.5, -3.1))
                .with_rotation(Quat::from_rotation_y(std::f32::consts::PI)),
            BuildingComponent::new("Wall_Plaster_Straight.gltf")
                .with_offset(Vec3::new(0.0, 1.5, -3.1))
                .with_rotation(Quat::from_rotation_y(std::f32::consts::PI)),
            BuildingComponent::new("Wall_Plaster_Straight.gltf")
                .with_offset(Vec3::new(2.0, 1.5, -3.1))
                .with_rotation(Quat::from_rotation_y(std::f32::consts::PI)),
            
            // Large roof
            BuildingComponent::new("Roof_RoundTiles_6x10.gltf")
                .with_offset(Vec3::new(0.0, 3.0, 0.0)),
        ],
        collider_size: Vec3::new(3.2, 1.5, 3.2),
        collider_offset: Vec3::new(0.0, 1.5, 0.0),
    }
}

/// Smithy - 4x4m with chimney
pub fn template_smithy() -> BuildingTemplate {
    BuildingTemplate {
        name: "Smithy".to_string(),
        components: vec![
            // Floor 4x4m
            BuildingComponent::new("Floor_UnevenBrick.gltf")
                .with_offset(Vec3::new(-1.0, 0.0, -1.0)),
            BuildingComponent::new("Floor_UnevenBrick.gltf")
                .with_offset(Vec3::new(1.0, 0.0, -1.0)),
            BuildingComponent::new("Floor_UnevenBrick.gltf")
                .with_offset(Vec3::new(-1.0, 0.0, 1.0)),
            BuildingComponent::new("Floor_UnevenBrick.gltf")
                .with_offset(Vec3::new(1.0, 0.0, 1.0)),
            
            // Walls with large door and ventilation
            BuildingComponent::new("Wall_UnevenBrick_Door_Flat.gltf")
                .with_offset(Vec3::new(0.0, 1.5, 2.1)),
            BuildingComponent::new("Wall_UnevenBrick_Straight.gltf")
                .with_offset(Vec3::new(0.0, 1.5, 2.1)),
            
            BuildingComponent::new("Wall_UnevenBrick_Window_Wide_Flat.gltf")
                .with_offset(Vec3::new(2.1, 1.5, 0.0))
                .with_rotation(Quat::from_rotation_y(-std::f32::consts::PI / 2.0)),
            BuildingComponent::new("Wall_UnevenBrick_Window_Wide_Flat.gltf")
                .with_offset(Vec3::new(-2.1, 1.5, 0.0))
                .with_rotation(Quat::from_rotation_y(std::f32::consts::PI / 2.0)),
            
            BuildingComponent::new("Wall_UnevenBrick_Straight.gltf")
                .with_offset(Vec3::new(-1.0, 1.5, -2.1))
                .with_rotation(Quat::from_rotation_y(std::f32::consts::PI)),
            BuildingComponent::new("Wall_UnevenBrick_Straight.gltf")
                .with_offset(Vec3::new(1.0, 1.5, -2.1))
                .with_rotation(Quat::from_rotation_y(std::f32::consts::PI)),
            
            // Roof
            BuildingComponent::new("Roof_RoundTiles_6x6.gltf")
                .with_offset(Vec3::new(0.0, 3.0, 0.0)),
            
            // Chimney
            BuildingComponent::new("Prop_Chimney.gltf")
                .with_offset(Vec3::new(-1.0, 3.5, -1.0)),
        ],
        collider_size: Vec3::new(2.2, 1.5, 2.2),
        collider_offset: Vec3::new(0.0, 1.5, 0.0),
    }
}

/// Market stall - Simple 2x2m with open front
pub fn template_market_stall() -> BuildingTemplate {
    BuildingTemplate {
        name: "Market Stall".to_string(),
        components: vec![
            // Floor
            BuildingComponent::new("Floor_WoodDark.gltf")
                .with_offset(Vec3::new(0.0, 0.0, 0.0)),
            
            // Back wall only
            BuildingComponent::new("Wall_Plaster_Straight.gltf")
                .with_offset(Vec3::new(0.0, 1.5, -1.1))
                .with_rotation(Quat::from_rotation_y(std::f32::consts::PI)),
            
            // Simple roof
            BuildingComponent::new("Roof_Modular_RoundTiles.gltf")
                .with_offset(Vec3::new(0.0, 2.5, 0.0)),
            
            // Props
            BuildingComponent::new("Prop_Crate.gltf")
                .with_offset(Vec3::new(0.5, 0.0, 0.3)),
            BuildingComponent::new("Prop_Crate.gltf")
                .with_offset(Vec3::new(-0.5, 0.0, 0.3)),
        ],
        collider_size: Vec3::new(1.2, 1.5, 1.2),
        collider_offset: Vec3::new(0.0, 1.5, 0.0),
    }
}

/// Tower - 4x4m tall defensive structure (2 floors)
pub fn template_tower() -> BuildingTemplate {
    BuildingTemplate {
        name: "Tower".to_string(),
        components: vec![
            // Floor 4x4m (ground level)
            BuildingComponent::new("Floor_Brick.gltf")
                .with_offset(Vec3::new(-1.0, 0.0, -1.0)),
            BuildingComponent::new("Floor_Brick.gltf")
                .with_offset(Vec3::new(1.0, 0.0, -1.0)),
            BuildingComponent::new("Floor_Brick.gltf")
                .with_offset(Vec3::new(-1.0, 0.0, 1.0)),
            BuildingComponent::new("Floor_Brick.gltf")
                .with_offset(Vec3::new(1.0, 0.0, 1.0)),
            
            // Ground floor walls - solid brick
            BuildingComponent::new("Wall_UnevenBrick_Straight.gltf")
                .with_offset(Vec3::new(-1.0, 1.5, 2.1)),
            BuildingComponent::new("Wall_UnevenBrick_Straight.gltf")
                .with_offset(Vec3::new(1.0, 1.5, 2.1)),
            
            BuildingComponent::new("Wall_UnevenBrick_Straight.gltf")
                .with_offset(Vec3::new(-1.0, 1.5, -2.1))
                .with_rotation(Quat::from_rotation_y(std::f32::consts::PI)),
            BuildingComponent::new("Wall_UnevenBrick_Straight.gltf")
                .with_offset(Vec3::new(1.0, 1.5, -2.1))
                .with_rotation(Quat::from_rotation_y(std::f32::consts::PI)),
            
            BuildingComponent::new("Wall_UnevenBrick_Straight.gltf")
                .with_offset(Vec3::new(2.1, 1.5, -1.0))
                .with_rotation(Quat::from_rotation_y(-std::f32::consts::PI / 2.0)),
            BuildingComponent::new("Wall_UnevenBrick_Straight.gltf")
                .with_offset(Vec3::new(2.1, 1.5, 1.0))
                .with_rotation(Quat::from_rotation_y(-std::f32::consts::PI / 2.0)),
            
            BuildingComponent::new("Wall_UnevenBrick_Straight.gltf")
                .with_offset(Vec3::new(-2.1, 1.5, -1.0))
                .with_rotation(Quat::from_rotation_y(std::f32::consts::PI / 2.0)),
            BuildingComponent::new("Wall_UnevenBrick_Straight.gltf")
                .with_offset(Vec3::new(-2.1, 1.5, 1.0))
                .with_rotation(Quat::from_rotation_y(std::f32::consts::PI / 2.0)),
            
            // Second floor walls - with arrow slits
            BuildingComponent::new("Wall_UnevenBrick_Window_Thin_Round.gltf")
                .with_offset(Vec3::new(0.0, 4.5, 2.1)),
            
            BuildingComponent::new("Wall_UnevenBrick_Window_Thin_Round.gltf")
                .with_offset(Vec3::new(0.0, 4.5, -2.1))
                .with_rotation(Quat::from_rotation_y(std::f32::consts::PI)),
            
            BuildingComponent::new("Wall_UnevenBrick_Window_Thin_Round.gltf")
                .with_offset(Vec3::new(2.1, 4.5, 0.0))
                .with_rotation(Quat::from_rotation_y(-std::f32::consts::PI / 2.0)),
            
            BuildingComponent::new("Wall_UnevenBrick_Window_Thin_Round.gltf")
                .with_offset(Vec3::new(-2.1, 4.5, 0.0))
                .with_rotation(Quat::from_rotation_y(std::f32::consts::PI / 2.0)),
            
            // Roof/Top
            BuildingComponent::new("Roof_RoundTiles_4x4.gltf")
                .with_offset(Vec3::new(0.0, 6.0, 0.0)),
        ],
        collider_size: Vec3::new(2.2, 3.0, 2.2), // Tall building
        collider_offset: Vec3::new(0.0, 3.0, 0.0),
    }
}
