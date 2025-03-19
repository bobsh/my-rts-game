use crate::components::unit::Selectable;
use crate::components::unit::Unit;
use crate::components::unit::UnitAttributes;
use crate::components::unit::Velocity;
use crate::components::unit::WorkerAnimation;
use crate::components::unit::WorkerAnimationState;
use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;

use crate::components::inventory::Inventory;

pub fn setup_scene(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((
        Camera2d,
        Transform::from_xyz(1280.0, 720.0, 0.0),
    ));

    // Load the ldtk map file
    let map_handle = asset_server.load("test-map.ldtk");

    commands.spawn(LdtkWorldBundle {
        ldtk_handle: map_handle.into(),
        ..Default::default()
    });
}

// Update worker spawning with the new animation component
fn spawn_worker(
    commands: &mut Commands,
    asset_server: &Res<AssetServer>,
    position: Vec2,
    texture_path: String,
    i: usize,
) {
    let texture = asset_server.load(&texture_path);

    let _worker_entity = commands
        .spawn((
            Transform::from_translation(Vec3::new(position.x, position.y, 0.0))
                .with_scale(Vec3::new(0.8, 0.8, 1.0)),
            Sprite {
                image: texture,
                ..Default::default()
            },
            Name::new(format!("Worker {i}")),
            Unit,
            Selectable,
            WorkerAnimation {
                timer: Timer::from_seconds(2.0, TimerMode::Repeating),
                state: WorkerAnimationState::Idle, // Initialize with idle state
            },
            Velocity {
                value: Vec2::ZERO,
                target: None,
                speed: 100.0,
            },
            UnitAttributes {
                name: format!("Worker {i}"),
                health: 100.0,
                max_health: 100.0,
            },
            Inventory::new(20), // Each worker can carry 20 resource units
        ))
        .id();
}
