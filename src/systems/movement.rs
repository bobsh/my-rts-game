use crate::components::movement::*;
use crate::components::unit::Selected;
use bevy::input::mouse::MouseButton;
use bevy::prelude::*;
use bevy::window::Window;
use bevy_ecs_ldtk::prelude::*;

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

// Add this function for simple grid-based pathfinding (Manhattan style)
fn calculate_path(start: &GridCoords, end: &GridCoords) -> Vec<GridCoords> {
    let mut path = Vec::new();
    let mut current = *start;

    // First move horizontally
    while current.x != end.x {
        if current.x < end.x {
            current.x += 1;
        } else {
            current.x -= 1;
        }
        path.push(current);
    }

    // Then move vertically
    while current.y != end.y {
        if current.y < end.y {
            current.y += 1;
        } else {
            current.y -= 1;
        }
        path.push(current);
    }

    path
}

// Handle right-click to set movement destinations for selected units
fn handle_movement_input(
    mouse_button: Res<ButtonInput<MouseButton>>,
    windows: Query<&Window>,
    camera_q: Query<(&Camera, &GlobalTransform)>,
    // Use LdtkProjectHandle instead of LdtkAsset
    level_transform: Query<&GlobalTransform, With<LdtkProjectHandle>>,
    selected_units: Query<(Entity, &GridCoords), (With<Selected>, With<Movable>)>,
    mut unit_targets: Query<&mut MoveTarget>,
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

    // Get just the level transform
    let level_transform = match level_transform.get_single() {
        Ok(transform) => transform,
        Err(_) => return,
    };

    // Find the grid cell that was clicked
    let level_pos = level_transform.translation().truncate();

    let relative_pos = cursor_pos - level_pos;
    let grid_pos = GridCoords {
        x: (relative_pos.x / TILE_SIZE).floor() as i32,
        y: (relative_pos.y / TILE_SIZE).floor() as i32,
    };

    // Calculate path and set target for each selected unit
    for (entity, grid_coords) in selected_units.iter() {
        if let Ok(mut move_target) = unit_targets.get_mut(entity) {
            // Don't create a path if clicking on the same cell
            if grid_coords.x == grid_pos.x && grid_coords.y == grid_pos.y {
                continue;
            }

            move_target.destination = Some(grid_pos);
            move_target.path = calculate_path(grid_coords, &grid_pos);
        }
    }
}

// Start movement for units with targets
fn start_unit_movement(
    mut commands: Commands,
    level_transform: Query<&GlobalTransform, With<LdtkProjectHandle>>,
    mut movable_units: Query<
        (Entity, &Transform, &GridCoords, &mut MoveTarget, &Movable),
        Without<Moving>,
    >,
) {
    let level_transform = match level_transform.get_single() {
        Ok(transform) => transform,
        Err(_) => return,
    };

    for (entity, transform, grid_coords, mut move_target, _) in movable_units.iter_mut() {
        // If there's no path or the path is empty but there's a destination,
        // calculate a new path
        if move_target.path.is_empty() && move_target.destination.is_some() {
            let destination = move_target.destination.unwrap();
            move_target.path = calculate_path(grid_coords, &destination);
        }

        // If there's a path, start moving to the next cell
        if !move_target.path.is_empty() {
            let next_pos = move_target.path.remove(0);

            // Calculate world positions
            let current_pos = transform.translation;

            // Add half a tile size to position units at the center of grid cells
            let target_world_x = level_transform.translation().x + (next_pos.x as f32 + 0.5) * TILE_SIZE;
            let target_world_y = level_transform.translation().y + (next_pos.y as f32 + 0.5) * TILE_SIZE;
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
            if move_target.path.is_empty() && move_target.destination.is_some() {
                if next_pos == move_target.destination.unwrap() {
                    move_target.destination = None;
                }
            }
        }
    }
}

// Animate moving units
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
