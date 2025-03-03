use crate::components::unit::{Selectable, Selected, SelectionRing, Unit};
use bevy::input::mouse::MouseButton;
use bevy::input::ButtonInput;
use bevy::prelude::*;
use bevy::window::PrimaryWindow; // Added Unit here

pub fn selection_system(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    camera_query: Query<(&Camera, &GlobalTransform)>,
    mouse_button_input: Res<ButtonInput<MouseButton>>,
    selectable_query: Query<(Entity, &Transform), With<Selectable>>,
    selected_query: Query<Entity, With<Selected>>,
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
                // Simple AABB collision detection - assuming 64x64 size for sprites
                let sprite_size = Vec2::new(64.0, 64.0); // Adjust as needed for your sprite
                let min_x = transform.translation.x - sprite_size.x / 2.0;
                let max_x = transform.translation.x + sprite_size.x / 2.0;
                let min_y = transform.translation.y - sprite_size.y / 2.0;
                let max_y = transform.translation.y + sprite_size.y / 2.0;

                if world_position.x >= min_x
                    && world_position.x <= max_x
                    && world_position.y >= min_y
                    && world_position.y <= max_y
                {
                    // Add Selected component
                    commands.entity(entity).insert(Selected);

                    break;
                }
            }
        }
    }
}

// Add a new system for animating the selection ring
pub fn animate_selection_rings(
    time: Res<Time>,
    mut query: Query<(&mut SelectionRing, &mut Sprite)>,
) {
    for (mut ring, mut sprite) in &mut query {
        ring.timer.tick(time.delta());

        // Calculate a pulsing effect
        let pulse_factor = (ring.timer.fraction() * std::f32::consts::PI * 2.0).sin().mul_add(0.1, 1.0);
        let current_size = ring.base_size * pulse_factor;

        // Update sprite size
        sprite.custom_size = Some(Vec2::new(current_size, current_size));

        // Also pulse the opacity
        let alpha = (ring.timer.fraction() * std::f32::consts::PI * 2.0).cos().mul_add(0.2, 0.4);
        sprite.color = sprite.color.with_alpha(alpha);
    }
}

// Fix the update_selection_ring_positions system using ParamSet
#[allow(clippy::type_complexity)]
pub fn update_selection_ring(
    // Function parameters including your ParamSet
    mut params: ParamSet<(
        Query<(&SelectionRing, &mut Transform)>,
        Query<(Entity, &Transform), With<Unit>>,
    )>,
    // ... rest of function parameters
) {
    // First, collect the positions we need
    let mut unit_positions: Vec<(Entity, Vec3)> = Vec::new();

    // Get all unit positions
    for (entity, transform) in params.p1().iter() {
        unit_positions.push((entity, transform.translation));
    }

    // Update ring positions based on collected data
    let mut ring_query = params.p0();
    for (ring, mut ring_transform) in &mut ring_query {
        // Find the matching unit
        for (entity, position) in &unit_positions {
            if ring.owner == *entity {
                // Update ring position to match unit, but keep it slightly below (z-axis)
                ring_transform.translation.x = position.x;
                ring_transform.translation.y = position.y;
                ring_transform.translation.z = position.z - 0.1;
                break;
            }
        }
    }
}

pub fn highlight_selected(query: Query<(&Transform, Option<&Selected>), With<Selectable>>) {
    for (_transform, selected) in query.iter() {
        if selected.is_some() {
            // Selected units are highlighted by the selection ring
            // This system is now mostly redundant with the animated ring
        }
    }
}

pub fn draw_selection_boxes(
    mut gizmos: Gizmos,
    selection_query: Query<&Transform, With<Selected>>,
) {
    for transform in selection_query.iter() {
        // Get position from transform
        let position = transform.translation.truncate();

        // Use a consistent size for selection boxes (adjust as needed)
        let size = Vec2::new(70.0, 70.0);

        // Draw just the outline in green (no fill)
        gizmos.rect_2d(
            position,
            0.0, // No rotation
            size,
            Color::srgb(0.0, 1.0, 0.0), // Green
        );
    }
}
