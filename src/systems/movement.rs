use bevy::prelude::*;
use crate::components::movement::{Movable, MoveTarget, Moving};
use bevy_ecs_ldtk::prelude::*;
use pathfinding::prelude::astar;

pub struct MovementPlugin;

impl Plugin for MovementPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, handle_movement_input)
           .add_systems(Update, update_movement.after(handle_movement_input))
           .add_systems(Update, calculate_path.after(handle_movement_input))
           .add_systems(Update, move_along_path.after(calculate_path));
    }
}

// Fixed system to handle right-click movement input
fn handle_movement_input(
    mouse_button: Res<ButtonInput<MouseButton>>,
    windows: Query<&Window>,
    camera_q: Query<(&Camera, &GlobalTransform)>,
    selected_units: Query<(Entity, &GridCoords), With<crate::components::unit::Selected>>,
    mut move_targets: Query<&mut MoveTarget>,
    ldtk_tile_query: Query<&GridCoords, With<crate::components::movement::Collider>>,
    gatherers: Query<(), With<crate::systems::resource_gathering::Gathering>>,
) {
    // Only process right-click inputs when not already gathering resources
    if !mouse_button.just_pressed(MouseButton::Right) || !gatherers.is_empty() {
        return;
    }

    let window = windows.single();
    let Some(cursor_position) = window.cursor_position() else {
        return;
    };

    let (camera, camera_transform) = camera_q.single();
    let Ok(cursor_ray) = camera.viewport_to_world(camera_transform, cursor_position) else {
        return;
    };

    // Convert cursor world position to grid coordinates - FIXED
    let cursor_world_pos = cursor_ray.origin.truncate();

    // Log raw position for debugging
    info!("Raw cursor world position: {:?}", cursor_world_pos);

    // FIXED: Division by tile size (64.0) to get grid coordinates
    // Using floor to properly handle negative coordinates
    let target_grid = GridCoords {
        x: (cursor_world_pos.x / 64.0).floor() as i32,
        y: (cursor_world_pos.y / 64.0).floor() as i32,
    };

    info!("Target grid coordinates: {:?}", target_grid);

    // Get the first selected unit's position for reference
    if let Some((entity, current_pos)) = selected_units.iter().next() {
        // Log the current position and calculated target for debugging
        info!("Current position: {:?}, Target: {:?}", current_pos, target_grid);

        // Calculate distance to verify it's reasonable
        let dx = target_grid.x - current_pos.x;
        let dy = target_grid.y - current_pos.y;
        let distance = ((dx * dx + dy * dy) as f32).sqrt();

        info!("Movement distance: {:.1} grid cells", distance);

        // IMPROVED: Reduced maximum allowed distance to 30.0 (was 50.0)
        if distance > 30.0 {
            info!("Movement distance too large ({}), ignoring click", distance);
            return;
        }

        // Check if the target position is occupied by a collider
        let is_occupied = ldtk_tile_query.iter().any(|tile_pos| {
            tile_pos.x == target_grid.x && tile_pos.y == target_grid.y
        });

        if !is_occupied {
            if let Ok(mut move_target) = move_targets.get_mut(entity) {
                // Clear any existing path
                move_target.path.clear();
                // Set the new destination
                move_target.destination = Some(target_grid);
                info!("Setting movement destination to {:?} for entity {:?}", target_grid, entity);
            } else {
                info!("Selected entity {:?} has no MoveTarget component", entity);
            }
        } else {
            info!("Target position {:?} is occupied by a collider", target_grid);
        }
    }
}

