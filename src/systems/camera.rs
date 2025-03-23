use bevy::input::mouse::{MouseButton, MouseMotion, MouseWheel};
use bevy::prelude::*;

#[derive(Default, Resource)]
pub struct CameraPanState {
    is_panning: bool,
    zoom_level: f32,
}

impl CameraPanState {
    fn new() -> Self {
        Self {
            is_panning: false,
            zoom_level: 1.0, // Default zoom level
        }
    }
}

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(CameraPanState::new())
            .add_systems(Update, (camera_pan, camera_zoom));
    }
}

fn camera_pan(
    mut camera_pan_state: ResMut<CameraPanState>,
    mut camera_query: Query<&mut Transform, With<Camera>>,
    mouse_button_input: Res<ButtonInput<MouseButton>>,
    mut mouse_motion_events: EventReader<MouseMotion>,
) {
    // Start panning when middle mouse button is pressed
    if mouse_button_input.just_pressed(MouseButton::Middle) {
        camera_pan_state.is_panning = true;
    }

    // Stop panning when middle mouse button is released
    if mouse_button_input.just_released(MouseButton::Middle) {
        camera_pan_state.is_panning = false;
    }

    // If we're panning, move the camera based on mouse motion
    if camera_pan_state.is_panning {
        let mut total_delta = Vec2::ZERO;
        for event in mouse_motion_events.read() {
            total_delta += event.delta;
        }

        if total_delta != Vec2::ZERO {
            for mut transform in camera_query.iter_mut() {
                // Increased the speed multiplier from 0.5 to 1.5 for more responsive dragging
                // Adjust this value to your preference - higher means faster camera movement
                transform.translation.x -= total_delta.x * 1.5;
                transform.translation.y += total_delta.y * 1.5;
            }
        }
    }
}

fn camera_zoom(
    mut camera_state: ResMut<CameraPanState>,
    mut camera_query: Query<&mut Transform, With<Camera>>,
    mut mouse_wheel_events: EventReader<MouseWheel>,
) {
    // Base zoom factor
    let base_zoom_factor = 0.1;

    // Platform-specific adjustments
    #[cfg(target_arch = "wasm32")]
    let zoom_factor = base_zoom_factor * 0.05; // Much lower sensitivity for web (5% of normal)

    #[cfg(not(target_arch = "wasm32"))]
    let zoom_factor = base_zoom_factor; // Normal sensitivity for native builds

    let min_zoom = 0.25; // Maximum zoom out (25% of original size)
    let max_zoom = 2.0; // Maximum zoom in (200% of original size)

    for event in mouse_wheel_events.read() {
        // Apply platform-specific zoom factor
        let zoom_amount = event.y * zoom_factor;

        // Make zoom more gradual
        let old_zoom = camera_state.zoom_level;
        let new_zoom = (old_zoom + zoom_amount).clamp(min_zoom, max_zoom);
        camera_state.zoom_level = new_zoom;

        // Apply the zoom to the camera
        for mut transform in camera_query.iter_mut() {
            transform.scale = Vec3::splat(1.0 / camera_state.zoom_level);
        }
    }
}
