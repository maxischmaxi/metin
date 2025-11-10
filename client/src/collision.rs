use bevy::prelude::*;
use crate::GameState;
use std::collections::HashMap;
use rayon::prelude::*;  // Phase 4: Multi-Threading
use std::sync::Arc;  // Thread-safe data structures

pub struct CollisionPlugin;

impl Plugin for CollisionPlugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<SpatialGrid>()  // Phase 3: Spatial Partitioning
            .add_event::<CollisionStarted>()
            .add_event::<CollisionEnded>()
            .add_event::<TriggerEntered>()
            .add_systems(Update, (
                update_spatial_grid,      // Phase 3: Update grid first
                detect_collisions,
                update_colliding_with,
                resolve_collisions,
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

/// Internal struct for collision information
#[derive(Debug)]
struct CollisionInfo {
    penetration_depth: f32,
    normal: Vec3, // Points from A to B
    contact_point: Vec3,
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

/// Main collision detection system (Phase 4: Multi-Threaded)
/// Uses Rayon to parallelize collision checks across CPU cores
fn detect_collisions(
    grid: Res<SpatialGrid>,
    collider_query: Query<(Entity, &GlobalTransform, &Collider, &CollidingWith)>,
    mut collision_started: EventWriter<CollisionStarted>,
    mut trigger_entered: EventWriter<TriggerEntered>,
) {
    use std::collections::HashSet;
    use std::sync::Mutex;
    
    // Collect entity data into a thread-safe structure
    let entity_data: Vec<_> = collider_query
        .iter()
        .map(|(entity, transform, collider, colliding_with)| {
            (
                entity,
                transform.translation(),
                collider.shape,
                collider.collision_type,
                collider.layer,
                colliding_with.entities.clone(),
            )
        })
        .collect();
    
    // Thread-safe storage for found collisions
    let found_collisions = Arc::new(Mutex::new(Vec::new()));
    let checked_pairs = Arc::new(Mutex::new(HashSet::new()));
    
    // Process entities in parallel using Rayon
    entity_data.par_iter().for_each(|(entity_a, pos_a, shape_a, type_a, layer_a, colliding_with_a)| {
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
                if let Some((_, pos_b, shape_b, type_b, layer_b, colliding_with_b)) = 
                    entity_data.iter().find(|(e, ..)| *e == entity_b) {
                    
                    // Check collision layers
                    if !layer_a.can_collide_with(layer_b) {
                        continue;
                    }
                    
                    // Check if collision occurs
                    if let Some(collision_info) = check_collision(
                        *pos_a, shape_a,
                        *pos_b, shape_b,
                    ) {
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
        let pos_a = transform_a.translation();
        
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
                
                let pos_b = transform_b.translation();
                
                if check_collision(pos_a, &collider_a.shape, pos_b, &collider_b.shape).is_some() {
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
fn check_collision(
    pos_a: Vec3,
    shape_a: &ColliderShape,
    pos_b: Vec3,
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
        
        // Box collisions - simplified for now (treat as sphere)
        (ColliderShape::Box { half_extents: he }, _) => {
            let radius = he.max_element();
            check_sphere_sphere(pos_a, radius, pos_b, 0.5)
        }
        (_, ColliderShape::Box { half_extents: he }) => {
            let radius = he.max_element();
            check_sphere_sphere(pos_a, 0.5, pos_b, radius)
        }
    }
}

/// Cylinder-Cylinder collision detection
fn check_cylinder_cylinder(
    pos_a: Vec3,
    radius_a: f32,
    height_a: f32,
    pos_b: Vec3,
    radius_b: f32,
    height_b: f32,
) -> Option<CollisionInfo> {
    // 1. Check Y-axis overlap (height)
    let y_min_a = pos_a.y;
    let y_max_a = pos_a.y + height_a;
    let y_min_b = pos_b.y;
    let y_max_b = pos_b.y + height_b;
    
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
fn check_cylinder_sphere(
    pos_cyl: Vec3,
    radius_cyl: f32,
    height_cyl: f32,
    pos_sphere: Vec3,
    radius_sphere: f32,
) -> Option<CollisionInfo> {
    // Simplified: Check if sphere center is within cylinder's height
    let y_min = pos_cyl.y;
    let y_max = pos_cyl.y + height_cyl;
    
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
/// This system runs after collision detection and prevents entities from overlapping
fn resolve_collisions(
    mut query: Query<(
        Entity,
        &mut Transform,
        &Collider,
        &CollidingWith,
        Option<&CollisionPushback>,
    )>,
) {
    // Collect all collision pairs that need resolution
    let mut resolutions: Vec<CollisionResolution> = Vec::new();
    
    // We need to collect first, then apply, to avoid borrow checker issues
    let entities: Vec<_> = query.iter().collect();
    
    for i in 0..entities.len() {
        let (entity_a, transform_a, collider_a, colliding_with_a, pushback_a) = entities[i];
        let pos_a = transform_a.translation;
        
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
                
                // Skip triggers - they don't block movement
                if collider_b.collision_type == CollisionType::Trigger {
                    continue;
                }
                
                // Calculate collision info
                if let Some(collision_info) = check_collision(
                    pos_a, &collider_a.shape,
                    pos_b, &collider_b.shape,
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
                if let Ok((_, mut transform, _, _, _)) = query.get_mut(dynamic_entity) {
                    let separation = resolution.normal * resolution.penetration * pushback_strength;
                    transform.translation -= separation;
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
