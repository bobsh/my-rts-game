use bevy::prelude::*;
use bevy::input::mouse::MouseButton;
use bevy::window::PrimaryWindow;
use bevy::input::ButtonInput;
use crate::components::unit::{Selected, Velocity, MoveMarker};

// This system handles right-click commands for selected units
pub fn move_command_system(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    camera_query: Query<(&Camera, &GlobalTransform)>,
    mouse_button_input: Res<ButtonInput<MouseButton>>,
    mut query: Query<(&mut Velocity, &mut Transform), With<Selected>>,
) {
    // Only process right-clicks
    if !mouse_button_input.just_pressed(MouseButton::Right) {
        return;
    }

    // Get the primary window
    let window = window_query.single();
    
    // Get the camera
    let (camera, camera_transform) = camera_query.single();
    
    // Get the cursor position
    if let Some(cursor_position) = window.cursor_position() {
        // Convert cursor position to world coordinates
        if let Some(world_position) = camera.viewport_to_world(camera_transform, cursor_position) {
            let target_pos = world_position.origin.truncate();
            
            // Create a move marker at the target location
            commands.spawn((
                SpriteBundle {
                    sprite: Sprite {
                        color: Color::srgba(0.2, 0.8, 0.2, 0.7),
                        custom_size: Some(Vec2::new(15.0, 15.0)),
                        ..default()
                    },
                    transform: Transform::from_translation(Vec3::new(target_pos.x, target_pos.y, 0.0)),
                    ..default()
                },
                MoveMarker {
                    timer: Timer::from_seconds(1.0, TimerMode::Once),
                },
            ));
            
            // Set target for all selected units
            for (mut velocity, _) in query.iter_mut() {
                velocity.target = Some(target_pos);
                velocity.speed = 100.0;
            }
        }
    }
}

// This system handles the actual movement of units
pub fn movement_system(
    time: Res<Time>,
    mut query: Query<(&mut Velocity, &mut Transform)>,
    _commands: Commands, // Added underscore to unused variable 
    _asset_server: Res<AssetServer>, // Added underscore to unused variable
) {
    for (mut velocity, mut transform) in query.iter_mut() {
        if let Some(target) = velocity.target {
            let current_pos = transform.translation.truncate();
            let to_target = target - current_pos;
            
            // If we're close enough to the target, stop moving
            if to_target.length() < 5.0 {
                velocity.target = None;
                velocity.value = Vec2::ZERO;
                continue;
            }
            
            // Calculate direction and movement this frame
            let direction = to_target.normalize();
            velocity.value = direction * velocity.speed;
            let delta_movement = velocity.value * time.delta_seconds();
            
            // Update position
            transform.translation.x += delta_movement.x;
            transform.translation.y += delta_movement.y;
            
            // Optional: Rotate unit to face movement direction
            // let angle = direction.y.atan2(direction.x);
            // transform.rotation = Quat::from_rotation_z(angle);
        }
    }
}

pub fn show_destination_markers(
    mut commands: Commands,
    time: Res<Time>,
    mut markers_query: Query<(Entity, &mut MoveMarker, &mut Sprite)>,
) {
    // Update existing markers
    for (entity, mut marker, mut sprite) in markers_query.iter_mut() {
        marker.timer.tick(time.delta());
        
        // Fade out the marker as the timer progresses
        let alpha = 1.0 - marker.timer.fraction();
        sprite.color = sprite.color.with_alpha(alpha);
        
        // Remove marker when timer is finished
        if marker.timer.finished() {
            commands.entity(entity).despawn();
        }
    }
}
