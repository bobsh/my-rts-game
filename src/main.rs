use bevy::prelude::*;
use bevy::window::WindowPlugin;

mod assets;
mod components;
mod resources;
mod systems;

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
        .add_systems(Startup, setup)
        .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2dBundle::default());

    // Load the font
    let font_handle = asset_server.load("fonts/fira_sans/FiraSans-Bold.ttf");

    // Spawn the text
    commands.spawn(Text2dBundle {
        text: Text::from_section(
            "Hello, World! This is a test.",
            TextStyle {
                font: font_handle,
                font_size: 50.0,
                color: Color::WHITE,
            },
        )
        .with_alignment(TextAlignment::Center),
        ..Default::default()
    });
}
