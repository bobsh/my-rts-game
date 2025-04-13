use crate::components::unit::{Selectable, Selected, SelectionRing, Unit};
use bevy::input::mouse::MouseButton;
use bevy::input::ButtonInput;
use bevy::prelude::*;
use bevy::sprite::Sprite;
use bevy::window::PrimaryWindow;

pub struct SelectionPlugin;

impl Plugin for SelectionPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, selection_system)
            .add_systems(Update, update_selection_ring)
            .add_systems(Update, draw_selection_boxes);
    }
}

#[allow(clippy::too_many_arguments)]
fn selection_system(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    camera_query: Query<(&Camera, &GlobalTransform)>,
    mouse_button_input: Res<ButtonInput<MouseButton>>,
    selectable_query: Query<(Entity, &GlobalTransform, &Sprite), With<Selectable>>,
    selected_query: Query<Entity, With<Selected>>,
    selection_ring_query: Query<Entity, With<SelectionRing>>,
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
                // Get entity size from sprite
                let entity_size = get_entity_size(sprite, entity, &images);

                // Simple AABB collision detection with dynamic size
                let min_x = transform.translation().x - entity_size.x / 2.0;
                let max_x = transform.translation().x + entity_size.x / 2.0;
                let min_y = transform.translation().y - entity_size.y / 2.0;
                let max_y = transform.translation().y + entity_size.y / 2.0;

                if world_position.x >= min_x
                    && world_position.x <= max_x
                    && world_position.y >= min_y
                    && world_position.y <= max_y
                {
                    // Only log significant events
                    info!("Selected entity: {:?}", entity);

                    // Add Selected component
                    commands.entity(entity).insert(Selected);
                    break;
                }
            }
        }
    }
}

fn get_entity_size(sprite: &Sprite, _entity: Entity, images: &Res<Assets<Image>>) -> Vec2 {
    // First priority: Use custom_size if available (explicitly set size)
    if let Some(custom_size) = sprite.custom_size {
        return custom_size;
    }

    // Return the size stored in the entity
    // return Vec2::new(
    //     entity.height as f32,
    //     entity.width as f32,
    // );

    // Second priority: Get size from the actual image asset
    if let Some(image) = images.get(&sprite.image) {
        return Vec2::new(
            image.texture_descriptor.size.width as f32,
            image.texture_descriptor.size.height as f32,
        );
    }

    // Default fallback (64x64 is typical for tiles)
    Vec2::new(64.0, 64.0)
}

// Update selection ring function
#[allow(clippy::type_complexity)]
fn update_selection_ring(
    mut params: ParamSet<(
        Query<(&SelectionRing, &mut Transform)>,
        Query<(Entity, &GlobalTransform), With<Unit>>,
    )>,
) {
    // First, collect the positions we need
    let mut unit_positions: Vec<(Entity, Vec3)> = Vec::new();

    // Get all unit positions
    for (entity, transform) in params.p1().iter() {
        unit_positions.push((entity, transform.translation()));
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

fn draw_selection_boxes(
    mut gizmos: Gizmos,
    selection_query: Query<(Entity, &GlobalTransform, &Sprite), With<Selected>>,
    images: Res<Assets<Image>>,
) {
    for (entity, transform, sprite) in selection_query.iter() {
        // Get position from transform
        let position = transform.translation();

        // Get entity size
        let entity_size = get_entity_size(sprite, entity, &images);

        // Make selection box slightly larger than the entity
        let box_size = entity_size + Vec2::new(6.0, 6.0);

        // In Bevy 0.15, rect takes:
        // 1. Position+Rotation (as Vec3 or Transform)
        // 2. Size (Vec2)
        // 3. Color
        gizmos.rect(
            position,                   // Vec3 for position
            box_size,                   // Vec2 for size
            Color::srgb(0.0, 1.0, 0.0), // Green color
        );
    }
}
