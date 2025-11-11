use bevy::prelude::*;
use bevy::render::mesh::VertexAttributeValues;  // For mesh extraction
use crate::GameState;
use std::collections::HashMap;
use rayon::prelude::*;  // Phase 4: Multi-Threading
use std::sync::Arc;  // Thread-safe data structures

pub struct CollisionPlugin;

impl Plugin for CollisionPlugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<SpatialGrid>()  // Phase 3: Spatial Partitioning
            .init_resource::<CollisionDebugConfig>()  // Phase 6: Visual Debugging
            .add_event::<CollisionStarted>()
            .add_event::<CollisionEnded>()
            .add_event::<TriggerEntered>()
            .add_systems(Update, (
                update_collision_lod,     // Phase 5: Update LOD based on distance
                generate_colliders,       // Auto-generate colliders from meshes
                create_collision_caches,  // Phase 4: Create cache for new colliders
                update_collision_caches,  // Phase 4: Update cache when transform changes
                update_spatial_grid,      // Update spatial grid
                detect_collisions,        // Detect collisions (uses cache for broad-phase)
                update_colliding_with,
                resolve_collisions,
                toggle_collision_debug,   // Phase 6: F1 to toggle debug view
                draw_collision_debug,     // Phase 6: Draw debug wireframes
            ).chain().run_if(in_state(GameState::InGame)));
    }
}

/// Marker component for collidable entities
#[derive(Component, Debug, Clone)]
pub struct Collider {
    pub shape: ColliderShape,
    pub collision_type: CollisionType,
    pub layer: CollisionLayer,  // Phase 3: Collision Layers
}

/// Collision Layer - determines what can collide with what
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CollisionLayer {
    /// Players (collide with: World, NPCs, Monsters, other Players)
    Player,
    /// NPCs (collide with: World, Players, Monsters)
    NPC,
    /// Monsters (collide with: World, Players, NPCs, other Monsters)
    Monster,
    /// World geometry (collide with: everything)
    World,
    /// Items (collide with: World only)
    Item,
    /// Projectiles (collide with: Players, NPCs, Monsters, World)
    Projectile,
}

impl CollisionLayer {
    /// Check if this layer can collide with another layer
    pub fn can_collide_with(&self, other: &CollisionLayer) -> bool {
        match (self, other) {
            // World collides with everything
            (CollisionLayer::World, _) | (_, CollisionLayer::World) => true,
            
            // Players collide with NPCs, Monsters, other Players
            (CollisionLayer::Player, CollisionLayer::NPC) 
            | (CollisionLayer::NPC, CollisionLayer::Player) => true,
            (CollisionLayer::Player, CollisionLayer::Monster) 
            | (CollisionLayer::Monster, CollisionLayer::Player) => true,
            (CollisionLayer::Player, CollisionLayer::Player) => true,
            
            // NPCs collide with Monsters
            (CollisionLayer::NPC, CollisionLayer::Monster) 
            | (CollisionLayer::Monster, CollisionLayer::NPC) => true,
            
            // Monsters collide with other Monsters
            (CollisionLayer::Monster, CollisionLayer::Monster) => true,
            
            // Projectiles collide with Players, NPCs, Monsters
            (CollisionLayer::Projectile, CollisionLayer::Player)
            | (CollisionLayer::Player, CollisionLayer::Projectile) => true,
            (CollisionLayer::Projectile, CollisionLayer::NPC)
            | (CollisionLayer::NPC, CollisionLayer::Projectile) => true,
            (CollisionLayer::Projectile, CollisionLayer::Monster)
            | (CollisionLayer::Monster, CollisionLayer::Projectile) => true,
            
            // Items don't collide with anything except World
            (CollisionLayer::Item, _) | (_, CollisionLayer::Item) => false,
            
            // Projectiles don't collide with other Projectiles
            (CollisionLayer::Projectile, CollisionLayer::Projectile) => false,
            
            // Default: no collision
            _ => false,
        }
    }
}

/// Shape of the collider
#[derive(Debug, Clone, Copy)]
pub enum ColliderShape {
    /// Cylindrical collider (radius, height)
    /// Used for characters (Player, NPCs, Monsters)
    Cylinder { radius: f32, height: f32 },
    
    /// Sphere collider (radius)
    /// Used for items, projectiles, simple objects
    Sphere { radius: f32 },
    
    /// AABB Box collider (half-extents from center)
    /// Used for buildings, walls, static objects
    Box { half_extents: Vec3 },
}

/// Type of collision behavior
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CollisionType {
    /// Can move, collides with everything
    Dynamic,
    
    /// Can't move, blocks dynamic objects
    Static,
    
    /// Triggers events but doesn't block movement
    Trigger,
}

/// Stores entities this collider is currently touching
#[derive(Component, Default, Debug)]
pub struct CollidingWith {
    pub entities: Vec<Entity>,
}

/// Optional: Push-back force when colliding
#[derive(Component, Debug, Clone)]
pub struct CollisionPushback {
    /// 0.0 = no pushback (ghost), 1.0 = full stop/pushback
    pub strength: f32,
}

impl Default for CollisionPushback {
    fn default() -> Self {
        Self { strength: 0.8 }
    }
}

// ==================== PHASE 4: COLLISION CACHE ====================

/// Cache for fast collision broad-phase checks
/// Only updates when Transform changes, avoiding recalculation every frame
#[derive(Component, Debug, Clone)]
pub struct CollisionCache {
    /// Bounding sphere (center in local space, radius)
    pub bounding_sphere: (Vec3, f32),
    
    /// Axis-Aligned Bounding Box (min, max in world space)
    pub aabb: (Vec3, Vec3),
    
    /// Last known world position (to detect Transform changes)
    pub last_position: Vec3,
    
    /// Last known rotation (to detect Transform changes)
    pub last_rotation: Quat,
}

impl CollisionCache {
    /// Create cache from collider shape
    pub fn from_shape(shape: &ColliderShape, position: Vec3, rotation: Quat) -> Self {
        let (local_center, radius) = match shape {
            ColliderShape::Cylinder { radius, height } => {
                let center = Vec3::new(0.0, height / 2.0, 0.0);
                let r = radius.max(height / 2.0);
                (center, r)
            }
            ColliderShape::Sphere { radius } => {
                (Vec3::ZERO, *radius)
            }
            ColliderShape::Box { half_extents } => {
                let center = Vec3::ZERO;
                let r = half_extents.length();
                (center, r)
            }
        };
        
        // Calculate world-space AABB
        let aabb = Self::calculate_aabb(shape, position, rotation);
        
        Self {
            bounding_sphere: (local_center, radius),
            aabb,
            last_position: position,
            last_rotation: rotation,
        }
    }
    
    /// Calculate AABB from shape and transform
    fn calculate_aabb(shape: &ColliderShape, position: Vec3, rotation: Quat) -> (Vec3, Vec3) {
        match shape {
            ColliderShape::Cylinder { radius, height } => {
                // Approximate cylinder as box for AABB
                let half_extents = Vec3::new(*radius, height / 2.0, *radius);
                let rotated = rotation.mul_vec3(half_extents).abs();
                (position - rotated, position + rotated)
            }
            ColliderShape::Sphere { radius } => {
                let r = Vec3::splat(*radius);
                (position - r, position + r)
            }
            ColliderShape::Box { half_extents } => {
                let rotated = rotation.mul_vec3(*half_extents).abs();
                (position - rotated, position + rotated)
            }
        }
    }
    
    /// Update cache with new transform
    pub fn update(&mut self, shape: &ColliderShape, position: Vec3, rotation: Quat) {
        self.aabb = Self::calculate_aabb(shape, position, rotation);
        self.last_position = position;
        self.last_rotation = rotation;
    }
    
    /// Check if transform has changed (needs cache update)
    pub fn needs_update(&self, position: Vec3, rotation: Quat) -> bool {
        (self.last_position - position).length_squared() > 0.0001
            || self.last_rotation.angle_between(rotation) > 0.001
    }
    
    /// Get world-space bounding sphere center
    pub fn world_bounding_sphere_center(&self, position: Vec3, rotation: Quat) -> Vec3 {
        position + rotation.mul_vec3(self.bounding_sphere.0)
    }
    
    /// Get bounding sphere radius
    pub fn bounding_sphere_radius(&self) -> f32 {
        self.bounding_sphere.1
    }
    
    /// Fast AABB overlap test
    pub fn aabb_overlaps(&self, other: &CollisionCache) -> bool {
        let (min_a, max_a) = self.aabb;
        let (min_b, max_b) = other.aabb;
        
        max_a.x >= min_b.x && min_a.x <= max_b.x
            && max_a.y >= min_b.y && min_a.y <= max_b.y
            && max_a.z >= min_b.z && min_a.z <= max_b.z
    }
    
    /// Fast sphere overlap test (broad-phase)
    pub fn sphere_overlaps(&self, position_a: Vec3, rotation_a: Quat, other: &CollisionCache, position_b: Vec3, rotation_b: Quat) -> bool {
        let center_a = self.world_bounding_sphere_center(position_a, rotation_a);
        let center_b = other.world_bounding_sphere_center(position_b, rotation_b);
        let distance_sq = (center_a - center_b).length_squared();
        let combined_radius = self.bounding_sphere_radius() + other.bounding_sphere_radius();
        
        distance_sq < combined_radius * combined_radius
    }
}

/// Event sent when collision starts
#[derive(Event, Debug)]
pub struct CollisionStarted {
    pub entity_a: Entity,
    pub entity_b: Entity,
    pub contact_point: Vec3,
    pub penetration_depth: f32,
}