// System to calculate a path when a destination is set
fn calculate_path(
    _commands: Commands,
    mut query: Query<(Entity, &GridCoords, &mut MoveTarget), (With<Movable>, Without<Moving>)>,
    obstacles: Query<&GridCoords, With<crate::components::movement::Collider>>,
    _ldtk_level: Query<&LevelIid>,
) {
    for (entity, current_pos, mut move_target) in &mut query {
        if let Some(destination) = move_target.destination {
            if move_target.path.is_empty() {
                // Only recalculate if we don't already have a path
                info!("Calculating path from {:?} to {:?}", current_pos, destination);

                // Collect obstacles for debugging
                let obstacle_positions: Vec<(i32, i32)> = obstacles
                    .iter()
                    .map(|pos| (pos.x, pos.y))
                    .collect();

                info!("Found {} obstacles", obstacle_positions.len());

                // IMPROVED: Modified the function to use more realistic boundaries
                // Define a function to find neighboring grid positions
                let neighbors = |pos: &GridCoords| {
                    let dirs = [
                        (0, 1), (1, 0), (0, -1), (-1, 0), // Cardinal directions
                        (1, 1), (1, -1), (-1, 1), (-1, -1), // Diagonals
                    ];

                    dirs.iter()
                        .map(|(dx, dy)| GridCoords {
                            x: pos.x + dx,
                            y: pos.y + dy,
                        })
                        .filter(|next_pos| {
                            // IMPROVED: Use a more reasonable boundary
                            // Don't allow positions that are too far from the current position or destination
                            const MAX_BOUND: i32 = 50;
                            let min_x = (current_pos.x - MAX_BOUND).min(destination.x - MAX_BOUND);
                            let max_x = (current_pos.x + MAX_BOUND).max(destination.x + MAX_BOUND);
                            let min_y = (current_pos.y - MAX_BOUND).min(destination.y - MAX_BOUND);
                            let max_y = (current_pos.y + MAX_BOUND).max(destination.y + MAX_BOUND);

                            if next_pos.x < min_x || next_pos.x > max_x ||
                               next_pos.y < min_y || next_pos.y > max_y {
                                return false;
                            }

                            // Only filter out positions occupied by actual obstacles
                            !obstacles.iter().any(|obstacle_pos| {
                                obstacle_pos.x == next_pos.x && obstacle_pos.y == next_pos.y
                            })
                        })
                        .map(|pos| {
                            // Cost is 1 for cardinal, sqrt(2) for diagonal (scaled to int)
                            let dx = (pos.x - current_pos.x).abs();
                            let dy = (pos.y - current_pos.y).abs();
                            if dx == 1 && dy == 1 {
                                (pos, 14) // Approximate sqrt(2) * 10
                            } else {
                                (pos, 10) // 10 for scaling purposes
                            }
                        })
                        .collect::<Vec<_>>()
                };

                // Calculate heuristic (Manhattan distance)
                let heuristic = |pos: &GridCoords| {
                    ((pos.x - destination.x).abs() + (pos.y - destination.y).abs()) as u32 * 10
                };

                // Check if already at destination
                if current_pos.x == destination.x && current_pos.y == destination.y {
                    // Already at destination, clear the target
                    move_target.destination = None;
                    continue;
                }

                // Find path using A* algorithm
                if let Some((path, _)) = astar(
                    current_pos,
                    |pos| neighbors(pos),
                    |pos| heuristic(pos),
                    |pos| pos.x == destination.x && pos.y == destination.y
                ) {
                    // Skip the first position (current position)
                    if path.len() > 1 {
                        move_target.path = path.into_iter().skip(1).collect();
                        info!("Path found with {} steps for entity {:?}", move_target.path.len(), entity);
                    } else {
                        info!("Path is too short, already at destination");
                        move_target.destination = None;
                    }
                } else {
                    // No path found - provide detailed debug info
                    info!("No path found to destination for entity {:?}", entity);
                    info!("Start: {:?}, End: {:?}, Distance: {}",
                        current_pos,
                        destination,
                        ((current_pos.x - destination.x).pow(2) + (current_pos.y - destination.y).pow(2)) as f32
                    );

                    // IMPROVED: Better fallback path with obstacle avoidance
                    let direct_path = create_safer_direct_path(current_pos, &destination, &obstacles);

                    // IMPROVED: Only use fallback path if it's reasonably short
                    if direct_path.len() <= 30 {
                        move_target.path = direct_path;
                        info!("Using fallback path with {} steps", move_target.path.len());
                    } else {
                        // Path is too long, likely invalid - cancel movement
                        move_target.destination = None;
                        info!("Fallback path too long ({} steps), canceling movement", direct_path.len());
                    }
                }
            }
        }
    }
}

