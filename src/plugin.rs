use bevy::input::common_conditions::input_toggle_active;
use bevy::app::{App, Plugin};
use bevy::prelude::KeyCode;
use bevy_ecs_ldtk::prelude::*;
use bevy_inspector_egui::quick::WorldInspectorPlugin;

use crate::entities::EntitiesPlugin;
use crate::systems::camera::CameraPlugin;
use crate::systems::movement::MovementPlugin;
use crate::systems::scene::ScenePlugin;
use crate::systems::selection::SelectionPlugin;
use crate::systems::ui::UiPlugin;
use crate::systems::window::WindowPlugin;

pub struct RtsPlugin;

impl Plugin for RtsPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(LdtkPlugin)
            .add_plugins(WorldInspectorPlugin::new().run_if(input_toggle_active(false, KeyCode::F10)))
            .add_plugins(EntitiesPlugin)
            .add_plugins(MovementPlugin)
            .add_plugins(CameraPlugin)
            .add_plugins(WindowPlugin)
            .add_plugins(SelectionPlugin)
            .add_plugins(ScenePlugin)
            .add_plugins(UiPlugin);
    }
}