/// Event sent when collision ends
#[derive(Event, Debug)]
pub struct CollisionEnded {
    pub entity_a: Entity,
    pub entity_b: Entity,
}

/// Event for trigger zones
#[derive(Event, Debug)]
pub struct TriggerEntered {
    pub trigger: Entity,
    pub entered_by: Entity,
}

/// Collision information struct
#[derive(Debug)]
pub struct CollisionInfo {
    pub penetration_depth: f32,
    pub normal: Vec3, // Points from A to B
    pub contact_point: Vec3,
}

// ==================== AUTO-COLLIDER SYSTEM (Phase 1) ====================

/// Collision Detail Level (LOD for collision shapes)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CollisionDetail {
    /// Coarse collision (Bounding Box/Sphere)
    /// Performance: Best
    /// Accuracy: Lowest
    /// Use Case: Many objects, far away, simple shapes
    Low,
    
    /// Medium collision (Simplified Hull, few vertices)
    /// Performance: Medium
    /// Accuracy: Medium
    /// Use Case: Standard objects, normal distance
    Medium,
    
    /// Detailed collision (Convex Hull, many vertices)
    /// Performance: Slow
    /// Accuracy: Highest
    /// Use Case: Important objects, near player, complex shapes
    High,
}

/// Preferred shape strategy for auto-generation
#[derive(Debug, Clone, Copy)]
pub enum PreferredShape {
    /// Always use Bounding Box (fast, inaccurate)
    Box,
    
    /// Always use Sphere (very fast, very inaccurate)
    Sphere,
    
    /// Use Convex Hull (slow, accurate)
    ConvexHull,
    
    /// Automatically choose best shape
    Auto,
}

/// Component for automatic collision generation from mesh
#[derive(Component, Clone)]
pub struct AutoCollider {
    /// Which detail level to use
    pub detail: CollisionDetail,
    
    /// Collision type (Dynamic, Static, Trigger)
    pub collision_type: CollisionType,
    
    /// Collision layer
    pub layer: CollisionLayer,
    
    /// Optional: Manual padding (enlarges/shrinks collision)
    pub padding: f32,
    
    /// Optional: Which shape strategy to prefer
    pub preferred_shape: Option<PreferredShape>,
}

impl Default for AutoCollider {
    fn default() -> Self {
        Self {
            detail: CollisionDetail::Medium,
            collision_type: CollisionType::Static,
            layer: CollisionLayer::World,
            padding: 0.0,
            preferred_shape: Some(PreferredShape::Auto),
        }
    }
}

// ==================== PHASE 5: LOD SYSTEM ====================

/// Level-of-Detail configuration for collision shapes
/// Automatically switches collision detail based on distance to camera
#[derive(Component, Clone, Debug)]
pub struct CollisionLOD {
    /// Enable automatic LOD switching
    pub auto_switch: bool,
    
    /// Distance thresholds [High, Medium, Low]
    /// - Distance < distances[0] = High Detail
    /// - Distance < distances[1] = Medium Detail
    /// - Distance >= distances[1] = Low Detail
    pub distances: [f32; 2],
    
    /// Hysteresis (prevents flickering at boundaries)
    /// Switch threshold is increased by this amount when switching back
    pub hysteresis: f32,
    
    /// Last known detail level (for change detection)
    last_detail: CollisionDetail,
}

impl CollisionLOD {
    /// Create LOD with default distances
    /// High < 10m, Medium < 30m, Low >= 30m
    pub fn new() -> Self {
        Self {
            auto_switch: true,
            distances: [10.0, 30.0],
            hysteresis: 2.0,
            last_detail: CollisionDetail::Medium,
        }
    }
    
    /// Create LOD with custom distances
    pub fn with_distances(high_threshold: f32, medium_threshold: f32) -> Self {
        Self {
            auto_switch: true,
            distances: [high_threshold, medium_threshold],
            hysteresis: 2.0,
            last_detail: CollisionDetail::Medium,
        }
    }
    
    /// Create LOD with custom distances and hysteresis
    pub fn with_hysteresis(high_threshold: f32, medium_threshold: f32, hysteresis: f32) -> Self {
        Self {
            auto_switch: true,
            distances: [high_threshold, medium_threshold],
            hysteresis,
            last_detail: CollisionDetail::Medium,
        }
    }
    
    /// Determine detail level from distance
    pub fn detail_for_distance(&mut self, distance: f32) -> CollisionDetail {
        // Apply hysteresis to prevent flickering
        let (high_threshold, medium_threshold) = match self.last_detail {
            CollisionDetail::High => {
                // When at High, make it harder to switch to Medium
                (self.distances[0] + self.hysteresis, self.distances[1])
            }
            CollisionDetail::Medium => {
                // When at Medium, make it harder to switch in either direction
                (self.distances[0] - self.hysteresis, self.distances[1] + self.hysteresis)
            }
            CollisionDetail::Low => {
                // When at Low, make it harder to switch to Medium
                (self.distances[0], self.distances[1] - self.hysteresis)
            }
        };
        
        let new_detail = if distance < high_threshold {
            CollisionDetail::High
        } else if distance < medium_threshold {
            CollisionDetail::Medium
        } else {
            CollisionDetail::Low
        };
        
        self.last_detail = new_detail;
        new_detail
    }
    
    /// Disable automatic switching
    pub fn disable(&mut self) {
        self.auto_switch = false;
    }
    
    /// Enable automatic switching
    pub fn enable(&mut self) {
        self.auto_switch = true;
    }
}

impl Default for CollisionLOD {
    fn default() -> Self {
        Self::new()
    }
}

/// Generated collider (created at runtime from mesh)
#[derive(Component, Clone)]
pub struct GeneratedCollider {
    /// The generated shape
    pub shape: GeneratedShape,
    
    /// Source mesh it was generated from
    pub source_mesh: Handle<Mesh>,
    
    /// Which detail level was used
    pub detail_used: CollisionDetail,
}

/// Generated collision shapes
#[derive(Debug, Clone)]
pub enum GeneratedShape {
    /// Simple Bounding Box (6 faces)
    BoundingBox { min: Vec3, max: Vec3 },
    
    /// Bounding Sphere
    BoundingSphere { center: Vec3, radius: f32 },
    
    /// Convex Hull (list of vertices + faces)
    ConvexHull { vertices: Vec<Vec3>, faces: Vec<[usize; 3]> },
    
    /// Simplified Mesh (reduced vertex count)
    SimplifiedMesh { vertices: Vec<Vec3>, indices: Vec<u32> },
}

impl GeneratedShape {
    /// Convert to Collider for collision detection
    pub fn to_collider(&self) -> ColliderShape {
        match self {
            GeneratedShape::BoundingBox { min, max } => {
                let center = (*min + *max) / 2.0;
                let half_extents = (*max - center).abs();
                ColliderShape::Box { half_extents }
            }
            GeneratedShape::BoundingSphere { center: _, radius } => {
                ColliderShape::Sphere { radius: *radius }
            }
            // TODO: For ConvexHull and SimplifiedMesh, we'll approximate with Box for now
            GeneratedShape::ConvexHull { vertices, .. } => {
                let min = vertices.iter().fold(Vec3::splat(f32::MAX), |acc, v| acc.min(*v));
                let max = vertices.iter().fold(Vec3::splat(f32::MIN), |acc, v| acc.max(*v));
                let center = (min + max) / 2.0;
                let half_extents = (max - center).abs();
                ColliderShape::Box { half_extents }
            }
            GeneratedShape::SimplifiedMesh { vertices, .. } => {
                let min = vertices.iter().fold(Vec3::splat(f32::MAX), |acc, v| acc.min(*v));
                let max = vertices.iter().fold(Vec3::splat(f32::MIN), |acc, v| acc.max(*v));
                let center = (min + max) / 2.0;
                let half_extents = (max - center).abs();
                ColliderShape::Box { half_extents }
            }
        }
    }
}

// ==================== PHASE 2: MESH EXTRACTION ====================

/// Extract vertices from a Bevy mesh
fn extract_vertices(mesh: &Mesh) -> Option<Vec<Vec3>> {
    let positions = mesh.attribute(Mesh::ATTRIBUTE_POSITION)?;
    
    match positions {
        VertexAttributeValues::Float32x3(pos) => {
            Some(pos.iter().map(|p| Vec3::new(p[0], p[1], p[2])).collect())
        }
        _ => {
            warn!("Unsupported vertex format for mesh collision generation");
            None
        }
    }
}

// ==================== PHASE 3: LOW DETAIL GENERATION ====================

/// Generate low detail collision shape (Bounding Box or Sphere)
fn generate_low_detail(vertices: &[Vec3], config: &AutoCollider) -> GeneratedShape {
    if vertices.is_empty() {
        return GeneratedShape::BoundingBox {
            min: Vec3::ZERO,
            max: Vec3::ONE,
        };
    }
    
    match config.preferred_shape.unwrap_or(PreferredShape::Auto) {
        PreferredShape::Sphere => {
            // Calculate Bounding Sphere
            let center = vertices.iter().copied().sum::<Vec3>() / vertices.len() as f32;
            let radius = vertices.iter()
                .map(|v| v.distance(center))
                .fold(0.0f32, |a, b| a.max(b));
            
            GeneratedShape::BoundingSphere { 
                center, 
                radius: radius + config.padding 
            }
        }
        _ => {
            // Calculate Axis-Aligned Bounding Box (AABB)
            let min = vertices.iter().fold(Vec3::splat(f32::MAX), |acc, v| acc.min(*v));
            let max = vertices.iter().fold(Vec3::splat(f32::MIN), |acc, v| acc.max(*v));
            
            // Apply padding
            let padding_vec = Vec3::splat(config.padding);
            
            GeneratedShape::BoundingBox { 
                min: min - padding_vec,
                max: max + padding_vec,
            }
        }
    }
}

