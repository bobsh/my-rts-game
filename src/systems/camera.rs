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
    // Base zoom factor (for native builds)
    let base_zoom_factor = 0.1;

    // Get the current platform
    let is_wasm = cfg!(target_arch = "wasm32");

    // Much more aggressive reduction for WASM
    let zoom_factor = if is_wasm {
        // Try an extremely small value for WASM
        0.001 // 1/1000th of the original sensitivity
    } else {
        base_zoom_factor
    };

    let min_zoom = 0.25;
    let max_zoom = 2.0;

    for event in mouse_wheel_events.read() {
        // Apply even more smoothing for WASM
        let mut zoom_amount = event.y * zoom_factor;

        // Further smooth the zoom on WASM by clamping large deltas
        if is_wasm && zoom_amount.abs() > 0.005 {
            zoom_amount = zoom_amount.signum() * 0.005;
        }

        // Apply zoom
        camera_state.zoom_level = (camera_state.zoom_level + zoom_amount).clamp(min_zoom, max_zoom);

        // Apply the zoom to the camera
        for mut transform in camera_query.iter_mut() {
            transform.scale = Vec3::splat(1.0 / camera_state.zoom_level);
        }
    }
}
