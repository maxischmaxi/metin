use bevy::prelude::*;
use super::{MaterialConfig, RoofType, materials, BuildingDetails};

/// Configuration for a single floor of a building
#[derive(Clone)]
pub struct FloorConfig {
    pub height: f32,
    pub material: MaterialConfig,
    pub has_trim_below: bool,
}

/// Complete floor plan for a multi-story building
#[derive(Clone)]
pub struct FloorPlan {
    pub width: f32,
    pub depth: f32,
    pub ground_floor: FloorConfig,
    pub mid_floors: Vec<FloorConfig>,
    pub top_floor: FloorConfig,
    pub roof_type: RoofType,
    pub roof_height: f32,
    pub roof_material: MaterialConfig,
    pub details: BuildingDetails, // STEP 3: Architectural details
}

impl FloorPlan {
    /// Calculate total building height
    pub fn total_height(&self) -> f32 {
        let mut height = self.ground_floor.height;
        for floor in &self.mid_floors {
            height += floor.height;
        }
        height += self.top_floor.height;
        height
    }

    /// Get the number of floors
    pub fn floor_count(&self) -> usize {
        2 + self.mid_floors.len() // ground + top + mid floors
    }
}

/// Predefined floor plans for different building types
pub mod floor_plans {
    use super::*;
    use super::super::MaterialConfig;

    /// Creates a 3-story townhouse floor plan (Fachwerk style)
    /// Ground floor darker, mid floor medium, top floor lighter
    pub fn townhouse_3_floors(width: f32, depth: f32) -> FloorPlan {
        FloorPlan {
            width,
            depth,
            ground_floor: FloorConfig {
                height: 3.0,
                material: MaterialConfig {
                    base_color: Color::srgb(0.7, 0.65, 0.55), // Dark beige
                    roughness: 0.85,
                    metallic: 0.0,
                    reflectance: 0.3,
                },
                has_trim_below: false,
            },
            mid_floors: vec![FloorConfig {
                height: 3.0,
                material: MaterialConfig {
                    base_color: Color::srgb(0.8, 0.75, 0.6), // Medium beige
                    roughness: 0.85,
                    metallic: 0.0,
                    reflectance: 0.3,
                },
                has_trim_below: true,
            }],
            top_floor: FloorConfig {
                height: 3.0,
                material: MaterialConfig {
                    base_color: Color::srgb(0.85, 0.8, 0.65), // Light beige
                    roughness: 0.85,
                    metallic: 0.0,
                    reflectance: 0.3,
                },
                has_trim_below: true,
            },
            roof_type: RoofType::Gabled,
            roof_height: 2.5,
            roof_material: materials::roof_tiles_red(),
            details: BuildingDetails::townhouse(), // STEP 3
        }
    }

    /// Creates a 2-story house floor plan
    pub fn house_2_floors(width: f32, depth: f32) -> FloorPlan {
        FloorPlan {
            width,
            depth,
            ground_floor: FloorConfig {
                height: 3.5,
                material: MaterialConfig {
                    base_color: Color::srgb(0.75, 0.7, 0.6),
                    roughness: 0.85,
                    metallic: 0.0,
                    reflectance: 0.3,
                },
                has_trim_below: false,
            },
            mid_floors: vec![],
            top_floor: FloorConfig {
                height: 3.5,
                material: MaterialConfig {
                    base_color: Color::srgb(0.82, 0.77, 0.65),
                    roughness: 0.85,
                    metallic: 0.0,
                    reflectance: 0.3,
                },
                has_trim_below: true,
            },
            roof_type: RoofType::Gabled,
            roof_height: 2.0,
            roof_material: materials::roof_wood(),
            details: BuildingDetails::simple_house(), // STEP 3
        }
    }

