use bevy::app::App;
use bevy::prelude::{PluginGroup, Window};
use bevy::window::WindowPlugin;
use bevy::DefaultPlugins;
use bevy::ecs::system::NonSend;

mod components;
mod entities;
mod plugin;
mod systems;

use plugin::RtsPlugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "RTS Game".to_string(),
                // Add window icon silently without logging
                fit_canvas_to_parent: true,
                canvas: Some("#bevy".to_string()),
                ..Default::default()
            }),
            ..Default::default()
        }))
        .add_plugins(RtsPlugin)
        .run();
}

#[allow(dead_code)]
fn set_window_icon(windows: NonSend<bevy::winit::WinitWindows>) {
    // If you were setting a custom icon, move that logic here
    // but don't print anything to the console
}