// ==================== PHASE 2.5: MEDIUM DETAIL GENERATION ====================

/// Simplify vertices to a target count using 3D grid clustering
/// This reduces vertex count while preserving overall shape
fn simplify_vertices(vertices: &[Vec3], target_count: usize) -> Vec<Vec3> {
    if vertices.len() <= target_count {
        return vertices.to_vec();
    }
    
    // Calculate bounding box
    let min = vertices.iter().fold(Vec3::splat(f32::MAX), |acc, v| acc.min(*v));
    let max = vertices.iter().fold(Vec3::splat(f32::MIN), |acc, v| acc.max(*v));
    let size = max - min;
    
    // Avoid division by zero
    if size.x < 0.001 || size.y < 0.001 || size.z < 0.001 {
        // Degenerate mesh (flat or line), just sample evenly
        let step = vertices.len() / target_count.max(1);
        return vertices.iter().step_by(step).copied().collect();
    }
    
    // Determine grid dimensions (aim for roughly cubic cells)
    // We want roughly target_count cells, distributed as a cube
    let cells_per_axis = (target_count as f32).powf(1.0 / 3.0).ceil() as usize;
    let cells_per_axis = cells_per_axis.max(2); // At least 2x2x2 grid
    
    // Create 3D grid of cells
    use std::collections::HashMap;
    let mut grid: HashMap<(usize, usize, usize), Vec<Vec3>> = HashMap::new();
    
    // Assign vertices to cells
    for vertex in vertices {
        let normalized = (*vertex - min) / size; // 0..1 range
        let ix = ((normalized.x * cells_per_axis as f32).floor() as usize).min(cells_per_axis - 1);
        let iy = ((normalized.y * cells_per_axis as f32).floor() as usize).min(cells_per_axis - 1);
        let iz = ((normalized.z * cells_per_axis as f32).floor() as usize).min(cells_per_axis - 1);
        
        grid.entry((ix, iy, iz)).or_insert_with(Vec::new).push(*vertex);
    }
    
    // Average vertices per cell
    let mut simplified: Vec<Vec3> = grid.values()
        .map(|cell_vertices| {
            let sum: Vec3 = cell_vertices.iter().copied().sum();
            sum / cell_vertices.len() as f32
        })
        .collect();
    
    // If we got too few vertices, add extreme points to ensure good coverage
    if simplified.len() < 6 {
        simplified.push(min);
        simplified.push(max);
        simplified.push(Vec3::new(min.x, min.y, max.z));
        simplified.push(Vec3::new(min.x, max.y, min.z));
        simplified.push(Vec3::new(max.x, min.y, min.z));
        simplified.push(Vec3::new(max.x, max.y, min.z));
    }
    
    simplified
}

/// Compute a simplified convex hull (Quickhull algorithm - simplified version)
/// Returns vertices and triangle faces
fn compute_convex_hull(vertices: &[Vec3]) -> (Vec<Vec3>, Vec<[usize; 3]>) {
    if vertices.len() < 4 {
        // Not enough vertices for a hull, return as-is
        return (vertices.to_vec(), Vec::new());
    }
    
    // Step 1: Find extreme points (6 points: min/max in each axis)
    let mut min_x = vertices[0];
    let mut max_x = vertices[0];
    let mut min_y = vertices[0];
    let mut max_y = vertices[0];
    let mut min_z = vertices[0];
    let mut max_z = vertices[0];
    
    for v in vertices {
        if v.x < min_x.x { min_x = *v; }
        if v.x > max_x.x { max_x = *v; }
        if v.y < min_y.y { min_y = *v; }
        if v.y > max_y.y { max_y = *v; }
        if v.z < min_z.z { min_z = *v; }
        if v.z > max_z.z { max_z = *v; }
    }
    
    // Collect unique extreme points
    let mut hull_vertices = Vec::new();
    let extreme_points = [min_x, max_x, min_y, max_y, min_z, max_z];
    
    for point in extreme_points {
        if !hull_vertices.iter().any(|v: &Vec3| v.distance(point) < 0.001) {
            hull_vertices.push(point);
        }
    }
    
    // Step 2: Add more points from original set to improve hull quality
    // Select points that are furthest from the center
    let center: Vec3 = vertices.iter().copied().sum::<Vec3>() / vertices.len() as f32;
    
    let mut sorted_vertices: Vec<(f32, Vec3)> = vertices.iter()
        .map(|v| (v.distance(center), *v))
        .collect();
    sorted_vertices.sort_by(|a, b| b.0.partial_cmp(&a.0).unwrap());
    
    // Add up to 10 more vertices (furthest from center)
    for (_, vertex) in sorted_vertices.iter().take(16) {
        if !hull_vertices.iter().any(|v: &Vec3| v.distance(*vertex) < 0.001) {
            hull_vertices.push(*vertex);
        }
    }
    
    // Step 3: Generate faces (simplified - create triangles to nearest neighbors)
    // For a proper convex hull, we'd use Quickhull or Gift Wrapping
    // Here we just create a simple triangulation
    let mut faces = Vec::new();
    
    if hull_vertices.len() >= 3 {
        // Simple fan triangulation from first vertex
        for i in 1..hull_vertices.len()-1 {
            faces.push([0, i, i + 1]);
        }
    }
    
    (hull_vertices, faces)
}

/// Generate medium detail collision shape (Simplified Convex Hull)
fn generate_medium_detail(vertices: &[Vec3], config: &AutoCollider) -> GeneratedShape {
    if vertices.is_empty() {
        return GeneratedShape::BoundingBox {
            min: Vec3::ZERO,
            max: Vec3::ONE,
        };
    }
    
    // Step 1: Simplify vertex count to ~16-32 vertices
    let simplified = simplify_vertices(vertices, 24);
    
    // Step 2: Compute convex hull
    let (hull_vertices, hull_faces) = compute_convex_hull(&simplified);
    
    // Apply padding if needed
    let final_vertices = if config.padding != 0.0 {
        let center: Vec3 = hull_vertices.iter().copied().sum::<Vec3>() / hull_vertices.len() as f32;
        hull_vertices.iter()
            .map(|v| {
                let dir = (*v - center).normalize_or_zero();
                *v + dir * config.padding
            })
            .collect()
    } else {
        hull_vertices
    };
    
    GeneratedShape::ConvexHull {
        vertices: final_vertices,
        faces: hull_faces,
    }
}

// ==================== PHASE 3: HIGH DETAIL WITH PARRY3D ====================

/// Generate high detail collision shape using parry3d's Quickhull algorithm
/// This provides professional-grade convex hull computation
fn generate_high_detail(vertices: &[Vec3], config: &AutoCollider) -> GeneratedShape {
    if vertices.is_empty() {
        return GeneratedShape::BoundingBox {
            min: Vec3::ZERO,
            max: Vec3::ONE,
        };
    }
    
    // Optional: Simplify if mesh is extremely dense (>500 vertices)
    // This keeps performance reasonable while maintaining high quality
    let working_vertices = if vertices.len() > 500 {
        simplify_vertices(vertices, 200)
    } else {
        vertices.to_vec()
    };
    
    // Convert Bevy Vec3 to parry3d Point3
    use parry3d::math::Point;
    let points: Vec<Point<f32>> = working_vertices.iter()
        .map(|v| Point::new(v.x, v.y, v.z))
        .collect();
    
    // Compute convex hull using parry3d's Quickhull algorithm
    use parry3d::transformation::convex_hull;
    
    // parry3d returns (vertices, indices) directly
    let (hull_vertices, hull_indices) = convex_hull(&points);
    
    // Convert back to Bevy Vec3
    let mut bevy_vertices: Vec<Vec3> = hull_vertices.iter()
        .map(|p| Vec3::new(p.x, p.y, p.z))
        .collect();
    
    // Apply padding if needed
    if config.padding != 0.0 {
        let center: Vec3 = bevy_vertices.iter().copied().sum::<Vec3>() / bevy_vertices.len() as f32;
        bevy_vertices = bevy_vertices.iter()
            .map(|v| {
                let dir = (*v - center).normalize_or_zero();
                *v + dir * config.padding
            })
            .collect();
    }
    
    // Convert indices (parry3d uses [[u32; 3]] for triangle faces)
    let faces: Vec<[usize; 3]> = hull_indices.iter()
        .map(|face| [face[0] as usize, face[1] as usize, face[2] as usize])
        .collect();
    
    GeneratedShape::ConvexHull {
        vertices: bevy_vertices,
        faces,
    }
}

// ==================== PHASE 5: LOD SYSTEM ====================

