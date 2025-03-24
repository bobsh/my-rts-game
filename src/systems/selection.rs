use crate::components::unit::{Selectable, Selected, SelectionRing, Unit};
use bevy::input::mouse::MouseButton;
use bevy::input::ButtonInput;
use bevy::prelude::*;
use bevy::sprite::Sprite;
use bevy::window::PrimaryWindow;

pub struct SelectionPlugin;

impl Plugin for SelectionPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                selection_system,
                update_selection_ring,
                highlight_selected,
                draw_selection_boxes,
            ),
        );
    }
}

#[allow(clippy::too_many_arguments)]
fn selection_system(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    camera_query: Query<(&Camera, &GlobalTransform)>,
    mouse_button_input: Res<ButtonInput<MouseButton>>,
    selectable_query: Query<(Entity, &Transform, &Sprite), With<Selectable>>,
    selected_query: Query<Entity, With<Selected>>,
    selection_ring_query: Query<Entity, With<SelectionRing>>,
    _asset_server: Res<AssetServer>,
    images: Res<Assets<Image>>,
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
        if let Ok(world_position) = camera.viewport_to_world(camera_transform, cursor_position) {
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
            for (entity, transform, sprite) in selectable_query.iter() {
                // Get entity size from LDTK or sprite
                let entity_size = get_entity_size(sprite, &images);

                // Simple AABB collision detection with dynamic size
                let min_x = transform.translation.x - entity_size.x / 2.0;
                let max_x = transform.translation.x + entity_size.x / 2.0;
                let min_y = transform.translation.y - entity_size.y / 2.0;
                let max_y = transform.translation.y + entity_size.y / 2.0;

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

fn get_entity_size(sprite: &Sprite, images: &Res<Assets<Image>>) -> Vec2 {
    // First priority: Use custom_size if available (explicitly set size)
    if let Some(custom_size) = sprite.custom_size {
        return custom_size;
    }

    // Second priority: Get size from the actual image asset
    if let Some(image) = images.get(&sprite.image) {
        return Vec2::new(
            image.texture_descriptor.size.width as f32,
            image.texture_descriptor.size.height as f32,
        );
    }

    // Default fallback
    Vec2::new(64.0, 64.0)
}

// Fix the update_selection_ring function
// Complexity:
#[allow(clippy::type_complexity)]
fn update_selection_ring(
    mut params: ParamSet<(
        Query<(&SelectionRing, &mut Transform)>,
        Query<(Entity, &Transform), With<Unit>>,
    )>,
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

fn highlight_selected(query: Query<(&Transform, Option<&Selected>), With<Selectable>>) {
    for (_transform, selected) in query.iter() {
        if selected.is_some() {
            // Selected units are highlighted by the selection ring
            // This system is now mostly redundant with the animated ring
        }
    }
}

fn draw_selection_boxes(
    mut gizmos: Gizmos,
    selection_query: Query<(&Transform, &Sprite), With<Selected>>,
    images: Res<Assets<Image>>,
) {
    for (transform, sprite) in selection_query.iter() {
        // Get position from transform
        let position = transform.translation.truncate();

        // Get entity size
        let entity_size = get_entity_size(sprite, &images);

        // Make selection box slightly larger than the entity
        let box_size = entity_size + Vec2::new(6.0, 6.0);

        // In Bevy 0.15, rect_2d takes position, size, color
        gizmos.rect_2d(
            position,
            box_size,
            Color::srgb(0.0, 1.0, 0.0), // Green color
        );
    }
}