    /// Creates a single-story building (workshop, warehouse)
    pub fn single_story(width: f32, depth: f32, height: f32, material: MaterialConfig) -> FloorPlan {
        FloorPlan {
            width,
            depth,
            ground_floor: FloorConfig {
                height,
                material: material.clone(),
                has_trim_below: false,
            },
            mid_floors: vec![],
            top_floor: FloorConfig {
                height: 0.0, // No separate top floor
                material: material.clone(),
                has_trim_below: false,
            },
            roof_type: RoofType::Gabled,
            roof_height: 1.5,
            roof_material: materials::roof_wood(),
            details: BuildingDetails::none(), // Single story buildings don't need windows/doors by default
        }
    }

    /// Creates a tall tower floor plan (4-5 floors)
    pub fn tower(width: f32, depth: f32, floors: usize) -> FloorPlan {
        let floor_height = 3.0;
        let mut mid_floors = Vec::new();

        // Create mid floors with gradual color lightening
        for i in 0..floors.saturating_sub(2) {
            let lightness = 0.4 + (i as f32 * 0.1);
            mid_floors.push(FloorConfig {
                height: floor_height,
                material: MaterialConfig {
                    base_color: Color::srgb(lightness, lightness, lightness + 0.05),
                    roughness: 0.9,
                    metallic: 0.0,
                    reflectance: 0.25,
                },
                has_trim_below: true,
            });
        }

        FloorPlan {
            width,
            depth,
            ground_floor: FloorConfig {
                height: floor_height,
                material: materials::stone(),
                has_trim_below: false,
            },
            mid_floors,
            top_floor: FloorConfig {
                height: floor_height,
                material: materials::light_stone(),
                has_trim_below: true,
            },
            roof_type: RoofType::Pyramid,
            roof_height: 2.5,
            roof_material: materials::roof_slate(),
            details: BuildingDetails::stone_building(), // Stone towers
        }
    }

    /// Creates a large inn/tavern floor plan (3 floors, spacious)
    pub fn inn(width: f32, depth: f32) -> FloorPlan {
        FloorPlan {
            width,
            depth,
            ground_floor: FloorConfig {
                height: 3.5, // Taller ground floor for tavern area
                material: MaterialConfig {
                    base_color: Color::srgb(0.55, 0.35, 0.18), // Dark wood
                    roughness: 0.9,
                    metallic: 0.0,
                    reflectance: 0.25,
                },
                has_trim_below: false,
            },
            mid_floors: vec![FloorConfig {
                height: 2.8,
                material: MaterialConfig {
                    base_color: Color::srgb(0.65, 0.45, 0.25), // Medium wood
                    roughness: 0.85,
                    metallic: 0.0,
                    reflectance: 0.3,
                },
                has_trim_below: true,
            }],
            top_floor: FloorConfig {
                height: 2.7,
                material: MaterialConfig {
                    base_color: Color::srgb(0.7, 0.5, 0.3), // Light wood
                    roughness: 0.85,
                    metallic: 0.0,
                    reflectance: 0.3,
                },
                has_trim_below: true,
            },
            roof_type: RoofType::Gabled,
            roof_height: 3.0,
            roof_material: materials::roof_tiles_red(),
            details: BuildingDetails::inn(), // Lots of windows and Fachwerk
        }
    }

    /// Creates a smithy floor plan (single tall floor)
    pub fn smithy(width: f32, depth: f32) -> FloorPlan {
        FloorPlan {
            width,
            depth,
            ground_floor: FloorConfig {
                height: 7.0, // Tall ceiling for forge
                material: materials::dark_stone(),
                has_trim_below: false,
            },
            mid_floors: vec![],
            top_floor: FloorConfig {
                height: 0.0,
                material: materials::dark_stone(),
                has_trim_below: false,
            },
            roof_type: RoofType::Gabled,
            roof_height: 2.5,
            roof_material: materials::roof_slate(),
            details: BuildingDetails::workshop(), // Smithy details
        }
    }

    /// Creates a cathedral floor plan (very tall, imposing)
    pub fn cathedral(width: f32, depth: f32) -> FloorPlan {
        FloorPlan {
            width,
            depth,
            ground_floor: FloorConfig {
                height: 16.0, // Very tall nave
                material: materials::light_stone(),
                has_trim_below: false,
            },
            mid_floors: vec![],
            top_floor: FloorConfig {
                height: 0.0,
                material: materials::light_stone(),
                has_trim_below: false,
            },
            roof_type: RoofType::Gabled,
            roof_height: 4.0,
            roof_material: materials::roof_slate(),
            details: BuildingDetails::stone_building(), // Cathedral - stone, tall windows
        }
    }

