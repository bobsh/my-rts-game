use bevy::app::{App, Plugin};
use bevy::input::common_conditions::input_toggle_active;
use bevy::prelude::KeyCode;
use bevy_ecs_ldtk::prelude::*;
use bevy_inspector_egui::quick::WorldInspectorPlugin;

use crate::components::entities::EntitiesPlugin;
use crate::systems::camera::CameraPlugin;
use crate::systems::construction::ConstructionPlugin;
use crate::systems::inventory::InventoryPlugin;
use crate::systems::ldtk_calibration::LdtkCalibrationPlugin;
use crate::systems::movement::MovementPlugin;
use crate::systems::resource_gathering::ResourceGatheringPlugin;
use crate::systems::scene::ScenePlugin;
use crate::systems::selection::SelectionPlugin;
use crate::systems::ui::UiPlugin;
use crate::systems::window::WindowPlugin; // Add this import

pub struct RtsPlugin;

impl Plugin for RtsPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(LdtkPlugin)
            .add_plugins(
                WorldInspectorPlugin::new().run_if(input_toggle_active(false, KeyCode::F10)),
            )
            .add_plugins(EntitiesPlugin)
            .add_plugins(LdtkCalibrationPlugin) // Add this plugin first so its transforms take effect
            .add_plugins(MovementPlugin)
            .add_plugins(ResourceGatheringPlugin)
            .add_plugins(ConstructionPlugin)
            .add_plugins(InventoryPlugin)
            .add_plugins(CameraPlugin)
            .add_plugins(WindowPlugin)
            .add_plugins(SelectionPlugin)
            .add_plugins(ScenePlugin)
            .add_plugins(UiPlugin);
    }
}
