use bevy::prelude::*;
use crate::components::movement::{Movable, MoveTarget, Moving};
use bevy_ecs_ldtk::prelude::*;
use pathfinding::prelude::astar;

pub struct MovementPlugin;

// Add a resource to store coordinate offsets that can be adjusted at runtime
#[derive(Resource)]
pub struct CoordinateOffset {
    pub x: f32,
    pub y: f32,
}

impl Default for CoordinateOffset {
    fn default() -> Self {
        Self {
            x: 0.0,
            y: 0.0
        }
    }
}

impl Plugin for MovementPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<CoordinateOffset>()
           .add_systems(Update, handle_movement_input)
           .add_systems(Update, update_movement.after(handle_movement_input))
           .add_systems(Update, calculate_path.after(handle_movement_input))
           .add_systems(Update, move_along_path.after(calculate_path))
           .add_systems(Update, adjust_coordinate_offset_debug); // Add a debug system to adjust offsets
    }
}

// Debug system to adjust coordinate offset with keyboard
fn adjust_coordinate_offset_debug(
    keys: Res<ButtonInput<KeyCode>>,
    mut offset: ResMut<CoordinateOffset>
) {
    // Only active in debug mode with Shift key held
    if keys.pressed(KeyCode::ShiftLeft) {
        if keys.pressed(KeyCode::KeyO) { // O for "Offset"
            if keys.pressed(KeyCode::ArrowLeft) {
                offset.x -= 1.0;
                info!("Offset X: {}, Y: {}", offset.x, offset.y);
            }
            if keys.pressed(KeyCode::ArrowRight) {
                offset.x += 1.0;
                info!("Offset X: {}, Y: {}", offset.x, offset.y);
            }
            if keys.pressed(KeyCode::ArrowUp) {
                offset.y += 1.0;
                info!("Offset X: {}, Y: {}", offset.x, offset.y);
            }
            if keys.pressed(KeyCode::ArrowDown) {
                offset.y -= 1.0;
                info!("Offset X: {}, Y: {}", offset.x, offset.y);
            }
            // Reset to zero with R
            if keys.just_pressed(KeyCode::KeyR) {
                offset.x = 0.0;
                offset.y = 0.0;
                info!("Offset reset to zero");
            }
        }
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
    offset: Res<CoordinateOffset>, // Use the offset resource
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

    // Convert cursor world position to grid coordinates
    let cursor_world_pos = cursor_ray.origin.truncate();

    // Log raw position for debugging
    info!("Raw cursor world position: {:?}", cursor_world_pos);

    // IMPORTANT: Apply the coordinate offset before conversion to grid coordinates
    let adjusted_world_pos = Vec2::new(
        cursor_world_pos.x + offset.x,
        cursor_world_pos.y + offset.y
    );

    info!("Adjusted cursor world position: {:?}", adjusted_world_pos);

    // Use round instead of floor for more accurate positioning
    let target_grid = GridCoords {
        x: (adjusted_world_pos.x / 64.0).round() as i32,
        y: (adjusted_world_pos.y / 64.0).round() as i32,
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

        // If the distance is too large, exit early
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
                // Set the new destination - this is what was actually causing the incorrect movement!
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
            // CRITICAL: Check if the destination is a reasonable distance
            // This ensures we don't process destinations set from other systems that are too far
            let dx = destination.x - current_pos.x;
            let dy = destination.y - current_pos.y;
            let distance = ((dx * dx + dy * dy) as f32).sqrt();

            // Enforce maximum distance limit for ALL movement, even from other systems
            if distance > 30.0 {
                info!("Path distance too large ({:.1}), canceling movement to {:?}", distance, destination);
                move_target.destination = None;
                continue;
            }

            if move_target.path.is_empty() {
                info!("Calculating path from {:?} to {:?}", current_pos, destination);

                // Store obstacle positions for debugging
                let obstacle_count = obstacles.iter().count();
                info!("Found {} obstacles", obstacle_count);

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
                            // Use reasonable boundaries
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
