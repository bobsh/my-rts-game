use bevy::prelude::*;
use bevy::window::WindowPlugin;

mod assets;
mod components;
mod resources;
mod systems;

use components::unit::{Selectable, Unit, WorkerAnimation, Velocity, MoveMarker};
use systems::selection::{selection_system, highlight_selected};
use systems::animation::animate_workers;
use systems::movement::{move_command_system, movement_system, show_destination_markers};

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
        .add_systems(Update, (
            selection_system, 
            highlight_selected, 
            animate_workers,
            move_command_system,
            movement_system,
            show_destination_markers,
        ))
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

    // Spawn worker units with different textures
    spawn_worker(&mut commands, &asset_server, Vec2::new(-200.0, 0.0), "units/worker.png");
    spawn_worker(&mut commands, &asset_server, Vec2::new(0.0, 0.0), "units/worker.png");
    spawn_worker(&mut commands, &asset_server, Vec2::new(200.0, 0.0), "units/worker.png");
}

fn spawn_worker(commands: &mut Commands, asset_server: &Res<AssetServer>, position: Vec2, texture_path: &str) {
    let texture = asset_server.load(texture_path);
    
    commands.spawn((
        SpriteBundle {
            texture,
            transform: Transform::from_translation(Vec3::new(position.x, position.y, 0.0))
                .with_scale(Vec3::new(0.8, 0.8, 1.0)), // Scale as needed for your sprite size
            ..Default::default()
        },
        Unit,
        Selectable,
        WorkerAnimation {
            timer: Timer::from_seconds(2.0, TimerMode::Repeating),
        },
        Velocity {
            value: Vec2::ZERO,
            target: None,
            speed: 100.0,
        },
    ));
}

// Keep the old spawn_unit function if you need colored units later
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