// IMPROVED: Better direct path creation that includes basic obstacle avoidance
fn create_safer_direct_path(start: &GridCoords, end: &GridCoords,
                          obstacles: &Query<&GridCoords, With<crate::components::movement::Collider>>) -> Vec<GridCoords> {
    let mut path = Vec::new();
    let dx = end.x - start.x;
    let dy = end.y - start.y;
    let steps = dx.abs().max(dy.abs());

    if steps == 0 {
        return path;
    }

    let step_x = dx as f32 / steps as f32;
    let step_y = dy as f32 / steps as f32;

    // Keep track of points we've tried to avoid obstacles
    let mut detour_points = Vec::new();

    for i in 1..=steps {
        let x = start.x + (step_x * i as f32).round() as i32;
        let y = start.y + (step_y * i as f32).round() as i32;
        let point = GridCoords { x, y };

        // Check if this point is blocked by an obstacle
        let is_blocked = obstacles.iter().any(|obstacle|
            obstacle.x == point.x && obstacle.y == point.y
        );

        if is_blocked {
            // Try to find a way around - check adjacent non-diagonal cells
            let adjacent_points = [
                GridCoords { x: point.x + 1, y: point.y },
                GridCoords { x: point.x - 1, y: point.y },
                GridCoords { x: point.x, y: point.y + 1 },
                GridCoords { x: point.x, y: point.y - 1 },
            ];

            // Find the first non-blocked adjacent point
            for adj_point in adjacent_points.iter() {
                let adj_blocked = obstacles.iter().any(|obstacle|
                    obstacle.x == adj_point.x && obstacle.y == adj_point.y
                );

                if !adj_blocked && !detour_points.contains(adj_point) {
                    path.push(*adj_point);
                    detour_points.push(*adj_point);
                    break;
                }
            }
        } else {
            path.push(point);
        }
    }

    path
}

// System to move along the calculated path
fn move_along_path(
    mut commands: Commands,
    mut query: Query<(Entity, &GridCoords, &mut MoveTarget, &Movable), Without<Moving>>,
    _time: Res<Time>,
) {
    for (entity, current_pos, mut move_target, _movable) in &mut query {
        if !move_target.path.is_empty() {
            let next_pos = move_target.path[0];

            // Convert grid coordinates to world positions
            let current_world_pos = Vec3::new(
                current_pos.x as f32 * 64.0,
                current_pos.y as f32 * 64.0,
                0.0
            );
            let next_world_pos = Vec3::new(
                next_pos.x as f32 * 64.0,
                next_pos.y as f32 * 64.0,
                0.0
            );

            // Start moving to the next position
            commands.entity(entity).insert(Moving {
                from: current_world_pos,
                to: next_world_pos,
                progress: 0.0,
            });

            // Remove the position we're moving to from the path
            move_target.path.remove(0);
        } else if move_target.destination.is_some() {
            // We've reached the end of the path, clear the destination
            move_target.destination = None;
        }
    }
}

// System to update entity position while moving
fn update_movement(
    mut commands: Commands,
    mut query: Query<(Entity, &mut Transform, &mut Moving, &Movable)>,
    mut grid_coords: Query<&mut GridCoords>,
    time: Res<Time>,
) {
    for (entity, mut transform, mut moving, movable) in &mut query {
        // Update progress
        moving.progress += time.delta_secs() * movable.speed;

        if moving.progress >= 1.0 {
            // Movement complete
            transform.translation = moving.to;

            // Update grid coordinates
            if let Ok(mut coords) = grid_coords.get_mut(entity) {
                *coords = GridCoords {
                    x: (moving.to.x / 64.0).round() as i32,
                    y: (moving.to.y / 64.0).round() as i32,
                };
            }

            // Remove Moving component
            commands.entity(entity).remove::<Moving>();
        } else {
            // Interpolate position
            transform.translation = moving.from.lerp(moving.to, moving.progress);
        }
    }
}