/// Update collision detail level based on distance to camera
/// Only affects entities with CollisionLOD component
fn update_collision_lod(
    camera_query: Query<&GlobalTransform, With<crate::camera::OrbitCamera>>,
    mut commands: Commands,
    mut lod_query: Query<(
        Entity,
        &GlobalTransform,
        &mut AutoCollider,
        &mut CollisionLOD,
        Option<&GeneratedCollider>,
    )>,
) {
    // Get camera position
    let Ok(camera_transform) = camera_query.get_single() else {
        return;
    };
    let camera_pos = camera_transform.translation();
    
    for (entity, transform, mut auto_collider, mut lod, generated) in lod_query.iter_mut() {
        // Skip if LOD is disabled
        if !lod.auto_switch {
            continue;
        }
        
        // Calculate distance to camera
        let object_pos = transform.translation();
        let distance = camera_pos.distance(object_pos);
        
        // Determine appropriate detail level with hysteresis
        let new_detail = lod.detail_for_distance(distance);
        
        // Check if detail level changed
        if auto_collider.detail != new_detail {
            // Update detail level
            auto_collider.detail = new_detail;
            
            // Trigger regeneration by removing GeneratedCollider
            // This will cause generate_colliders() to run again
            if generated.is_some() {
                commands.entity(entity).remove::<GeneratedCollider>();
                commands.entity(entity).remove::<Collider>();
                commands.entity(entity).remove::<CollisionCache>();
                
                // Log detail switch
                info!(
                    "LOD switched to {:?} at distance {:.1}m for entity {:?}",
                    new_detail, distance, entity
                );
            }
        }
    }
}

// ==================== PHASE 4: CACHE SYSTEMS ====================

/// Create CollisionCache for new colliders
/// Runs for entities with Collider but no CollisionCache
fn create_collision_caches(
    mut commands: Commands,
    query: Query<(Entity, &Collider, &GlobalTransform), Without<CollisionCache>>,
) {
    for (entity, collider, transform) in query.iter() {
        let position = transform.translation();
        let rotation = transform.to_scale_rotation_translation().1;
        
        let cache = CollisionCache::from_shape(&collider.shape, position, rotation);
        
        commands.entity(entity).insert(cache);
    }
}

/// Update CollisionCache when Transform changes
/// Only updates caches for entities whose transform has changed
fn update_collision_caches(
    mut query: Query<(&Collider, &GlobalTransform, &mut CollisionCache), Changed<GlobalTransform>>,
) {
    for (collider, transform, mut cache) in query.iter_mut() {
        let position = transform.translation();
        let rotation = transform.to_scale_rotation_translation().1;
        
        // Only update if transform actually changed (avoid false positives)
        if cache.needs_update(position, rotation) {
            cache.update(&collider.shape, position, rotation);
        }
    }
}

// ==================== PHASE 4: COLLIDER GENERATION SYSTEM ====================

/// System that analyzes meshes and generates colliders
/// Runs once for each entity with AutoCollider but no GeneratedCollider
fn generate_colliders(
    mut commands: Commands,
    query: Query<(Entity, &AutoCollider, &Handle<Mesh>), Without<GeneratedCollider>>,
    meshes: Res<Assets<Mesh>>,
) {
    for (entity, auto_collider, mesh_handle) in query.iter() {
        // Get mesh data
        let Some(mesh) = meshes.get(mesh_handle) else {
            warn!("⚠️  Mesh not loaded yet for entity {:?} - will retry next frame", entity);
            continue;
        };
        
        // Extract vertices
        let Some(vertices) = extract_vertices(mesh) else {
            warn!("Failed to extract vertices from mesh for entity {:?}", entity);
            continue;
        };
        
        if vertices.is_empty() {
            warn!("Mesh has no vertices for entity {:?}", entity);
            continue;
        }
        
        // Generate shape based on detail level
        let generated_shape = match auto_collider.detail {
            CollisionDetail::Low => generate_low_detail(&vertices, auto_collider),
            CollisionDetail::Medium => generate_medium_detail(&vertices, auto_collider),
            CollisionDetail::High => generate_high_detail(&vertices, auto_collider),
        };
        
        // Convert to Collider shape
        let collider_shape = generated_shape.to_collider();
        
        // Add both GeneratedCollider (for tracking) and Collider (for collision detection)
        commands.entity(entity).insert((
            GeneratedCollider {
                shape: generated_shape.clone(),
                source_mesh: mesh_handle.clone(),
                detail_used: auto_collider.detail,
            },
            Collider {
                shape: collider_shape,
                collision_type: auto_collider.collision_type,
                layer: auto_collider.layer,
            },
            CollidingWith::default(),
        ));
        
        info!(
            "✅ Generated {:?} collider for entity {:?} | Shape: {:?} | Layer: {:?} | Type: {:?} | Vertices: {}",
            auto_collider.detail, entity,
            match &collider_shape {
                ColliderShape::Box { .. } => "Box",
                ColliderShape::Cylinder { .. } => "Cylinder",
                ColliderShape::Sphere { .. } => "Sphere",
            },
            auto_collider.layer,
            auto_collider.collision_type,
            match &generated_shape {
                GeneratedShape::ConvexHull { vertices, .. } => vertices.len(),
                GeneratedShape::SimplifiedMesh { vertices, .. } => vertices.len(),
                GeneratedShape::BoundingBox { min, max } => {
                    info!("   📦 BoundingBox: min={:?}, max={:?}", min, max);
                    0
                },
                GeneratedShape::BoundingSphere { center, radius } => {
                    info!("   ⚪ BoundingSphere: center={:?}, radius={}", center, radius);
                    0
                },
            }
        );
    }
}

// ==================== PHASE 3: SPATIAL PARTITIONING ====================

/// Spatial Grid for efficient collision detection
/// Divides the world into a grid of cells, each tracking entities within it
#[derive(Resource)]
pub struct SpatialGrid {
    /// Grid cell size (in world units)
    cell_size: f32,
    /// Map of cell coordinates to entities in that cell
    cells: HashMap<(i32, i32), Vec<Entity>>,
}

impl Default for SpatialGrid {
    fn default() -> Self {
        Self {
            cell_size: 10.0,  // 10x10 meter cells
            cells: HashMap::new(),
        }
    }
}

impl SpatialGrid {
    /// Convert world position to grid cell coordinates
    fn world_to_cell(&self, pos: Vec3) -> (i32, i32) {
        let x = (pos.x / self.cell_size).floor() as i32;
        let z = (pos.z / self.cell_size).floor() as i32;
        (x, z)
    }
    
    /// Get all cells that a collider might occupy (including neighbors)
    fn get_relevant_cells(&self, pos: Vec3, radius: f32) -> Vec<(i32, i32)> {
        let center_cell = self.world_to_cell(pos);
        let cell_radius = (radius / self.cell_size).ceil() as i32;
        
        let mut cells = Vec::new();
        for dx in -cell_radius..=cell_radius {
            for dz in -cell_radius..=cell_radius {
                cells.push((center_cell.0 + dx, center_cell.1 + dz));
            }
        }
        cells
    }
    
    /// Get entities in a cell
    fn get_entities_in_cell(&self, cell: (i32, i32)) -> &[Entity] {
        self.cells.get(&cell).map(|v| v.as_slice()).unwrap_or(&[])
    }
    
    /// Clear all cells
    fn clear(&mut self) {
        self.cells.clear();
    }
    
    /// Insert entity into appropriate cells
    fn insert(&mut self, entity: Entity, pos: Vec3, radius: f32) {
        let cells = self.get_relevant_cells(pos, radius);
        for cell in cells {
            self.cells.entry(cell).or_insert_with(Vec::new).push(entity);
        }
    }
    
    // Phase 4: Multi-Threading support
    
    /// Get all non-empty cells (for parallel processing)
    fn get_all_cells(&self) -> Vec<(i32, i32)> {
        self.cells.keys().copied().collect()
    }
    
    /// Get entities in multiple cells (thread-safe read)
    fn get_entities_in_cells(&self, cells: &[(i32, i32)]) -> Vec<Entity> {
        let mut entities = Vec::new();
        for cell in cells {
            if let Some(cell_entities) = self.cells.get(cell) {
                entities.extend_from_slice(cell_entities);
            }
        }
        entities
    }
}

/// Update the spatial grid with current entity positions
fn update_spatial_grid(
    mut grid: ResMut<SpatialGrid>,
    collider_query: Query<(Entity, &GlobalTransform, &Collider)>,
) {
    // Clear previous frame's data
    grid.clear();
    
    // Insert all colliders into grid
    for (entity, transform, collider) in collider_query.iter() {
        let pos = transform.translation();
        
        // Calculate approximate radius for grid insertion
        let radius = match collider.shape {
            ColliderShape::Cylinder { radius, .. } => radius,
            ColliderShape::Sphere { radius } => radius,
            ColliderShape::Box { half_extents } => half_extents.max_element(),
        };
        
        grid.insert(entity, pos, radius);
    }
}

// ==================== PHASE 4: MULTI-THREADING ====================

/// Helper struct to store collision data found by threads
#[derive(Debug, Clone)]
struct CollisionData {
    entity_a: Entity,
    entity_b: Entity,
    contact_point: Vec3,
    penetration_depth: f32,
    was_new_collision: bool,
    is_trigger_a: bool,
    is_trigger_b: bool,
}

