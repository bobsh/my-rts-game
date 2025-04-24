use bevy::app::App;
use bevy::input::common_conditions::input_toggle_active;
use bevy::prelude::{KeyCode, PluginGroup, Window};
use bevy::window::WindowPlugin;
use bevy::DefaultPlugins;
use bevy_aseprite_ultra::prelude::*;
use bevy_ecs_ldtk::prelude::*;
use bevy_inspector_egui::quick::WorldInspectorPlugin;

mod components;
mod systems;

use crate::components::entities::EntitiesPlugin;
use crate::systems::audio::AudioSystemPlugin;
use crate::systems::camera::CameraPlugin;
use crate::systems::construction::ConstructionPlugin;
use crate::systems::inventory::InventoryPlugin;
use crate::systems::movement::MovementPlugin;
use crate::systems::resource_gathering::ResourceGatheringPlugin;
use crate::systems::scene::ScenePlugin;
use crate::systems::selection::SelectionPlugin;
use crate::systems::setup_window::SetupWindowPlugin;
use crate::systems::ui::UiPlugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "RTS Game".to_string(),
                fit_canvas_to_parent: true,
                canvas: Some("#bevy".to_string()),
                ..Default::default()
            }),
            ..Default::default()
        }))
        .add_plugins(LdtkPlugin)
        .add_plugins(WorldInspectorPlugin::new().run_if(input_toggle_active(false, KeyCode::F10)))
        .add_plugins(AsepriteUltraPlugin)
        .add_plugins(EntitiesPlugin)
        .add_plugins(MovementPlugin)
        .add_plugins(ResourceGatheringPlugin)
        .add_plugins(ConstructionPlugin)
        .add_plugins(InventoryPlugin)
        .add_plugins(CameraPlugin)
        .add_plugins(SetupWindowPlugin)
        .add_plugins(SelectionPlugin)
        .add_plugins(ScenePlugin)
        .add_plugins(UiPlugin)
        .add_plugins(AudioSystemPlugin)
        .run();
}
