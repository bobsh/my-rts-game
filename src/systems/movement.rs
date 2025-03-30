use crate::components::movement::{Movable, MoveTarget, Moving};
use crate::systems::ldtk_calibration::LdtkCalibration;
use bevy::prelude::*;
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

/// Calculates a grid position from cursor position and checks if it's valid
pub fn calculate_cursor_grid_position(
    cursor_position: Vec2,
    camera_q: &Query<(&Camera, &GlobalTransform)>,
    ldtk_worlds: &Query<&GlobalTransform, With<LdtkProjectHandle>>,
    ldtk_calibration: &LdtkCalibration,
) -> Option<GridCoords> {
    // Process the cursor ray and get world position
    let (camera, camera_transform) = camera_q.single();
    let Ok(cursor_ray) = camera.viewport_to_world(camera_transform, cursor_position) else {
        return None;
    };

    // Get the world position of the cursor
    let cursor_world_pos = cursor_ray.origin.truncate();

    // Get the LDTK world transform
    let ldtk_world_transform = ldtk_worlds.single();

    // To get a position relative to LDTK world, we need to account for the world transform
    let ldtk_relative_pos = cursor_world_pos - ldtk_world_transform.translation().truncate();

    // Convert to grid coordinates with fixed grid offset as specified
    let target_grid = GridCoords {
        x: ((ldtk_relative_pos.x) / 64.0).floor() as i32 + ldtk_calibration.grid_offset.x,
        y: ((ldtk_relative_pos.y) / 64.0).floor() as i32 + ldtk_calibration.grid_offset.y,
    };

    Some(target_grid)
}

/// Sets a movement target for an entity if the target position is valid
pub fn set_movement_target(
    entity: Entity,
    target_grid: GridCoords,
    current_pos: &GridCoords,
    ldtk_tile_query: &Query<&GridCoords, With<crate::components::movement::Collider>>,
    move_targets: &mut Query<&mut MoveTarget>,
) -> bool {
    // Calculate distance to verify it's reasonable
    let dx = target_grid.x - current_pos.x;
    let dy = target_grid.y - current_pos.y;
    let distance = ((dx * dx + dy * dy) as f32).sqrt();

    // If the distance is too large, exit early
    if distance > 30.0 {
        info!(
            "Movement distance too large ({:.1}), ignoring click",
            distance
        );
        return false;
    }

    // Check if the target position is occupied by a collider
    let is_occupied = ldtk_tile_query
        .iter()
        .any(|tile_pos| tile_pos.x == target_grid.x && tile_pos.y == target_grid.y);

    if !is_occupied {
        if let Ok(mut move_target) = move_targets.get_mut(entity) {
            // Clear any existing path
            move_target.path.clear();
            // Set the new destination
            move_target.destination = Some(target_grid);
            info!(
                "Setting movement destination to {:?} for entity {:?}",
                target_grid, entity
            );
            return true;
        } else {
            info!("Entity {:?} has no MoveTarget component", entity);
        }
    } else {
        info!(
            "Target position {:?} is occupied by a collider",
            target_grid
        );
    }

    false
}

