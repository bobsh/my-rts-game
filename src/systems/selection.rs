use bevy::prelude::*;
use bevy::input::mouse::MouseButton;
use bevy::window::PrimaryWindow;
use crate::components::unit::{Selectable, Selected, SelectionRing};

pub fn selection_system(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    camera_query: Query<(&Camera, &GlobalTransform)>,
    mouse_button_input: Res<Input<MouseButton>>,
    selectable_query: Query<(Entity, &Transform), With<Selectable>>,
    mut selected_query: Query<Entity, With<Selected>>,
    selection_ring_query: Query<Entity, With<SelectionRing>>,
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
            
            // Remove selection rings
            for entity in selection_ring_query.iter() {
                commands.entity(entity).despawn();
            }
            
            // Clear previous selections
            for entity in selected_query.iter() {
                commands.entity(entity).remove::<Selected>();
            }

            // Check if we clicked on a selectable entity
            for (entity, transform) in selectable_query.iter() {
                // Simple AABB collision detection - assuming 50x50 size for sprites
                let sprite_size = Vec2::new(64.0, 64.0); // Adjust as needed for your sprite
                let min_x = transform.translation.x - sprite_size.x / 2.0;
                let max_x = transform.translation.x + sprite_size.x / 2.0;
                let min_y = transform.translation.y - sprite_size.y / 2.0;
                let max_y = transform.translation.y + sprite_size.y / 2.0;
                
                if world_position.x >= min_x && world_position.x <= max_x &&
                   world_position.y >= min_y && world_position.y <= max_y {
                    // Add Selected component
                    commands.entity(entity).insert(Selected);
                    
                    // Create selection ring
                    commands.spawn((
                        SpriteBundle {
                            sprite: Sprite {
                                color: Color::rgba(0.2, 1.0, 0.2, 0.5),
                                custom_size: Some(Vec2::new(70.0, 70.0)),
                                ..default()
                            },
                            transform: Transform::from_translation(Vec3::new(
                                transform.translation.x,
                                transform.translation.y,
                                transform.translation.z - 0.1
                            )),
                            ..default()
                        },
                        SelectionRing,
                    ));
                    
                    break;
                }
            }
        }
    }
}

pub fn highlight_selected(
    query: Query<(&Transform, Option<&Selected>), With<Selectable>>,
) {
    for (transform, selected) in query.iter() {
        // Instead of modifying sprites directly (which won't work well with image textures),
        // we rely on the selection ring spawned in the selection_system
        // and the animation system to provide visual feedback
        if selected.is_some() {
            // Selected units are highlighted by the selection ring
        }
    }
}
