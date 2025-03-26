use crate::components::movement::*;
use crate::components::unit::Selected;
use bevy::input::mouse::MouseButton;
use bevy::prelude::*;
use bevy::window::Window;
use bevy_ecs_ldtk::prelude::*;
use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap, HashSet};

// Define a constant for tile size - adjust this to match your game
const TILE_SIZE: f32 = 64.0;

pub struct MovementPlugin;

impl Plugin for MovementPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (handle_movement_input, start_unit_movement, move_units).chain(),
        );
    }
}

// Same systems but with updated pathfinding
#[allow(clippy::type_complexity)]
fn handle_movement_input(
    mouse_button: Res<ButtonInput<MouseButton>>,
    windows: Query<&Window>,
    camera_q: Query<(&Camera, &GlobalTransform)>,
    level_query: Query<&Transform, With<LdtkProjectHandle>>,
    selected_units: Query<(Entity, &GridCoords), (With<Selected>, With<Movable>)>,
    mut unit_targets: Query<&mut MoveTarget>,
    colliders: Query<&GridCoords, With<Collider>>,
) {
    // Only process right-clicks
    if !mouse_button.just_pressed(MouseButton::Right) {
        return;
    }

    let window = windows.single();
    let Some(cursor_position) = window.cursor_position() else {
        return;
    };

    // Get camera transform to convert screen to world coordinates
    let (camera, camera_transform) = camera_q.single();
    let Ok(cursor_ray) = camera.viewport_to_world(camera_transform, cursor_position) else {
        return;
    };

    let cursor_pos = cursor_ray.origin.truncate();

    // Get the level transform
    let level_transform = match level_query.get_single() {
        Ok(transform) => transform,
        Err(_) => return,
    };

    // Debug info for world position
    info!("Cursor world position: {:?}", cursor_pos);
    info!(
        "Level position: {:?}",
        level_transform.translation.truncate()
    );

    // Calculate grid position with the OFFSET CORRECTION
    // Based on the debug logs, there's a consistent offset between entity grid coords
    // and the calculated grid coordinates
    let grid_pos = GridCoords {
        x: ((cursor_pos.x - level_transform.translation.x) / TILE_SIZE).floor() as i32 + 30,
        y: ((cursor_pos.y - level_transform.translation.y) / TILE_SIZE).floor() as i32 + 29,
    };

    info!("Calculated grid position: {:?}", grid_pos);

    // Collect all blocked grid coordinates
    let blocked_coords: HashSet<GridCoords> = colliders.iter().cloned().collect();

    // Set target for each selected unit
    for (entity, grid_coords) in selected_units.iter() {
        info!("Selected unit at grid: {:?}", grid_coords);

        if let Ok(mut move_target) = unit_targets.get_mut(entity) {
            // Don't create a path if clicking on the same cell
            if grid_coords.x == grid_pos.x && grid_coords.y == grid_pos.y {
                continue;
            }

            // Use A* to find a path avoiding obstacles
            move_target.destination = Some(grid_pos);
            let path = a_star_pathfinding(grid_coords, &grid_pos, &blocked_coords);
            info!("Path found: {:?}", path);
            move_target.path = path;
        }
    }
}

// Same start_unit_movement function
#[allow(clippy::type_complexity)]
fn start_unit_movement(
    mut commands: Commands,
    level_transform: Query<&GlobalTransform, With<LdtkProjectHandle>>,
    mut movable_units: Query<
        (Entity, &Transform, &GridCoords, &mut MoveTarget, &Movable),
        Without<Moving>,
    >,
    colliders: Query<&GridCoords, With<Collider>>,
) {
    let level_transform = match level_transform.get_single() {
        Ok(transform) => transform,
        Err(_) => return,
    };

    // Collect all blocked grid coordinates
    let blocked_coords: HashSet<GridCoords> = colliders.iter().cloned().collect();

    for (entity, transform, grid_coords, mut move_target, _) in movable_units.iter_mut() {
        // If there's no path or the path is empty but there's a destination,
        // calculate a new path
        if move_target.path.is_empty() && move_target.destination.is_some() {
            let destination = move_target.destination.unwrap();
            move_target.path = a_star_pathfinding(grid_coords, &destination, &blocked_coords);
        }

        // If there's a path, start moving to the next cell
        if !move_target.path.is_empty() {
            let next_pos = move_target.path.remove(0);

            // Calculate world positions
            let current_pos = transform.translation;

            // Add half a tile size to position units at the center of grid cells
            let target_world_x =
                level_transform.translation().x + (next_pos.x as f32 + 0.5) * TILE_SIZE;
            let target_world_y =
                level_transform.translation().y + (next_pos.y as f32 + 0.5) * TILE_SIZE;
            let target_pos = Vec3::new(target_world_x, target_world_y, current_pos.z);

            // Start movement animation
            commands.entity(entity).insert(Moving {
                from: current_pos,
                to: target_pos,
                progress: 0.0,
            });

            // Update grid coordinates (logical position change)
            commands.entity(entity).insert(next_pos);

            // If the path is now empty and we've reached the destination,
            // clear the destination too
            if move_target.path.is_empty()
                && move_target.destination.is_some()
                && next_pos == move_target.destination.unwrap()
            {
                move_target.destination = None;
            }
        }
    }
}