    /// Creates a library/study floor plan (2-3 stories, scholarly)
    pub fn library(width: f32, depth: f32) -> FloorPlan {
        FloorPlan {
            width,
            depth,
            ground_floor: FloorConfig {
                height: 4.0, // Tall ground floor for bookshelves
                material: MaterialConfig {
                    base_color: Color::srgb(0.4, 0.25, 0.15), // Dark wood/brown
                    roughness: 0.85,
                    metallic: 0.0,
                    reflectance: 0.25,
                },
                has_trim_below: false,
            },
            mid_floors: vec![FloorConfig {
                height: 3.0,
                material: MaterialConfig {
                    base_color: Color::srgb(0.5, 0.35, 0.2), // Medium brown
                    roughness: 0.85,
                    metallic: 0.0,
                    reflectance: 0.25,
                },
                has_trim_below: true,
            }],
            top_floor: FloorConfig {
                height: 3.0,
                material: MaterialConfig {
                    base_color: Color::srgb(0.55, 0.4, 0.25), // Light brown
                    roughness: 0.85,
                    metallic: 0.0,
                    reflectance: 0.3,
                },
                has_trim_below: true,
            },
            roof_type: RoofType::Pyramid,
            roof_height: 3.0,
            roof_material: materials::roof_wood(),
            details: BuildingDetails::simple_house(), // Library with standard windows
        }
    }

    /// Creates a workshop floor plan (1-2 floors, practical)
    pub fn workshop(width: f32, depth: f32) -> FloorPlan {
        FloorPlan {
            width,
            depth,
            ground_floor: FloorConfig {
                height: 5.0, // Tall ceiling for workspace
                material: materials::wood(),
                has_trim_below: false,
            },
            mid_floors: vec![],
            top_floor: FloorConfig {
                height: 0.0,
                material: materials::wood(),
                has_trim_below: false,
            },
            roof_type: RoofType::Gabled,
            roof_height: 2.0,
            roof_material: materials::roof_wood(),
            details: BuildingDetails::workshop(), // Workshop details
        }
    }

    /// Creates a chapel floor plan (tall single space, religious)
    pub fn chapel(width: f32, depth: f32) -> FloorPlan {
        FloorPlan {
            width,
            depth,
            ground_floor: FloorConfig {
                height: 9.0, // Tall vaulted ceiling
                material: materials::light_stone(),
                has_trim_below: false,
            },
            mid_floors: vec![],
            top_floor: FloorConfig {
                height: 0.0,
                material: materials::light_stone(),
                has_trim_below: false,
            },
            roof_type: RoofType::Gabled,
            roof_height: 2.5,
            roof_material: materials::roof_slate(),
            details: BuildingDetails::stone_building(), // Chapel - stone with tall windows
        }
    }

    /// Creates a warehouse floor plan (simple, functional)
    pub fn warehouse(width: f32, depth: f32) -> FloorPlan {
        FloorPlan {
            width,
            depth,
            ground_floor: FloorConfig {
                height: 4.5, // Medium-tall for storage
                material: materials::dark_wood(),
                has_trim_below: false,
            },
            mid_floors: vec![],
            top_floor: FloorConfig {
                height: 0.0,
                material: materials::dark_wood(),
                has_trim_below: false,
            },
            roof_type: RoofType::Gabled,
            roof_height: 1.5,
            roof_material: materials::roof_wood(),
            details: BuildingDetails::none(), // Warehouse - no details needed
        }
    }
}

/// Trim material (dark wood for floor separations)
pub fn trim_material() -> MaterialConfig {
    MaterialConfig {
        base_color: Color::srgb(0.25, 0.15, 0.1), // Very dark wood
        roughness: 0.9,
        metallic: 0.0,
        reflectance: 0.2,
    }
}
