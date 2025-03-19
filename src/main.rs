use bevy::app::App;
use bevy::prelude::{PluginGroup, Window};
use bevy::window::WindowPlugin;
use bevy::DefaultPlugins;

mod components;
mod entities;
mod plugin;
mod resources;
mod systems;

use plugin::RtsPlugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "My RTS Game".to_string(),
                resolution: (1280.0, 720.0).into(),
                ..Default::default()
            }),
            ..Default::default()
        }))
        .add_plugins(RtsPlugin)
        .run();
}
