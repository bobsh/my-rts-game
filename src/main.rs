use bevy::app::App;
use bevy::prelude::{PluginGroup, Window};
use bevy::window::WindowPlugin;
use bevy::DefaultPlugins;

mod components;
mod plugin;
mod systems;

use plugin::RtsPlugin;

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
        .add_plugins(RtsPlugin)
        .run();
}