/// Main collision detection system (Phase 4: Multi-Threaded with Cache)
/// Uses CollisionCache for fast broad-phase checks
/// Uses Rayon to parallelize collision checks across CPU cores
fn detect_collisions(
    grid: Res<SpatialGrid>,
    collider_query: Query<(Entity, &GlobalTransform, &Collider, &CollidingWith, Option<&CollisionCache>)>,
    mut collision_started: EventWriter<CollisionStarted>,
    mut trigger_entered: EventWriter<TriggerEntered>,
) {
    use std::collections::HashSet;
    use std::sync::Mutex;
    
    // Collect entity data into a thread-safe structure (now with cache)
    let entity_data: Vec<_> = collider_query
        .iter()
        .map(|(entity, transform, collider, colliding_with, cache)| {
            let (_, rotation, translation) = transform.to_scale_rotation_translation();
            (
                entity,
                translation,
                rotation,
                collider.shape,
                collider.collision_type,
                collider.layer,
                colliding_with.entities.clone(),
                cache.cloned(),  // Clone cache for thread-safety
            )
        })
        .collect();
    
    // Thread-safe storage for found collisions
    let found_collisions = Arc::new(Mutex::new(Vec::new()));
    let checked_pairs = Arc::new(Mutex::new(HashSet::new()));
    
    // Process entities in parallel using Rayon
    entity_data.par_iter().for_each(|(entity_a, pos_a, rot_a, shape_a, type_a, layer_a, colliding_with_a, cache_a)| {
        let radius_a = match shape_a {
            ColliderShape::Cylinder { radius, .. } => *radius,
            ColliderShape::Sphere { radius } => *radius,
            ColliderShape::Box { half_extents } => half_extents.max_element(),
        };
        
        // Get relevant cells from spatial grid
        let cells = grid.get_relevant_cells(*pos_a, radius_a * 2.0);
        
        // Check entities in nearby cells
        for cell in cells {
            for &entity_b in grid.get_entities_in_cell(cell) {
                // Skip self-collision
                if *entity_a == entity_b {
                    continue;
                }
                
                // Skip if we already checked this pair
                let pair = if *entity_a < entity_b {
                    (*entity_a, entity_b)
                } else {
                    (entity_b, *entity_a)
                };
                
                // Thread-safe check and insert
                {
                    let mut checked = checked_pairs.lock().unwrap();
                    if checked.contains(&pair) {
                        continue;
                    }
                    checked.insert(pair);
                }
                
                // Find entity_b's data
                if let Some((_, pos_b, rot_b, shape_b, type_b, layer_b, colliding_with_b, cache_b)) = 
                    entity_data.iter().find(|(e, ..)| *e == entity_b) {
                    
                    // Check collision layers
                    if !layer_a.can_collide_with(layer_b) {
                        continue;
                    }
                    
                    // Phase 4: Fast broad-phase check using cache (if available)
                    if let (Some(cache_a), Some(cache_b)) = (cache_a, cache_b) {
                        // First: Fast AABB overlap test
                        if !cache_a.aabb_overlaps(cache_b) {
                            continue;  // No AABB overlap = no collision
                        }
                        
                        // Second: Fast sphere overlap test
                        if !cache_a.sphere_overlaps(*pos_a, *rot_a, cache_b, *pos_b, *rot_b) {
                            continue;  // No sphere overlap = no collision
                        }
                        
                        // Both passed: proceed to detailed collision check
                    }
                    
                    // Narrow-phase: Detailed collision check
                    if let Some(collision_info) = check_collision(
                        *pos_a, *rot_a, shape_a,
                        *pos_b, *rot_b, shape_b,
                    ) {
                        // Debug: Log first few collisions
                        static COLLISION_COUNT: std::sync::atomic::AtomicUsize = std::sync::atomic::AtomicUsize::new(0);
                        let count = COLLISION_COUNT.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
                        if count < 5 {
                            info!("🔴 COLLISION DETECTED #{}: {:?} <-> {:?} | Penetration: {:.3} | Normal: {:?}", 
                                count + 1, entity_a, entity_b, collision_info.penetration_depth, collision_info.normal);
                        }
                        // Check if this is a new collision
                        let was_colliding_a = colliding_with_a.contains(&entity_b);
                        let was_colliding_b = colliding_with_b.contains(entity_a);
                        
                        if !was_colliding_a && !was_colliding_b {
                            // Store collision data
                            let collision = CollisionData {
                                entity_a: *entity_a,
                                entity_b,
                                contact_point: collision_info.contact_point,
                                penetration_depth: collision_info.penetration_depth,
                                was_new_collision: true,
                                is_trigger_a: *type_a == CollisionType::Trigger,
                                is_trigger_b: *type_b == CollisionType::Trigger,
                            };
                            
                            found_collisions.lock().unwrap().push(collision);
                        }
                    }
                }
            }
        }
    });
    
    // Process found collisions (single-threaded, Bevy events are not thread-safe)
    let collisions = Arc::try_unwrap(found_collisions).unwrap().into_inner().unwrap();
    for collision in collisions {
        // Handle triggers
        if collision.is_trigger_a {
            trigger_entered.send(TriggerEntered {
                trigger: collision.entity_a,
                entered_by: collision.entity_b,
            });
        }
        if collision.is_trigger_b {
            trigger_entered.send(TriggerEntered {
                trigger: collision.entity_b,
                entered_by: collision.entity_a,
            });
        }
        
        // Send collision event for non-triggers
        if !collision.is_trigger_a && !collision.is_trigger_b {
            collision_started.send(CollisionStarted {
                entity_a: collision.entity_a,
                entity_b: collision.entity_b,
                contact_point: collision.contact_point,
                penetration_depth: collision.penetration_depth,
            });
            
            info!(
                "Collision started: {:?} <-> {:?} (penetration: {:.2})",
                collision.entity_a, collision.entity_b, collision.penetration_depth
            );
        }
    }
}

/// Update the CollidingWith component based on current state (Phase 3: Optimized)
fn update_colliding_with(
    grid: Res<SpatialGrid>,
    mut collider_query: Query<(Entity, &GlobalTransform, &Collider, &mut CollidingWith)>,
    mut collision_ended: EventWriter<CollisionEnded>,
) {
    use std::collections::HashSet;
    
    // Track current collisions this frame
    let mut current_collisions: Vec<(Entity, Entity)> = Vec::new();
    let mut checked_pairs: HashSet<(Entity, Entity)> = HashSet::new();
    
    // Use spatial grid to find collisions efficiently
    for (entity_a, transform_a, collider_a, _) in collider_query.iter() {
        let (_, rot_a, pos_a) = transform_a.to_scale_rotation_translation();
        
        let radius_a = match collider_a.shape {
            ColliderShape::Cylinder { radius, .. } => radius,
            ColliderShape::Sphere { radius } => radius,
            ColliderShape::Box { half_extents } => half_extents.max_element(),
        };
        
        let cells = grid.get_relevant_cells(pos_a, radius_a * 2.0);
        
        for cell in cells {
            for &entity_b in grid.get_entities_in_cell(cell) {
                if entity_a == entity_b {
                    continue;
                }
                
                let pair = if entity_a < entity_b {
                    (entity_a, entity_b)
                } else {
                    (entity_b, entity_a)
                };
                
                if checked_pairs.contains(&pair) {
                    continue;
                }
                checked_pairs.insert(pair);
                
                let Ok((_, transform_b, collider_b, _)) = collider_query.get(entity_b) else {
                    continue;
                };
                
                // Check collision layers
                if !collider_a.layer.can_collide_with(&collider_b.layer) {
                    continue;
                }
                
                let (_, rot_b, pos_b) = transform_b.to_scale_rotation_translation();
                
                if check_collision(pos_a, rot_a, &collider_a.shape, pos_b, rot_b, &collider_b.shape).is_some() {
                    current_collisions.push((entity_a, entity_b));
                }
            }
        }
    }
    
    // Update CollidingWith and detect ended collisions
    for (entity, _, _, mut colliding_with) in collider_query.iter_mut() {
        let mut new_colliding: Vec<Entity> = Vec::new();
        
        // Find all entities this one is colliding with
        for (entity_a, entity_b) in &current_collisions {
            if *entity_a == entity {
                new_colliding.push(*entity_b);
            } else if *entity_b == entity {
                new_colliding.push(*entity_a);
            }
        }
        
        // Check for ended collisions
        for old_entity in &colliding_with.entities {
            if !new_colliding.contains(old_entity) {
                collision_ended.send(CollisionEnded {
                    entity_a: entity,
                    entity_b: *old_entity,
                });
                info!("Collision ended: {:?} <-> {:?}", entity, old_entity);
            }
        }
        
        // Update the list
        colliding_with.entities = new_colliding;
    }
}

/// Check collision between two shapes
/// Now supports rotation for accurate Oriented Bounding Box (OBB) collision
pub fn check_collision(
    pos_a: Vec3,
    rot_a: Quat,
    shape_a: &ColliderShape,
    pos_b: Vec3,
    rot_b: Quat,
    shape_b: &ColliderShape,
) -> Option<CollisionInfo> {
    match (shape_a, shape_b) {
        (
            ColliderShape::Cylinder { radius: r_a, height: h_a },
            ColliderShape::Cylinder { radius: r_b, height: h_b },
        ) => check_cylinder_cylinder(pos_a, *r_a, *h_a, pos_b, *r_b, *h_b),
        
        (ColliderShape::Sphere { radius: r_a }, ColliderShape::Sphere { radius: r_b }) => {
            check_sphere_sphere(pos_a, *r_a, pos_b, *r_b)
        }
        
        (ColliderShape::Cylinder { radius: r, height: h }, ColliderShape::Sphere { radius: r_s })
        | (ColliderShape::Sphere { radius: r_s }, ColliderShape::Cylinder { radius: r, height: h }) => {
            check_cylinder_sphere(pos_a, *r, *h, pos_b, *r_s)
        }
        
        // Box vs Box - OBB collision with rotation support
        (ColliderShape::Box { half_extents: he_a }, ColliderShape::Box { half_extents: he_b }) => {
            check_box_box(pos_a, rot_a, *he_a, pos_b, rot_b, *he_b)
        }
        
        // Box vs Cylinder
        (ColliderShape::Box { half_extents: he }, ColliderShape::Cylinder { radius, height }) => {
            check_box_cylinder(pos_a, rot_a, *he, pos_b, *radius, *height)
        }
        (ColliderShape::Cylinder { radius, height }, ColliderShape::Box { half_extents: he }) => {
            check_box_cylinder(pos_b, rot_b, *he, pos_a, *radius, *height).map(|info| {
                // Reverse normal for swapped order
                CollisionInfo {
                    normal: -info.normal,
                    ..info
                }
            })
        }
        
        // Box vs Sphere
        (ColliderShape::Box { half_extents: he }, ColliderShape::Sphere { radius }) => {
            check_box_sphere(pos_a, rot_a, *he, pos_b, *radius)
        }
        (ColliderShape::Sphere { radius }, ColliderShape::Box { half_extents: he }) => {
            check_box_sphere(pos_b, rot_b, *he, pos_a, *radius).map(|info| {
                // Reverse normal for swapped order
                CollisionInfo {
                    normal: -info.normal,
                    ..info
                }
            })
        }
    }
}

