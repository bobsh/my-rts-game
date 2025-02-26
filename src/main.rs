use bevy::prelude::*;
use bevy::window::WindowPlugin;

mod assets;
mod components;
mod resources;
mod systems;

use components::unit::{Selectable, Selected, Unit};
use systems::selection::{selection_system, highlight_selected};

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
        .add_systems(Update, (selection_system, highlight_selected))
        .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2dBundle::default());

    // Load the font
    let font_handle = asset_server.load("fonts/fira_sans/FiraSans-Bold.ttf");

    // Spawn the text
    commands.spawn(Text2dBundle {
        text: Text::from_section(
            "Click on units to select them",
            TextStyle {
                font: font_handle,
                font_size: 30.0,
                color: Color::WHITE,
            },
        )
        .with_alignment(TextAlignment::Center),
        transform: Transform::from_translation(Vec3::new(0.0, 300.0, 0.0)),
        ..Default::default()
    });

    // Spawn some units
    spawn_unit(&mut commands, Vec2::new(-200.0, 0.0), Color::RED);
    spawn_unit(&mut commands, Vec2::new(0.0, 0.0), Color::GREEN);
    spawn_unit(&mut commands, Vec2::new(200.0, 0.0), Color::BLUE);
}

fn spawn_unit(commands: &mut Commands, position: Vec2, color: Color) {
    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                color,
                custom_size: Some(Vec2::new(50.0, 50.0)),
                ..Default::default()
            },
            transform: Transform::from_translation(Vec3::new(position.x, position.y, 0.0)),
            ..Default::default()
        },
        Unit,
        Selectable,
    ));
}
