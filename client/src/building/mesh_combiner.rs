use bevy::prelude::*;
use bevy::render::mesh::{Indices, PrimitiveTopology};

/// Combines multiple quads/meshes into a single optimized mesh
/// This dramatically reduces draw calls and CPU overhead
pub struct MeshBuilder {
    positions: Vec<[f32; 3]>,
    normals: Vec<[f32; 3]>,
    uvs: Vec<[f32; 2]>,
    indices: Vec<u32>,
}

impl Default for MeshBuilder {
    fn default() -> Self {
        Self::new()
    }
}

impl MeshBuilder {
    pub fn new() -> Self {
        Self {
            positions: Vec::new(),
            normals: Vec::new(),
            uvs: Vec::new(),
            indices: Vec::new(),
        }
    }

    /// Adds a quad (rectangle) to the mesh
    /// 
    /// # Arguments
    /// * `center` - Center position of the quad
    /// * `size` - Size (width, height)
    /// * `rotation` - Rotation quaternion
    /// * `normal` - Surface normal direction
    pub fn add_quad(
        &mut self,
        center: Vec3,
        size: Vec2,
        rotation: Quat,
        normal: Vec3,
    ) {
        let half_w = size.x / 2.0;
        let half_h = size.y / 2.0;

        // Local quad vertices (in XY plane)
        let local_vertices = [
            Vec3::new(-half_w, -half_h, 0.0), // Bottom-left
            Vec3::new(half_w, -half_h, 0.0),  // Bottom-right
            Vec3::new(half_w, half_h, 0.0),   // Top-right
            Vec3::new(-half_w, half_h, 0.0),  // Top-left
        ];

        // Transform to world space
        let base_index = self.positions.len() as u32;
        
        for local_vert in local_vertices.iter() {
            let world_vert = center + rotation * *local_vert;
            self.positions.push(world_vert.to_array());
            self.normals.push(normal.to_array());
        }

        // UVs for texture mapping
        self.uvs.push([0.0, 0.0]);
        self.uvs.push([1.0, 0.0]);
        self.uvs.push([1.0, 1.0]);
        self.uvs.push([0.0, 1.0]);

        // Two triangles forming a quad
        self.indices.extend_from_slice(&[
            base_index,
            base_index + 1,
            base_index + 2,
            base_index,
            base_index + 2,
            base_index + 3,
        ]);
    }

    /// Adds a cuboid (box) to the mesh
    pub fn add_cuboid(&mut self, center: Vec3, size: Vec3, rotation: Quat) {
        let half_x = size.x / 2.0;
        let half_y = size.y / 2.0;
        let half_z = size.z / 2.0;

        // Front face (+Z)
        self.add_quad(
            center + rotation * Vec3::new(0.0, 0.0, half_z),
            Vec2::new(size.x, size.y),
            rotation * Quat::from_rotation_y(0.0),
            rotation * Vec3::Z,
        );

        // Back face (-Z)
        self.add_quad(
            center + rotation * Vec3::new(0.0, 0.0, -half_z),
            Vec2::new(size.x, size.y),
            rotation * Quat::from_rotation_y(std::f32::consts::PI),
            rotation * Vec3::NEG_Z,
        );

        // Left face (-X)
        self.add_quad(
            center + rotation * Vec3::new(-half_x, 0.0, 0.0),
            Vec2::new(size.z, size.y),
            rotation * Quat::from_rotation_y(-std::f32::consts::PI / 2.0),
            rotation * Vec3::NEG_X,
        );

        // Right face (+X)
        self.add_quad(
            center + rotation * Vec3::new(half_x, 0.0, 0.0),
            Vec2::new(size.z, size.y),
            rotation * Quat::from_rotation_y(std::f32::consts::PI / 2.0),
            rotation * Vec3::X,
        );

        // Top face (+Y)
        self.add_quad(
            center + rotation * Vec3::new(0.0, half_y, 0.0),
            Vec2::new(size.x, size.z),
            rotation * Quat::from_rotation_x(-std::f32::consts::PI / 2.0),
            rotation * Vec3::Y,
        );

        // Bottom face (-Y)
        self.add_quad(
            center + rotation * Vec3::new(0.0, -half_y, 0.0),
            Vec2::new(size.x, size.z),
            rotation * Quat::from_rotation_x(std::f32::consts::PI / 2.0),
            rotation * Vec3::NEG_Y,
        );
    }