// ==================== ORIENTED BOUNDING BOX (OBB) COLLISION ====================

/// Box-Box collision detection using Separating Axis Theorem (SAT)
/// Supports full rotation via Oriented Bounding Boxes (OBB)
fn check_box_box(
    pos_a: Vec3,
    rot_a: Quat,
    half_extents_a: Vec3,
    pos_b: Vec3,
    rot_b: Quat,
    half_extents_b: Vec3,
) -> Option<CollisionInfo> {
    // Get OBB axes (local X, Y, Z rotated to world space)
    let axes_a = [
        rot_a * Vec3::X,
        rot_a * Vec3::Y,
        rot_a * Vec3::Z,
    ];
    let axes_b = [
        rot_b * Vec3::X,
        rot_b * Vec3::Y,
        rot_b * Vec3::Z,
    ];
    
    // Vector from A to B
    let t = pos_b - pos_a;
    
    let mut min_overlap = f32::MAX;
    let mut min_axis = Vec3::ZERO;
    
    // Test axes from box A (3 axes)
    for (i, axis) in axes_a.iter().enumerate() {
        let overlap = test_axis_obb(
            *axis, t,
            &axes_a, half_extents_a,
            &axes_b, half_extents_b,
        );
        
        if overlap <= 0.0 {
            return None; // Separating axis found - no collision
        }
        
        if overlap < min_overlap {
            min_overlap = overlap;
            min_axis = *axis;
        }
    }
    
    // Test axes from box B (3 axes)
    for (i, axis) in axes_b.iter().enumerate() {
        let overlap = test_axis_obb(
            *axis, t,
            &axes_a, half_extents_a,
            &axes_b, half_extents_b,
        );
        
        if overlap <= 0.0 {
            return None;
        }
        
        if overlap < min_overlap {
            min_overlap = overlap;
            min_axis = *axis;
        }
    }
    
    // Test cross product axes (9 axes: 3x3 combinations)
    for axis_a in &axes_a {
        for axis_b in &axes_b {
            let axis = axis_a.cross(*axis_b);
            let axis_len = axis.length();
            
            // Skip near-parallel axes
            if axis_len < 0.001 {
                continue;
            }
            
            let axis_normalized = axis / axis_len;
            let overlap = test_axis_obb(
                axis_normalized, t,
                &axes_a, half_extents_a,
                &axes_b, half_extents_b,
            );
            
            if overlap <= 0.0 {
                return None;
            }
            
            if overlap < min_overlap {
                min_overlap = overlap;
                min_axis = axis_normalized;
            }
        }
    }
    
    // Ensure normal points from A to B
    if min_axis.dot(t) < 0.0 {
        min_axis = -min_axis;
    }
    
    Some(CollisionInfo {
        penetration_depth: min_overlap,
        normal: min_axis,
        contact_point: pos_a + min_axis * (min_overlap * 0.5),
    })
}

/// Helper function to test a single axis for OBB separation
fn test_axis_obb(
    axis: Vec3,
    t: Vec3,
    axes_a: &[Vec3; 3],
    half_extents_a: Vec3,
    axes_b: &[Vec3; 3],
    half_extents_b: Vec3,
) -> f32 {
    // Project box A onto axis
    let r_a = half_extents_a.x * axes_a[0].dot(axis).abs()
        + half_extents_a.y * axes_a[1].dot(axis).abs()
        + half_extents_a.z * axes_a[2].dot(axis).abs();
    
    // Project box B onto axis
    let r_b = half_extents_b.x * axes_b[0].dot(axis).abs()
        + half_extents_b.y * axes_b[1].dot(axis).abs()
        + half_extents_b.z * axes_b[2].dot(axis).abs();
    
    // Project separation vector onto axis
    let distance = t.dot(axis).abs();
    
    // Overlap = sum of projections - distance
    r_a + r_b - distance
}

/// Box-Cylinder collision detection
/// Box is oriented, cylinder is always axis-aligned (Y-up)
fn check_box_cylinder(
    box_pos: Vec3,
    box_rot: Quat,
    box_half_extents: Vec3,
    cyl_pos: Vec3,
    cyl_radius: f32,
    cyl_height: f32,
) -> Option<CollisionInfo> {
    // Transform cylinder center to box's local space
    let local_cyl_pos = box_rot.inverse() * (cyl_pos - box_pos);
    
    // Find closest point on box (in local space) to cylinder center
    let clamped_x = local_cyl_pos.x.clamp(-box_half_extents.x, box_half_extents.x);
    let clamped_y = local_cyl_pos.y.clamp(-box_half_extents.y, box_half_extents.y);
    let clamped_z = local_cyl_pos.z.clamp(-box_half_extents.z, box_half_extents.z);
    let closest_point_local = Vec3::new(clamped_x, clamped_y, clamped_z);
    
    // Transform back to world space
    let closest_point_world = box_pos + box_rot * closest_point_local;
    
    // Check if closest point is inside cylinder
    let diff = closest_point_world - cyl_pos;
    let horizontal_dist = (diff.x * diff.x + diff.z * diff.z).sqrt();
    
    // Cylinder is centered at cyl_pos, so min/max are offset by half height
    let half_height = cyl_height / 2.0;
    let cyl_min_y = cyl_pos.y - half_height;
    let cyl_max_y = cyl_pos.y + half_height;
    
    // Check if inside cylinder's vertical range and horizontal radius
    if closest_point_world.y >= cyl_min_y && 
       closest_point_world.y <= cyl_max_y && 
       horizontal_dist <= cyl_radius {
        
        // Calculate penetration
        let radial_penetration = cyl_radius - horizontal_dist;
        let top_penetration = cyl_max_y - closest_point_world.y;
        let bottom_penetration = closest_point_world.y - cyl_min_y;
        
        // Use minimum penetration
        let (penetration, normal) = if radial_penetration < top_penetration && radial_penetration < bottom_penetration {
            // Horizontal collision
            let dir = if horizontal_dist > 0.001 {
                Vec3::new(diff.x, 0.0, diff.z).normalize()
            } else {
                Vec3::X
            };
            (radial_penetration, dir)
        } else if top_penetration < bottom_penetration {
            // Top collision
            (top_penetration, Vec3::Y)
        } else {
            // Bottom collision
            (bottom_penetration, Vec3::NEG_Y)
        };
        
        Some(CollisionInfo {
            penetration_depth: penetration,
            normal,
            contact_point: closest_point_world,
        })
    } else {
        None
    }
}

/// Box-Sphere collision detection
/// Box is oriented, sphere is simple
fn check_box_sphere(
    box_pos: Vec3,
    box_rot: Quat,
    box_half_extents: Vec3,
    sphere_pos: Vec3,
    sphere_radius: f32,
) -> Option<CollisionInfo> {
    // Transform sphere center to box's local space
    let local_sphere_pos = box_rot.inverse() * (sphere_pos - box_pos);
    
    // Find closest point on box (in local space) to sphere center
    let clamped_x = local_sphere_pos.x.clamp(-box_half_extents.x, box_half_extents.x);
    let clamped_y = local_sphere_pos.y.clamp(-box_half_extents.y, box_half_extents.y);
    let clamped_z = local_sphere_pos.z.clamp(-box_half_extents.z, box_half_extents.z);
    let closest_point_local = Vec3::new(clamped_x, clamped_y, clamped_z);
    
    // Distance from sphere center to closest point
    let diff_local = local_sphere_pos - closest_point_local;
    let distance_sq = diff_local.length_squared();
    
    if distance_sq <= sphere_radius * sphere_radius {
        // Collision detected
        let distance = distance_sq.sqrt();
        let penetration = sphere_radius - distance;
        
        // Normal in world space
        let normal = if distance > 0.001 {
            (box_rot * diff_local).normalize()
        } else {
            // Sphere center is inside box - use direction from box center
            (sphere_pos - box_pos).normalize()
        };
        
        // Contact point in world space
        let closest_point_world = box_pos + box_rot * closest_point_local;
        
        Some(CollisionInfo {
            penetration_depth: penetration,
            normal,
            contact_point: closest_point_world,
        })
    } else {
        None
    }
}