// Fixed system to handle right-click movement input
#[allow(clippy::too_many_arguments)]
fn handle_movement_input(
    mouse_button: Res<ButtonInput<MouseButton>>,
    windows: Query<&Window>,
    camera_q: Query<(&Camera, &GlobalTransform)>,
    selected_units: Query<(Entity, &GridCoords), With<crate::components::unit::Selected>>,
    mut move_targets: Query<&mut MoveTarget>,
    ldtk_tile_query: Query<&GridCoords, With<crate::components::movement::Collider>>,
    gatherers: Query<Entity, With<crate::systems::resource_gathering::Gathering>>,
    ldtk_calibration: Res<LdtkCalibration>,
    ldtk_worlds: Query<&GlobalTransform, With<LdtkProjectHandle>>,
) {
    // Only process right-click inputs
    if !mouse_button.just_pressed(MouseButton::Right) {
        return;
    }

    let window = windows.single();
    let Some(cursor_position) = window.cursor_position() else {
        return;
    };

    // Get the first selected unit
    let Some((entity, current_pos)) = selected_units.iter().next() else {
        return;
    };

    // Skip if this entity is gathering (movement interruption is handled in resource_gathering.rs)
    if gatherers.contains(entity) {
        info!("Entity is currently gathering, ignoring movement command");
        return;
    }

    // Calculate grid position from cursor
    let Some(target_grid) =
        calculate_cursor_grid_position(cursor_position, &camera_q, &ldtk_worlds, &ldtk_calibration)
    else {
        return;
    };

    info!("Raw cursor world position: {:?}", cursor_position);
    info!("Target grid coordinates: {:?}", target_grid);
    info!(
        "Current position: {:?}, Target: {:?}",
        current_pos, target_grid
    );

    // Try to set movement target
    set_movement_target(
        entity,
        target_grid,
        current_pos,
        &ldtk_tile_query,
        &mut move_targets,
    );
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
            // CRITICAL: Check if the destination is a reasonable distance
            // This ensures we don't process destinations set from other systems that are too far
            let dx = destination.x - current_pos.x;
            let dy = destination.y - current_pos.y;
            let distance = ((dx * dx + dy * dy) as f32).sqrt();

            // Enforce maximum distance limit for ALL movement, even from other systems
            if distance > 30.0 {
                info!(
                    "Path distance too large ({:.1}), canceling movement to {:?}",
                    distance, destination
                );
                move_target.destination = None;
                continue;
            }

            if move_target.path.is_empty() {
                info!(
                    "Calculating path from {:?} to {:?}",
                    current_pos, destination
                );

                // Store obstacle positions for debugging
                let obstacle_count = obstacles.iter().count();
                info!("Found {} obstacles", obstacle_count);

                // Define a function to find neighboring grid positions
                let neighbors = |pos: &GridCoords| {
                    let dirs = [
                        (0, 1),
                        (1, 0),
                        (0, -1),
                        (-1, 0), // Cardinal directions
                        (1, 1),
                        (1, -1),
                        (-1, 1),
                        (-1, -1), // Diagonals
                    ];

                    dirs.iter()
                        .map(|(dx, dy)| GridCoords {
                            x: pos.x + dx,
                            y: pos.y + dy,
                        })
                        .filter(|next_pos| {
                            // Use reasonable boundaries
                            const MAX_BOUND: i32 = 50;
                            let min_x = (current_pos.x - MAX_BOUND).min(destination.x - MAX_BOUND);
                            let max_x = (current_pos.x + MAX_BOUND).max(destination.x + MAX_BOUND);
                            let min_y = (current_pos.y - MAX_BOUND).min(destination.y - MAX_BOUND);
                            let max_y = (current_pos.y + MAX_BOUND).max(destination.y + MAX_BOUND);

                            if next_pos.x < min_x
                                || next_pos.x > max_x
                                || next_pos.y < min_y
                                || next_pos.y > max_y
                            {
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
                if let Some((path, _)) = astar(current_pos, neighbors, heuristic, |pos| {
                    pos.x == destination.x && pos.y == destination.y
                }) {
                    // Skip the first position (current position)
                    if path.len() > 1 {
                        move_target.path = path.into_iter().skip(1).collect();
                        info!(
                            "Path found with {} steps for entity {:?}",
                            move_target.path.len(),
                            entity
                        );
                    } else {
                        info!("Path is too short, already at destination");
                        move_target.destination = None;
                    }
                } else {
                    info!("No path found to destination for entity {:?}", entity);
                    // Clear the destination if no path is found
                    move_target.destination = None;
                }
            }
        }
    }
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
            // Add 32.0 (half tile size) to center movement on tiles
            let current_world_pos = Vec3::new(
                current_pos.x as f32 * 64.0 + 32.0,
                current_pos.y as f32 * 64.0 + 32.0,
                0.0,
            );

            let next_world_pos = Vec3::new(
                next_pos.x as f32 * 64.0 + 32.0,
                next_pos.y as f32 * 64.0 + 32.0,
                0.0,
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

            // Update grid coordinates based on the target position directly
            // This eliminates any potential for rounding errors
            if let Ok(mut coords) = grid_coords.get_mut(entity) {
                // Calculate grid coords based on absolute world position / tile size
                // Subtract the 32.0 offset when converting from world to grid
                *coords = GridCoords {
                    x: ((moving.to.x - 32.0) / 64.0).floor() as i32,
                    y: ((moving.to.y - 32.0) / 64.0).floor() as i32,
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
