use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;

pub struct LdtkCalibrationPlugin;

#[derive(Resource)]
pub struct LdtkCalibration {
    pub offset: Vec2,
}

impl Default for LdtkCalibration {
    fn default() -> Self {
        Self {
            offset: Vec2::ZERO,
        }
    }
}

impl Plugin for LdtkCalibrationPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<LdtkCalibration>()
           .add_systems(Update, adjust_ldtk_offset)
           .add_systems(Update, apply_ldtk_calibration);
    }
}

// System to adjust the calibration with keyboard input
fn adjust_ldtk_offset(
    keys: Res<ButtonInput<KeyCode>>,
    mut calibration: ResMut<LdtkCalibration>
) {
    // Only active when Shift key held
    if keys.pressed(KeyCode::ShiftLeft) {
        if keys.pressed(KeyCode::KeyO) { // O for "Offset"
            if keys.pressed(KeyCode::ArrowLeft) {
                calibration.offset.x -= 1.0;
                info!("LDtk Calibration X: {}, Y: {}", calibration.offset.x, calibration.offset.y);
            }
            if keys.pressed(KeyCode::ArrowRight) {
                calibration.offset.x += 1.0;
                info!("LDtk Calibration X: {}, Y: {}", calibration.offset.x, calibration.offset.y);
            }
            if keys.pressed(KeyCode::ArrowUp) {
                calibration.offset.y += 1.0;
                info!("LDtk Calibration X: {}, Y: {}", calibration.offset.x, calibration.offset.y);
            }
            if keys.pressed(KeyCode::ArrowDown) {
                calibration.offset.y -= 1.0;
                info!("LDtk Calibration X: {}, Y: {}", calibration.offset.x, calibration.offset.y);
            }
            // Reset to zero with R
            if keys.just_pressed(KeyCode::KeyR) {
                calibration.offset = Vec2::ZERO;
                info!("LDtk Calibration reset to zero ");
            }
        }
    }
}

// Apply the calibration to all LDtk worlds
fn apply_ldtk_calibration(
    calibration: Res<LdtkCalibration>,
    mut ldtk_worlds: Query<&mut Transform, With<LdtkProjectHandle>>,
) {
    if calibration.is_changed() {
        for mut transform in &mut ldtk_worlds {
            transform.translation.x = calibration.offset.x;
            transform.translation.y = calibration.offset.y;
        }
    }
}