/// Cylinder-Cylinder collision detection
/// NOTE: Cylinders are centered at pos, not bottom-based
fn check_cylinder_cylinder(
    pos_a: Vec3,
    radius_a: f32,
    height_a: f32,
    pos_b: Vec3,
    radius_b: f32,
    height_b: f32,
) -> Option<CollisionInfo> {
    // 1. Check Y-axis overlap (height)
    // Cylinders are centered, so offset by half height
    let half_height_a = height_a / 2.0;
    let half_height_b = height_b / 2.0;
    let y_min_a = pos_a.y - half_height_a;
    let y_max_a = pos_a.y + half_height_a;
    let y_min_b = pos_b.y - half_height_b;
    let y_max_b = pos_b.y + half_height_b;
    
    if y_max_a < y_min_b || y_max_b < y_min_a {
        return None; // No vertical overlap
    }
    
    // 2. Check XZ plane (circle collision)
    let diff_xz = Vec2::new(pos_b.x - pos_a.x, pos_b.z - pos_a.z);
    let distance_xz = diff_xz.length();
    let combined_radius = radius_a + radius_b;
    
    if distance_xz < combined_radius {
        // Collision detected!
        let penetration = combined_radius - distance_xz;
        let direction = if distance_xz > 0.001 {
            diff_xz.normalize()
        } else {
            Vec2::new(1.0, 0.0) // Fallback if entities are at exact same position
        };
        
        Some(CollisionInfo {
            penetration_depth: penetration,
            normal: Vec3::new(direction.x, 0.0, direction.y),
            contact_point: Vec3::new(
                pos_a.x + direction.x * radius_a,
                (y_max_a.min(y_max_b) + y_min_a.max(y_min_b)) / 2.0,
                pos_a.z + direction.y * radius_a,
            ),
        })
    } else {
        None
    }
}

/// Sphere-Sphere collision detection
fn check_sphere_sphere(
    pos_a: Vec3,
    radius_a: f32,
    pos_b: Vec3,
    radius_b: f32,
) -> Option<CollisionInfo> {
    let diff = pos_b - pos_a;
    let distance = diff.length();
    let combined_radius = radius_a + radius_b;
    
    if distance < combined_radius {
        let penetration = combined_radius - distance;
        let normal = if distance > 0.001 {
            diff.normalize()
        } else {
            Vec3::Y // Fallback
        };
        
        Some(CollisionInfo {
            penetration_depth: penetration,
            normal,
            contact_point: pos_a + normal * radius_a,
        })
    } else {
        None
    }
}

/// Cylinder-Sphere collision detection (simplified)
/// NOTE: Cylinder is centered at pos_cyl, not bottom-based
fn check_cylinder_sphere(
    pos_cyl: Vec3,
    radius_cyl: f32,
    height_cyl: f32,
    pos_sphere: Vec3,
    radius_sphere: f32,
) -> Option<CollisionInfo> {
    // Cylinder is centered, so offset by half height
    let half_height = height_cyl / 2.0;
    let y_min = pos_cyl.y - half_height;
    let y_max = pos_cyl.y + half_height;
    
    // Clamp sphere Y to cylinder height
    let clamped_y = pos_sphere.y.clamp(y_min, y_max);
    
    // Check 2D distance in XZ plane
    let diff_xz = Vec2::new(pos_sphere.x - pos_cyl.x, pos_sphere.z - pos_cyl.z);
    let distance_xz = diff_xz.length();
    let combined_radius = radius_cyl + radius_sphere;
    
    if distance_xz < combined_radius && pos_sphere.y >= y_min - radius_sphere && pos_sphere.y <= y_max + radius_sphere {
        let penetration = combined_radius - distance_xz;
        let direction = if distance_xz > 0.001 {
            diff_xz.normalize()
        } else {
            Vec2::new(1.0, 0.0)
        };
        
        Some(CollisionInfo {
            penetration_depth: penetration,
            normal: Vec3::new(direction.x, 0.0, direction.y),
            contact_point: Vec3::new(
                pos_cyl.x + direction.x * radius_cyl,
                clamped_y,
                pos_cyl.z + direction.y * radius_cyl,
            ),
        })
    } else {
        None
    }
}

// ==================== PHASE 2: COLLISION RESOLUTION ====================

/// Resolves collisions by pushing entities apart
/// NOTE: Player collision is now handled predictively in player_movement()
/// This system only handles NPCs, monsters, and other dynamic non-player entities
fn resolve_collisions(
    mut query: Query<(
        Entity,
        &mut Transform,
        &Collider,
        &CollidingWith,
        Option<&CollisionPushback>,
    ), Without<crate::player::Player>>,  // Exclude player - handled predictively
) {
    // Collect all collision pairs that need resolution
    let mut resolutions: Vec<CollisionResolution> = Vec::new();
    
    // We need to collect first, then apply, to avoid borrow checker issues
    let entities: Vec<_> = query.iter().collect();
    
    for i in 0..entities.len() {
        let (entity_a, transform_a, collider_a, colliding_with_a, pushback_a) = entities[i];
        let pos_a = transform_a.translation;
        let rot_a = transform_a.rotation;
        
        // Only process dynamic entities
        if collider_a.collision_type != CollisionType::Dynamic {
            continue;
        }
        
        // Check each entity this one is colliding with
        for &entity_b in &colliding_with_a.entities {
            // Find entity_b in our list
            if let Some((_, transform_b, collider_b, _, pushback_b)) = 
                entities.iter().find(|(e, ..)| *e == entity_b) {
                
                let pos_b = transform_b.translation;
                let rot_b = transform_b.rotation;
                
                // Skip triggers - they don't block movement
                if collider_b.collision_type == CollisionType::Trigger {
                    continue;
                }
                
                // Calculate collision info
                if let Some(collision_info) = check_collision(
                    pos_a, rot_a, &collider_a.shape,
                    pos_b, rot_b, &collider_b.shape,
                ) {
                    // Determine resolution based on collision types
                    let resolution_type = match (collider_a.collision_type, collider_b.collision_type) {
                        (CollisionType::Dynamic, CollisionType::Static) => {
                            // Dynamic vs Static: Only move dynamic
                            ResolutionType::DynamicVsStatic {
                                dynamic_entity: entity_a,
                                pushback_strength: pushback_a.map(|p| p.strength).unwrap_or(0.8),
                            }
                        }
                        (CollisionType::Dynamic, CollisionType::Dynamic) => {
                            // Dynamic vs Dynamic: Push both apart
                            ResolutionType::DynamicVsDynamic {
                                entity_a,
                                entity_b,
                                pushback_a: pushback_a.map(|p| p.strength).unwrap_or(0.8),
                                pushback_b: pushback_b.map(|p| p.strength).unwrap_or(0.8),
                            }
                        }
                        _ => continue, // Static vs Static or Static vs Dynamic (handled by reverse case)
                    };
                    
                    resolutions.push(CollisionResolution {
                        resolution_type,
                        normal: collision_info.normal,
                        penetration: collision_info.penetration_depth,
                    });
                }
            }
        }
    }
    
    // Apply all resolutions
    for resolution in resolutions {
        match resolution.resolution_type {
            ResolutionType::DynamicVsStatic { dynamic_entity, pushback_strength } => {
                // Push dynamic entity out of static object
                // Instant separation - no bouncing
                if let Ok((_, mut transform, _, _, _)) = query.get_mut(dynamic_entity) {
                    // Only push if actually penetrating
                    if resolution.penetration > 0.001 {
                        // Full separation - instant push out
                        let separation = resolution.normal * resolution.penetration;
                        transform.translation -= separation;
                    }
                }
            }
            ResolutionType::DynamicVsDynamic { entity_a, entity_b, pushback_a, pushback_b } => {
                // Push both entities apart based on their pushback strengths
                let total_pushback = pushback_a + pushback_b;
                let ratio_a = pushback_a / total_pushback;
                let ratio_b = pushback_b / total_pushback;
                
                // Apply separation to both entities
                if let Ok((_, mut transform_a, _, _, _)) = query.get_mut(entity_a) {
                    let separation_a = resolution.normal * resolution.penetration * ratio_a;
                    transform_a.translation -= separation_a;
                }
                
                if let Ok((_, mut transform_b, _, _, _)) = query.get_mut(entity_b) {
                    let separation_b = resolution.normal * resolution.penetration * ratio_b;
                    transform_b.translation += separation_b;
                }
            }
        }
    }
}

/// Helper struct for collision resolution
struct CollisionResolution {
    resolution_type: ResolutionType,
    normal: Vec3,
    penetration: f32,
}

enum ResolutionType {
    DynamicVsStatic {
        dynamic_entity: Entity,
        pushback_strength: f32,
    },
    DynamicVsDynamic {
        entity_a: Entity,
        entity_b: Entity,
        pushback_a: f32,
        pushback_b: f32,
    },
}

// ==================== PHASE 6: VISUAL DEBUGGING ====================

/// Resource to control collision debug visualization
#[derive(Resource)]
pub struct CollisionDebugConfig {
    pub enabled: bool,
    pub show_aabb: bool,
    pub show_shapes: bool,
    pub show_caches: bool,
}

impl Default for CollisionDebugConfig {
    fn default() -> Self {
        Self {
            enabled: false,        // F1 to toggle
            show_aabb: false,      // Show AABB boxes (F2 to toggle when debug is on)
            show_shapes: true,     // Show collision shapes (F3 to toggle when debug is on)
            show_caches: false,    // Show cache bounding spheres (F4 to toggle when debug is on)
        }
    }
}

