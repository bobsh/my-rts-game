use bevy::prelude::*;
use bevy::input::mouse::{MouseButton, MouseButtonInput};
use bevy::window::PrimaryWindow;
use crate::components::unit::{Selectable, Selected, Unit};

pub fn selection_system(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    camera_query: Query<(&Camera, &GlobalTransform)>,
    mouse_button_input: Res<Input<MouseButton>>,
    selectable_query: Query<(Entity, &Transform, &Sprite), With<Selectable>>,
    mut selected_query: Query<Entity, With<Selected>>,
) {
    // Only process clicks
    if !mouse_button_input.just_pressed(MouseButton::Left) {
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
            let world_position = world_position.origin.truncate();
            
            // Clear previous selections
            for entity in selected_query.iter() {
                commands.entity(entity).remove::<Selected>();
            }

            // Check if we clicked on a selectable entity
            for (entity, transform, sprite) in selectable_query.iter() {
                // Simple AABB collision detection
                let sprite_size = sprite.custom_size.unwrap_or(Vec2::splat(100.0));
                let min_x = transform.translation.x - sprite_size.x / 2.0;
                let max_x = transform.translation.x + sprite_size.x / 2.0;
                let min_y = transform.translation.y - sprite_size.y / 2.0;
                let max_y = transform.translation.y + sprite_size.y / 2.0;
                
                if world_position.x >= min_x && world_position.x <= max_x &&
                   world_position.y >= min_y && world_position.y <= max_y {
                    commands.entity(entity).insert(Selected);
                    break;
                }
            }
        }
    }
}

pub fn highlight_selected(
    mut query: Query<(&mut Sprite, Option<&Selected>), With<Selectable>>,
) {
    for (mut sprite, selected) in query.iter_mut() {
        if selected.is_some() {
            // Highlight selected units
            sprite.color = Color::rgba(1.0, 1.0, 1.0, 1.0);
        } else {
            // Normal color for unselected units
            sprite.color = Color::rgba(0.8, 0.8, 0.8, 1.0);
        }
    }
}