    /// Adds a cylinder to the mesh (approximated with segments)
    pub fn add_cylinder(&mut self, center: Vec3, radius: f32, height: f32, segments: u32, rotation: Quat) {
        let half_height = height / 2.0;
        
        // Generate vertices for top and bottom circles
        for i in 0..segments {
            let angle = (i as f32 / segments as f32) * std::f32::consts::TAU;
            let next_angle = ((i + 1) as f32 / segments as f32) * std::f32::consts::TAU;
            
            let x1 = angle.cos() * radius;
            let z1 = angle.sin() * radius;
            let x2 = next_angle.cos() * radius;
            let z2 = next_angle.sin() * radius;

            // Side quad
            let p1 = center + rotation * Vec3::new(x1, -half_height, z1);
            let p2 = center + rotation * Vec3::new(x2, -half_height, z2);
            let p3 = center + rotation * Vec3::new(x2, half_height, z2);
            let p4 = center + rotation * Vec3::new(x1, half_height, z1);

            let base_index = self.positions.len() as u32;
            self.positions.push(p1.to_array());
            self.positions.push(p2.to_array());
            self.positions.push(p3.to_array());
            self.positions.push(p4.to_array());

            let normal = rotation * Vec3::new(angle.cos(), 0.0, angle.sin());
            self.normals.push(normal.to_array());
            self.normals.push(normal.to_array());
            self.normals.push(normal.to_array());
            self.normals.push(normal.to_array());

            self.uvs.push([i as f32 / segments as f32, 0.0]);
            self.uvs.push([(i + 1) as f32 / segments as f32, 0.0]);
            self.uvs.push([(i + 1) as f32 / segments as f32, 1.0]);
            self.uvs.push([i as f32 / segments as f32, 1.0]);

            self.indices.extend_from_slice(&[
                base_index,
                base_index + 1,
                base_index + 2,
                base_index,
                base_index + 2,
                base_index + 3,
            ]);
        }
    }

    /// Builds the final Bevy mesh from collected geometry
    pub fn build(self) -> Mesh {
        let mut mesh = Mesh::new(PrimitiveTopology::TriangleList, default());
        mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, self.positions);
        mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, self.normals);
        mesh.insert_attribute(Mesh::ATTRIBUTE_UV_0, self.uvs);
        mesh.insert_indices(Indices::U32(self.indices));
        mesh
    }

    /// Returns true if the mesh is empty
    pub fn is_empty(&self) -> bool {
        self.positions.is_empty()
    }
}

/// Helper function to create a quad mesh (for windows, doors, etc.)
pub fn create_quad_mesh(size: Vec2) -> Mesh {
    let mut builder = MeshBuilder::new();
    builder.add_quad(Vec3::ZERO, size, Quat::IDENTITY, Vec3::Z);
    builder.build()
}

/// Helper function to create a cuboid mesh
pub fn create_cuboid_mesh(size: Vec3) -> Mesh {
    let mut builder = MeshBuilder::new();
    builder.add_cuboid(Vec3::ZERO, size, Quat::IDENTITY);
    builder.build()
}

/// Helper function to create a cylinder mesh
pub fn create_cylinder_mesh(radius: f32, height: f32, segments: u32) -> Mesh {
    let mut builder = MeshBuilder::new();
    builder.add_cylinder(Vec3::ZERO, radius, height, segments, Quat::IDENTITY);
    builder.build()
}