fn move_units(
    mut commands: Commands,
    time: Res<Time>,
    mut moving_units: Query<(Entity, &mut Transform, &mut Moving, &Movable)>,
) {
    for (entity, mut transform, mut moving, movable) in moving_units.iter_mut() {
        // Update progress based on time and speed
        moving.progress += time.delta_secs() * movable.speed;

        if moving.progress >= 1.0 {
            // Movement complete
            transform.translation = moving.to;
            commands.entity(entity).remove::<Moving>();
        } else {
            // Smooth interpolation between grid cells
            transform.translation = moving.from.lerp(moving.to, moving.progress);
        }
    }
}

// A* Node for priority queue
#[derive(Copy, Clone, Eq, PartialEq)]
struct Node {
    position: GridCoords,
    f_score: i32, // f = g + h
    g_score: i32, // cost from start
}

// Required for BinaryHeap
impl Ord for Node {
    fn cmp(&self, other: &Self) -> Ordering {
        // Reverse ordering for min-heap
        other.f_score.cmp(&self.f_score)
    }
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

// A* pathfinding algorithm
fn a_star_pathfinding(
    start: &GridCoords,
    end: &GridCoords,
    blocked: &HashSet<GridCoords>,
) -> Vec<GridCoords> {
    // If the destination is blocked, we can't go there
    if blocked.contains(end) {
        return Vec::new();
    }

    // Initialize data structures
    let mut open_set = BinaryHeap::new();
    let mut came_from: HashMap<GridCoords, GridCoords> = HashMap::new();
    let mut g_score: HashMap<GridCoords, i32> = HashMap::new();
    let mut f_score: HashMap<GridCoords, i32> = HashMap::new();
    let mut closed_set: HashSet<GridCoords> = HashSet::new();

    // Start node
    g_score.insert(*start, 0);
    f_score.insert(*start, heuristic(start, end));
    open_set.push(Node {
        position: *start,
        f_score: heuristic(start, end),
        g_score: 0,
    });

    while let Some(current) = open_set.pop() {
        let current_pos = current.position;

        // Check if we reached the goal
        if current_pos == *end {
            return reconstruct_path(&came_from, current_pos);
        }

        // Mark as visited
        closed_set.insert(current_pos);

        // Check all neighbors
        for neighbor_pos in get_neighbors(&current_pos) {
            // Skip if already visited or blocked
            if closed_set.contains(&neighbor_pos) || blocked.contains(&neighbor_pos) {
                continue;
            }

            // Calculate tentative g score
            let movement_cost = if is_diagonal(&current_pos, &neighbor_pos) {
                14
            } else {
                10
            };
            let tentative_g = current.g_score + movement_cost;

            // If this path is better than any previous one
            let is_better = match g_score.get(&neighbor_pos) {
                Some(score) => tentative_g < *score,
                None => true,
            };

            if is_better {
                // Record this path
                came_from.insert(neighbor_pos, current_pos);
                g_score.insert(neighbor_pos, tentative_g);

                let h = heuristic(&neighbor_pos, end);
                let f = tentative_g + h;
                f_score.insert(neighbor_pos, f);

                // Only add to open set if not already there
                if !open_set.iter().any(|n| n.position == neighbor_pos) {
                    open_set.push(Node {
                        position: neighbor_pos,
                        f_score: f,
                        g_score: tentative_g,
                    });
                }
            }
        }
    }

    // No path found
    Vec::new()
}

// Manhattan distance heuristic
fn heuristic(a: &GridCoords, b: &GridCoords) -> i32 {
    let dx = (b.x - a.x).abs();
    let dy = (b.y - a.y).abs();
    10 * (dx + dy)
}

// Check if movement between two coords is diagonal
fn is_diagonal(a: &GridCoords, b: &GridCoords) -> bool {
    a.x != b.x && a.y != b.y
}

// Get all valid adjacent positions
fn get_neighbors(pos: &GridCoords) -> Vec<GridCoords> {
    let neighbors = vec![
        GridCoords {
            x: pos.x + 1,
            y: pos.y,
        },
        GridCoords {
            x: pos.x - 1,
            y: pos.y,
        },
        GridCoords {
            x: pos.x,
            y: pos.y + 1,
        },
        GridCoords {
            x: pos.x,
            y: pos.y - 1,
        },
        GridCoords {
            x: pos.x + 1,
            y: pos.y + 1,
        },
        GridCoords {
            x: pos.x + 1,
            y: pos.y - 1,
        },
        GridCoords {
            x: pos.x - 1,
            y: pos.y + 1,
        },
        GridCoords {
            x: pos.x - 1,
            y: pos.y - 1,
        },
    ];

    neighbors
}

// Reconstruct path from came_from map
fn reconstruct_path(
    came_from: &HashMap<GridCoords, GridCoords>,
    end: GridCoords,
) -> Vec<GridCoords> {
    let mut path = Vec::new();
    let mut current = end;

    while let Some(&prev) = came_from.get(&current) {
        path.push(current);
        current = prev;

        // Safety check for cycles (shouldn't happen with A* but just in case)
        if path.len() > 1000 {
            break;
        }
    }

    // Path is from end to start, so reverse it
    path.reverse();
    path
}
