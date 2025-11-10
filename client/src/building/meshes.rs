use bevy::prelude::*;
use bevy::render::mesh::{Indices, PrimitiveTopology};

/// Creates a prism mesh for gabled roofs (Satteldach)
/// 
/// # Arguments
/// * `width` - Width of the roof (X axis)
/// * `length` - Length of the roof (Z axis)
/// * `height` - Height of the roof peak (Y axis)
pub fn create_prism_roof(width: f32, length: f32, height: f32) -> Mesh {
    let half_width = width / 2.0;
    let half_length = length / 2.0;

    // Vertices for a triangular prism (roof)
    // Bottom rectangle + top ridge line
    let vertices = vec![
        // Bottom front-left
        [-half_width, 0.0, half_length],
        // Bottom front-right
        [half_width, 0.0, half_length],
        // Bottom back-right
        [half_width, 0.0, -half_length],
        // Bottom back-left
        [-half_width, 0.0, -half_length],
        // Top ridge front (peak)
        [0.0, height, half_length],
        // Top ridge back (peak)
        [0.0, height, -half_length],
    ];

    // Normals for each vertex
    let normals = vec![
        // Bottom face normals (pointing down)
        [0.0, -1.0, 0.0],
        [0.0, -1.0, 0.0],
        [0.0, -1.0, 0.0],
        [0.0, -1.0, 0.0],
        // Top ridge normals (average of adjacent faces)
        [0.0, 0.707, 0.0],
        [0.0, 0.707, 0.0],
    ];

    // UV coordinates
    let uvs = vec![
        [0.0, 0.0],
        [1.0, 0.0],
        [1.0, 1.0],
        [0.0, 1.0],
        [0.5, 0.0],
        [0.5, 1.0],
    ];

    // Indices for triangles
    // Each face is made of triangles
    let indices = vec![
        // Front triangular face
        0, 4, 1,
        // Back triangular face
        3, 2, 5,
        // Left sloped face
        0, 3, 5,
        0, 5, 4,
        // Right sloped face
        1, 4, 5,
        1, 5, 2,
        // Bottom face
        0, 1, 2,
        0, 2, 3,
    ];

    let mut mesh = Mesh::new(PrimitiveTopology::TriangleList, default());
    mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, vertices);
    mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, normals);
    mesh.insert_attribute(Mesh::ATTRIBUTE_UV_0, uvs);
    mesh.insert_indices(Indices::U32(indices));

    mesh
}

/// Creates a pyramid mesh for pyramid roofs
/// 
/// # Arguments
/// * `width` - Width of the roof base (X axis)
/// * `length` - Length of the roof base (Z axis)
/// * `height` - Height of the pyramid peak (Y axis)
pub fn create_pyramid_roof(width: f32, length: f32, height: f32) -> Mesh {
    let half_width = width / 2.0;
    let half_length = length / 2.0;

    // Vertices: 4 base corners + 1 apex
    let vertices = vec![
        // Base corners
        [-half_width, 0.0, half_length],   // 0: front-left
        [half_width, 0.0, half_length],    // 1: front-right
        [half_width, 0.0, -half_length],   // 2: back-right
        [-half_width, 0.0, -half_length],  // 3: back-left
        // Apex
        [0.0, height, 0.0],                // 4: top
    ];

    // Calculate normals for sloped faces
    // For a pyramid, each face has its own normal
    let normals = vec![
        [0.0, -1.0, 0.0],  // Base corners point down
        [0.0, -1.0, 0.0],
        [0.0, -1.0, 0.0],
        [0.0, -1.0, 0.0],
        [0.0, 1.0, 0.0],   // Apex points up
    ];

    // UV coordinates
    let uvs = vec![
        [0.0, 0.0],
        [1.0, 0.0],
        [1.0, 1.0],
        [0.0, 1.0],
        [0.5, 0.5],  // Center for apex
    ];

    // Indices for triangles
    let indices = vec![
        // Front face
        0, 4, 1,
        // Right face
        1, 4, 2,
        // Back face
        2, 4, 3,
        // Left face
        3, 4, 0,
        // Bottom face (two triangles)
        0, 1, 2,
        0, 2, 3,
    ];

    let mut mesh = Mesh::new(PrimitiveTopology::TriangleList, default());
    mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, vertices);
    mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, normals);
    mesh.insert_attribute(Mesh::ATTRIBUTE_UV_0, uvs);
    mesh.insert_indices(Indices::U32(indices));

    mesh
}

/// Helper function to spawn a building with a roof
/// 
/// # Arguments
/// * `commands` - Bevy commands for spawning entities
/// * `meshes` - Mesh asset storage
/// * `materials` - Material asset storage
/// * `wall_mesh` - Mesh for the building walls
/// * `wall_material` - Material config for walls
/// * `wall_transform` - Transform for the walls
/// * `roof_type` - Type of roof to add
/// * `roof_size` - Size of the roof (width, height, length)
/// * `roof_material` - Material config for the roof
pub fn spawn_building_with_roof(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
    wall_mesh: Handle<Mesh>,
    wall_material: super::MaterialConfig,
    wall_transform: Transform,
    roof_type: super::RoofType,
    roof_size: Vec3,  // (width, height, length)
    roof_material: super::MaterialConfig,
) -> Entity {
    use super::RoofType;

    // Calculate roof position (on top of walls)
    let wall_height = wall_transform.scale.y;
    let roof_y_offset = wall_height / 2.0 + roof_size.y / 2.0;

    // Create roof mesh based on type
    let roof_mesh = match roof_type {
        RoofType::Gabled => create_prism_roof(roof_size.x, roof_size.z, roof_size.y),
        RoofType::Pyramid => create_pyramid_roof(roof_size.x, roof_size.z, roof_size.y),
        RoofType::Flat => {
            // Use a simple cuboid for flat roofs
            Cuboid::new(roof_size.x, 0.3, roof_size.z).into()
        }
    };

    // Spawn the building with walls as parent
    let building_entity = commands
        .spawn(PbrBundle {
            mesh: wall_mesh,
            material: materials.add(wall_material.to_standard_material()),
            transform: wall_transform,
            ..default()
        })
        .with_children(|parent| {
            // Add roof as child entity
            parent.spawn(PbrBundle {
                mesh: meshes.add(roof_mesh),
                material: materials.add(roof_material.to_standard_material()),
                transform: Transform::from_xyz(0.0, roof_y_offset, 0.0),
                ..default()
            });
        })
        .id();

    building_entity
}
