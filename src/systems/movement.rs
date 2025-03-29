use bevy::prelude::*;
use crate::components::movement::{Movable, MoveTarget, Moving};
use bevy_ecs_ldtk::prelude::*;
use pathfinding::prelude::astar;

pub struct MovementPlugin;

impl Plugin for MovementPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (
            handle_movement_input,  // New system for handling input
            update_movement,
            calculate_path,
            move_along_path
        ).chain());
    }
}

// New system to handle right-click movement input
fn handle_movement_input(
    mouse_button: Res<ButtonInput<MouseButton>>,
    windows: Query<&Window>,
    camera_q: Query<(&Camera, &GlobalTransform)>,
    selected_units: Query<(Entity, &GridCoords), With<crate::components::unit::Selected>>,
    mut move_targets: Query<&mut MoveTarget>,
    ldtk_tile_query: Query<&GridCoords>,
) {
    // Only process right-click inputs
    if !mouse_button.just_pressed(MouseButton::Right) {
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
    let target_grid = GridCoords {
        x: (cursor_world_pos.x / 64.0).round() as i32,
        y: (cursor_world_pos.y / 64.0).round() as i32,
    };

    // Check if the target position is occupied
    let is_occupied = ldtk_tile_query.iter().any(|tile_pos| {
        tile_pos.x == target_grid.x && tile_pos.y == target_grid.y
    });

    if !is_occupied {
        // Set destination for all selected units
        for (entity, _) in selected_units.iter() {
            if let Ok(mut move_target) = move_targets.get_mut(entity) {
                // Clear any existing path
                move_target.path.clear();
                // Set the new destination
                move_target.destination = Some(target_grid);
                info!("Setting movement destination to {:?} for entity {:?}", target_grid, entity);
            } else {
                info!("Selected entity {:?} has no MoveTarget component", entity);
            }
        }
    } else {
        info!("Target position {:?} is occupied", target_grid);
    }
}

// System to calculate a path when a destination is set
fn calculate_path(
    _commands: Commands,
    mut query: Query<(Entity, &GridCoords, &mut MoveTarget), (With<Movable>, Without<Moving>)>,
    ldtk_entities: Query<&GridCoords>,
    _ldtk_level: Query<&LevelIid>,
) {
    for (entity, current_pos, mut move_target) in &mut query {
        if let Some(destination) = move_target.destination {
            if move_target.path.is_empty() {
                // Only recalculate if we don't already have a path
                info!("Calculating path from {:?} to {:?}", current_pos, destination);

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
                            // Check if the position is valid (not occupied by other entities)
                            !ldtk_entities.iter().any(|entity_pos| {
                                entity_pos.x == next_pos.x && entity_pos.y == next_pos.y
                            })
                        })
                        .map(|pos| (pos, 1)) // Cost is always 1 for now
                        .collect::<Vec<_>>()
                };

                // Calculate heuristic (Manhattan distance)
                let heuristic = |pos: &GridCoords| {
                    ((pos.x - destination.x).abs() + (pos.y - destination.y).abs()) as u32
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
                    // No path found
                    info!("No path found to destination for entity {:?}", entity);
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
