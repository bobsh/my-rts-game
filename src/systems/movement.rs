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

// Handle right-click to set movement destinations for selected units
fn handle_movement_input(
    mouse_button: Res<ButtonInput<MouseButton>>,
    windows: Query<&Window>,
    camera_q: Query<(&Camera, &GlobalTransform)>,
    // Use LdtkProjectHandle instead of LdtkAsset
    level_transform: Query<&GlobalTransform, With<LdtkProjectHandle>>,
    selected_units: Query<Entity, (With<Selected>, With<Movable>)>,
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

    // Set movement target for all selected units
    for selected_entity in selected_units.iter() {
        if let Ok(mut move_target) = unit_targets.get_mut(selected_entity) {
            move_target.destination = Some(grid_pos);
        }
    }
}

// Start movement for units with targets
fn start_unit_movement(
    mut commands: Commands,
    // Use LdtkProjectHandle here too
    level_transform: Query<&GlobalTransform, With<LdtkProjectHandle>>,
    mut movable_units: Query<
        (Entity, &Transform, &GridCoords, &mut MoveTarget, &Movable),
        Without<Moving>,
    >,
) {
    // Get level transform
    let level_transform = match level_transform.get_single() {
        Ok(transform) => transform,
        Err(_) => return,
    };

    // Process each unit with a move target
    for (entity, transform, grid_coords, mut move_target, _) in movable_units.iter_mut() {
        if let Some(target) = move_target.destination {
            if target == *grid_coords {
                // Already at destination
                move_target.destination = None;
                continue;
            }

            // Calculate world positions
            let current_pos = transform.translation;
            let target_world_x = level_transform.translation().x + target.x as f32 * TILE_SIZE;
            let target_world_y = level_transform.translation().y + target.y as f32 * TILE_SIZE;
            let target_pos = Vec3::new(target_world_x, target_world_y, current_pos.z);

            // Start movement animation
            commands.entity(entity).insert(Moving {
                from: current_pos,
                to: target_pos,
                progress: 0.0,
            });

            // Update grid coordinates (logical position change)
            commands.entity(entity).insert(target);

            // Clear target
            move_target.destination = None;
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
