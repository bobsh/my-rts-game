use bevy::prelude::*;
use bevy::input::mouse::{MouseButton, MouseMotion};

#[derive(Default, Resource)]
pub struct CameraPanState {
    is_panning: bool,
}

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<CameraPanState>()
           .add_systems(Update, camera_pan);
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
