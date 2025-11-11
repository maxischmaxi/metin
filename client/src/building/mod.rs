use bevy::prelude::*;

pub mod meshes;
pub mod city;
pub mod floors;
pub mod details;
pub mod decorations;
pub mod mesh_combiner;
pub mod optimized_spawner;

pub use meshes::*;
pub use city::{spawn_city_buildings, spawn_building_with_floors};
pub use floors::*;
pub use details::*;
pub use decorations::*;
pub use mesh_combiner::*;
pub use optimized_spawner::*;

/// Building plugin for enhanced medieval buildings
pub struct BuildingPlugin;

impl Plugin for BuildingPlugin {
    fn build(&self, app: &mut App) {
        // Future: Add building-specific systems here
    }
}

/// Marker component for buildings
#[derive(Component)]
pub struct Building;

/// Types of buildings in the city
#[derive(Debug, Clone, Copy, PartialEq, Component)]
pub enum BuildingType {
    Inn,
    House,
    Temple,
    Workshop,
    Market,
    Smithy,
    Tower,
    Chapel,
    Library,
}

/// Roof types for buildings
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum RoofType {
    /// Gabled roof (Satteldach) - two slopes meeting at a ridge
    Gabled,
    /// Pyramid roof (Pyramidendach) - four slopes meeting at a point
    Pyramid,
    /// Flat roof
    Flat,
}

/// Configuration for PBR materials
#[derive(Clone)]
pub struct MaterialConfig {
    pub base_color: Color,
    pub roughness: f32,
    pub metallic: f32,
    pub reflectance: f32,
}

impl MaterialConfig {
    /// Create a standard material from this config
    pub fn to_standard_material(&self) -> StandardMaterial {
        StandardMaterial {
            base_color: self.base_color,
            perceptual_roughness: self.roughness,
            metallic: self.metallic,
            reflectance: self.reflectance,
            ..default()
        }
    }
}

/// Predefined material configurations for medieval buildings
pub mod materials {
    use super::*;
    
    pub use super::MaterialConfig;

    /// Wooden material (brown, matte)
    pub fn wood() -> MaterialConfig {
        MaterialConfig {
            base_color: Color::srgb(0.6, 0.4, 0.2),
            roughness: 0.85,
            metallic: 0.0,
            reflectance: 0.25,
        }
    }

    /// Dark wood material (for beams, doors)
    pub fn dark_wood() -> MaterialConfig {
        MaterialConfig {
            base_color: Color::srgb(0.3, 0.2, 0.15),
            roughness: 0.9,
            metallic: 0.0,
            reflectance: 0.2,
        }
    }

    /// Stone material (gray, rough)
    pub fn stone() -> MaterialConfig {
        MaterialConfig {
            base_color: Color::srgb(0.5, 0.5, 0.55),
            roughness: 0.9,
            metallic: 0.0,
            reflectance: 0.3,
        }
    }

    /// Light stone (for temples, chapels)
    pub fn light_stone() -> MaterialConfig {
        MaterialConfig {
            base_color: Color::srgb(0.9, 0.9, 0.85),
            roughness: 0.85,
            metallic: 0.0,
            reflectance: 0.35,
        }
    }

    /// Plaster/render material (beige, matte)
    pub fn plaster() -> MaterialConfig {
        MaterialConfig {
            base_color: Color::srgb(0.8, 0.75, 0.6),
            roughness: 0.8,
            metallic: 0.0,
            reflectance: 0.3,
        }
    }

    /// Brick material (red/orange)
    pub fn brick() -> MaterialConfig {
        MaterialConfig {
            base_color: Color::srgb(0.7, 0.3, 0.2),
            roughness: 0.85,
            metallic: 0.0,
            reflectance: 0.25,
        }
    }

    /// Red roof tiles
    pub fn roof_tiles_red() -> MaterialConfig {
        MaterialConfig {
            base_color: Color::srgb(0.75, 0.3, 0.2),
            roughness: 0.8,
            metallic: 0.0,
            reflectance: 0.3,
        }
    }

    /// Gray slate roof
    pub fn roof_slate() -> MaterialConfig {
        MaterialConfig {
            base_color: Color::srgb(0.4, 0.4, 0.45),
            roughness: 0.75,
            metallic: 0.0,
            reflectance: 0.35,
        }
    }

    /// Brown wooden roof
    pub fn roof_wood() -> MaterialConfig {
        MaterialConfig {
            base_color: Color::srgb(0.5, 0.35, 0.2),
            roughness: 0.85,
            metallic: 0.0,
            reflectance: 0.25,
        }
    }

    /// Dark gray stone (for smithies)
    pub fn dark_stone() -> MaterialConfig {
        MaterialConfig {
            base_color: Color::srgb(0.25, 0.25, 0.3),
            roughness: 0.9,
            metallic: 0.0,
            reflectance: 0.25,
        }
    }
}
