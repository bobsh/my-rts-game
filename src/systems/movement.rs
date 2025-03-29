use bevy::prelude::*;
use crate::components::movement::{Movable, MoveTarget, Moving};
use bevy_ecs_ldtk::prelude::*;
use pathfinding::prelude::astar;

pub struct MovementPlugin;

impl Plugin for MovementPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (
            update_movement,
            calculate_path,
            move_along_path
        ).chain());
    }
}

// System to calculate a path when a destination is set
fn calculate_path(
    _commands: Commands, // Add underscore to unused variable
    mut query: Query<(Entity, &GridCoords, &mut MoveTarget), (With<Movable>, Without<Moving>)>,
    ldtk_entities: Query<&GridCoords>,
    _ldtk_level: Query<&LevelIid>, // Add underscore to unused variable,
) {
    if let Ok(level_iid) = _ldtk_level.get_single() {ut query {
        for (entity, current_pos, mut move_target) in &mut query {
            if let Some(destination) = move_target.destination {
                if move_target.path.is_empty() {already have a path
                    // Only recalculate if we don't already have a path, destination);
                    info!("Calculating path from {:?} to {:?}", current_pos, destination);
                // Define a function to find neighboring grid positions
                    // Define a function to find neighboring grid positions
                    let neighbors = |pos: &GridCoords| {
                        let dirs = [0), (0, -1), (-1, 0), // Cardinal directions
                            (0, 1), (1, 0), (0, -1), (-1, 0), // Cardinal directions
                            (1, 1), (1, -1), (-1, 1), (-1, -1), // Diagonals
                        ];
                    dirs.iter()
                        dirs.iter()dy)| GridCoords {
                            .map(|(dx, dy)| GridCoords {
                                x: pos.x + dx,
                                y: pos.y + dy,
                            })r(|next_pos| {
                            .filter(|next_pos| {tion is valid (not occupied by other entities)
                                // Check if the position is valid (not occupied by other entities)
                                !ldtk_entities.iter().any(|entity_pos| {.y == next_pos.y
                                    entity_pos.x == next_pos.x && entity_pos.y == next_pos.y
                                })
                            })pos| (pos, 1)) // Cost is always 1 for now
                            .map(|pos| (pos, 1)) // Cost is always 1 for now
                            .collect::<Vec<_>>()
                    };
                // Calculate heuristic (Manhattan distance)
                    // Calculate heuristic (Manhattan distance)
                    let heuristic = |pos: &GridCoords| {s.y - destination.y).abs()) as u32
                        ((pos.x - destination.x).abs() + (pos.y - destination.y).abs()) as u32
                    };
                // Check if already at destination
                    // Check if already at destinationurrent_pos.y == destination.y {
                    if current_pos.x == destination.x && current_pos.y == destination.y {
                        // Already at destination, clear the target
                        move_target.destination = None;
                        continue;
                    }
                // Find path using A* algorithm
                    // Find path using A* algorithm
                    if let Some((path, _)) = astar(
                        current_pos,pos),
                        |pos| neighbors(pos),
                        |pos| heuristic(pos),n.x && pos.y == destination.y
                        |pos| pos.x == destination.x && pos.y == destination.y
                    ) {Skip the first position (current position)
                        // Skip the first position (current position)
                        move_target.path = path.into_iter().skip(1).collect();
                        info!("Path found with {} steps", move_target.path.len());get.path.len(), entity);
                    } else {
                        // No path foundoo short, already at destination");
                        info!("No path found to destination");
                        move_target.destination = None;
                    }e {
                }   // No path found
            }       info!("No path found to destination for entity {:?}", entity);
        }           move_target.destination = None;
    }           }
}           }
        }
// System to move along the calculated path
fn move_along_path(
    mut commands: Commands,
    mut query: Query<(Entity, &GridCoords, &mut MoveTarget, &Movable), Without<Moving>>,
    _time: Res<Time>,
) { mut commands: Commands,
    for (entity, current_pos, mut move_target, _movable) in &mut query {ithout<Moving>>,
        if !move_target.path.is_empty() {
            let next_pos = move_target.path[0];
    for (entity, current_pos, mut move_target, _movable) in &mut query {
            // Convert grid coordinates to world positions
            let current_world_pos = Vec3::new(;
                current_pos.x as f32 * 64.0,
                current_pos.y as f32 * 64.0,orld positions
                0.0rent_world_pos = Vec3::new(
            );  current_pos.x as f32 * 64.0,
                current_pos.y as f32 * 64.0,
            let next_world_pos = Vec3::new(
                next_pos.x as f32 * 64.0,
                next_pos.y as f32 * 64.0,
                0.0t_world_pos = Vec3::new(
            );  next_pos.x as f32 * 64.0,
                next_pos.y as f32 * 64.0,
            // Start moving to the next position
            commands.entity(entity).insert(Moving {
                from: current_world_pos,
                to: next_world_pos,next position
                progress: 0.0,tity).insert(Moving {
            }); from: current_world_pos,
                to: next_world_pos,
            // Remove the position we're moving to from the path
            move_target.path.remove(0);
        } else if move_target.destination.is_some() {
            // We've reached the end of the path, clear the destination
            move_target.destination = None;
        } else if move_target.destination.is_some() {
    }       // We've reached the end of the path, clear the destination
}           move_target.destination = None;
        }
// System to update entity position while moving
fn update_movement(
    mut commands: Commands,
    mut query: Query<(Entity, &mut Transform, &mut Moving, &Movable)>,
    mut grid_coords: Query<&mut GridCoords>,
    time: Res<Time>,mmands,
) { mut query: Query<(Entity, &mut Transform, &mut Moving, &Movable)>,
    for (entity, mut transform, mut moving, movable) in &mut query {
        // Update progress
        moving.progress += time.delta_secs() * movable.speed;
    for (entity, mut transform, mut moving, movable) in &mut query {
        if moving.progress >= 1.0 {
            // Movement completedelta_secs() * movable.speed;
            transform.translation = moving.to;
        if moving.progress >= 1.0 {
            // Update grid coordinates
            if let Ok(mut coords) = grid_coords.get_mut(entity) {
                *coords = GridCoords {
                    x: (moving.to.x / 64.0).round() as i32,
                    y: (moving.to.y / 64.0).round() as i32,ity) {
                };oords = GridCoords {
            }       x: (moving.to.x / 64.0).round() as i32,
                    y: (moving.to.y / 64.0).round() as i32,
            // Remove Moving component
            commands.entity(entity).remove::<Moving>();
        } else {
            // Interpolate positionent
            transform.translation = moving.from.lerp(moving.to, moving.progress);
        } else {
    }       // Interpolate position
}           transform.translation = moving.from.lerp(moving.to, moving.progress);
        }
    }
}