/// Toggle collision debug visualization with F1 key
fn toggle_collision_debug(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut config: ResMut<CollisionDebugConfig>,
) {
    if keyboard.just_pressed(KeyCode::F1) {
        config.enabled = !config.enabled;
        if config.enabled {
            info!("🔍 Collision Debug: ON (F1 to toggle off)");
        } else {
            info!("👁️ Collision Debug: OFF (F1 to toggle on)");
        }
    }
    
    // Additional toggles when debug is enabled
    if config.enabled {
        if keyboard.just_pressed(KeyCode::F2) {
            config.show_aabb = !config.show_aabb;
            info!("📦 AABB Display: {}", if config.show_aabb { "ON" } else { "OFF" });
        }
        if keyboard.just_pressed(KeyCode::F3) {
            config.show_shapes = !config.show_shapes;
            info!("🔷 Shapes Display: {}", if config.show_shapes { "ON" } else { "OFF" });
        }
        if keyboard.just_pressed(KeyCode::F4) {
            config.show_caches = !config.show_caches;
            info!("⭕ Cache Display: {}", if config.show_caches { "ON" } else { "OFF" });
        }
    }
}

/// Draw collision shapes for debugging
fn draw_collision_debug(
    config: Res<CollisionDebugConfig>,
    query: Query<(
        &Transform,
        &Collider,
        Option<&AutoCollider>,
        Option<&CollisionCache>,
        Option<&GeneratedCollider>,
    )>,
    mut gizmos: Gizmos,
) {
    if !config.enabled {
        return;
    }
    
    for (transform, collider, auto_collider, cache, generated) in query.iter() {
        let position = transform.translation;
        
        // Get color based on detail level (if AutoCollider)
        let color = if let Some(auto) = auto_collider {
            match auto.detail {
                CollisionDetail::Low => Color::srgb(0.2, 0.8, 0.2),    // Green
                CollisionDetail::Medium => Color::srgb(0.8, 0.8, 0.2), // Yellow
                CollisionDetail::High => Color::srgb(0.8, 0.2, 0.2),   // Red
            }
        } else {
            Color::srgb(0.5, 0.5, 0.8) // Blue for manual colliders
        };
        
        // Draw collision shape
        if config.show_shapes {
            match &collider.shape {
                ColliderShape::Cylinder { radius, height } => {
                    draw_cylinder(&mut gizmos, position, *radius, *height, color);
                }
                ColliderShape::Sphere { radius } => {
                    gizmos.sphere(position, Quat::IDENTITY, *radius, color);
                }
                ColliderShape::Box { half_extents } => {
                    draw_box(&mut gizmos, position, transform.rotation, *half_extents, color);
                }
            }
            
            // Draw generated shape if it exists (convex hulls, etc.)
            // NOTE: We only draw GeneratedShape for complex shapes (ConvexHull, SimplifiedMesh)
            // For simple shapes (Box, Sphere), we already drew the Collider above
            if let Some(gen) = generated {
                match &gen.shape {
                    GeneratedShape::ConvexHull { vertices, .. } => {
                        draw_convex_hull(&mut gizmos, position, transform.rotation, vertices, color);
                    }
                    GeneratedShape::BoundingBox { .. } | GeneratedShape::BoundingSphere { .. } => {
                        // Skip - already drawn as Collider shape above
                    }
                    GeneratedShape::SimplifiedMesh { vertices, .. } => {
                        // Draw simplified mesh vertices as points
                        for vertex in vertices {
                            let world_pos = position + transform.rotation * *vertex;
                            gizmos.sphere(world_pos, Quat::IDENTITY, 0.05, color);
                        }
                    }
                }
            }
        }
        
        // Draw cache bounding sphere (slightly transparent)
        if config.show_caches {
            if let Some(cache) = cache {
                let cache_color = Color::srgba(0.3, 0.6, 1.0, 0.3);
                gizmos.sphere(
                    cache.bounding_sphere.0,
                    Quat::IDENTITY,
                    cache.bounding_sphere.1,
                    cache_color,
                );
                
                // Draw AABB if enabled
                if config.show_aabb {
                    let (min, max) = cache.aabb;
                    draw_aabb(&mut gizmos, min, max, Color::srgba(1.0, 1.0, 0.0, 0.2));
                }
            }
        }
    }
}

/// Draw a cylinder wireframe
fn draw_cylinder(gizmos: &mut Gizmos, center: Vec3, radius: f32, height: f32, color: Color) {
    let segments = 16;
    let half_height = height / 2.0;
    
    // Bottom circle
    let bottom = center + Vec3::new(0.0, -half_height, 0.0);
    for i in 0..segments {
        let angle1 = (i as f32 / segments as f32) * std::f32::consts::TAU;
        let angle2 = ((i + 1) as f32 / segments as f32) * std::f32::consts::TAU;
        
        let p1 = bottom + Vec3::new(angle1.cos() * radius, 0.0, angle1.sin() * radius);
        let p2 = bottom + Vec3::new(angle2.cos() * radius, 0.0, angle2.sin() * radius);
        
        gizmos.line(p1, p2, color);
    }
    
    // Top circle
    let top = center + Vec3::new(0.0, half_height, 0.0);
    for i in 0..segments {
        let angle1 = (i as f32 / segments as f32) * std::f32::consts::TAU;
        let angle2 = ((i + 1) as f32 / segments as f32) * std::f32::consts::TAU;
        
        let p1 = top + Vec3::new(angle1.cos() * radius, 0.0, angle1.sin() * radius);
        let p2 = top + Vec3::new(angle2.cos() * radius, 0.0, angle2.sin() * radius);
        
        gizmos.line(p1, p2, color);
    }
    
    // Vertical lines connecting top and bottom
    for i in 0..4 {
        let angle = (i as f32 / 4.0) * std::f32::consts::TAU;
        let p1 = bottom + Vec3::new(angle.cos() * radius, 0.0, angle.sin() * radius);
        let p2 = top + Vec3::new(angle.cos() * radius, 0.0, angle.sin() * radius);
        gizmos.line(p1, p2, color);
    }
}



/// Draw a box wireframe
fn draw_box(gizmos: &mut Gizmos, center: Vec3, rotation: Quat, half_extents: Vec3, color: Color) {
    // Define corners in local space
    let local_corners = [
        Vec3::new(-half_extents.x, -half_extents.y, -half_extents.z),
        Vec3::new(half_extents.x, -half_extents.y, -half_extents.z),
        Vec3::new(half_extents.x, -half_extents.y, half_extents.z),
        Vec3::new(-half_extents.x, -half_extents.y, half_extents.z),
        Vec3::new(-half_extents.x, half_extents.y, -half_extents.z),
        Vec3::new(half_extents.x, half_extents.y, -half_extents.z),
        Vec3::new(half_extents.x, half_extents.y, half_extents.z),
        Vec3::new(-half_extents.x, half_extents.y, half_extents.z),
    ];
    
    // Transform to world space with rotation
    let corners: Vec<Vec3> = local_corners
        .iter()
        .map(|&local| center + rotation * local)
        .collect();
    
    // Bottom face
    gizmos.line(corners[0], corners[1], color);
    gizmos.line(corners[1], corners[2], color);
    gizmos.line(corners[2], corners[3], color);
    gizmos.line(corners[3], corners[0], color);
    
    // Top face
    gizmos.line(corners[4], corners[5], color);
    gizmos.line(corners[5], corners[6], color);
    gizmos.line(corners[6], corners[7], color);
    gizmos.line(corners[7], corners[4], color);
    
    // Vertical edges
    gizmos.line(corners[0], corners[4], color);
    gizmos.line(corners[1], corners[5], color);
    gizmos.line(corners[2], corners[6], color);
    gizmos.line(corners[3], corners[7], color);
}

/// Draw a convex hull wireframe
fn draw_convex_hull(
    gizmos: &mut Gizmos,
    position: Vec3,
    rotation: Quat,
    vertices: &[Vec3],
    color: Color,
) {
    if vertices.len() < 2 {
        return;
    }
    
    // Transform vertices to world space
    let world_vertices: Vec<Vec3> = vertices
        .iter()
        .map(|v| position + rotation * *v)
        .collect();
    
    // Draw edges between all nearby vertices (simple wireframe)
    // For proper hull edges, we'd need the face data
    for i in 0..world_vertices.len() {
        for j in (i + 1)..world_vertices.len() {
            let distance = world_vertices[i].distance(world_vertices[j]);
            
            // Only draw edges between close vertices (heuristic for hull edges)
            if distance < 3.0 {
                gizmos.line(world_vertices[i], world_vertices[j], color);
            }
        }
    }
    
    // Also draw a point at each vertex for clarity
    for vertex in world_vertices {
        gizmos.sphere(vertex, Quat::IDENTITY, 0.05, color);
    }
}

/// Draw an AABB wireframe
fn draw_aabb(gizmos: &mut Gizmos, min: Vec3, max: Vec3, color: Color) {
    let corners = [
        Vec3::new(min.x, min.y, min.z),
        Vec3::new(max.x, min.y, min.z),
        Vec3::new(max.x, min.y, max.z),
        Vec3::new(min.x, min.y, max.z),
        Vec3::new(min.x, max.y, min.z),
        Vec3::new(max.x, max.y, min.z),
        Vec3::new(max.x, max.y, max.z),
        Vec3::new(min.x, max.y, max.z),
    ];
    
    // Bottom face
    gizmos.line(corners[0], corners[1], color);
    gizmos.line(corners[1], corners[2], color);
    gizmos.line(corners[2], corners[3], color);
    gizmos.line(corners[3], corners[0], color);
    
    // Top face
    gizmos.line(corners[4], corners[5], color);
    gizmos.line(corners[5], corners[6], color);
    gizmos.line(corners[6], corners[7], color);
    gizmos.line(corners[7], corners[4], color);
    
    // Vertical edges
    gizmos.line(corners[0], corners[4], color);
    gizmos.line(corners[1], corners[5], color);
    gizmos.line(corners[2], corners[6], color);
    gizmos.line(corners[3], corners[7], color);
}
