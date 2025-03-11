use crate::components::unit::Selectable;
use crate::components::unit::Unit;
use crate::components::unit::UnitAttributes;
use crate::components::unit::Velocity;
use crate::components::unit::WorkerAnimation;
use crate::components::unit::WorkerAnimationState;
use crate::resources::{ResourceId, ResourceRegistry};
use bevy::prelude::*;

use crate::components::inventory::Inventory;
use crate::systems::map::spawn_resource_node;

pub fn setup_scene(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    resource_registry: Res<ResourceRegistry>,
) {
    commands.spawn(Camera2d);

    // Spawn worker units
    spawn_worker(
        &mut commands,
        &asset_server,
        Vec2::new(-200.0, 0.0),
        "units/jungleman/jungleman.png".to_string(),
        1,
    );
    spawn_worker(
        &mut commands,
        &asset_server,
        Vec2::new(0.0, 0.0),
        "units/jungleman/jungleman.png".to_string(),
        2,
    );
    spawn_worker(
        &mut commands,
        &asset_server,
        Vec2::new(200.0, 0.0),
        "units/jungleman/jungleman.png".to_string(),
        3,
    );

    // Spawn resource nodes using the registry
    let gold_id = ResourceId("gold".to_string());
    let wood_id = ResourceId("wood".to_string());
    let stone_id = ResourceId("stone".to_string());

    // Increased spacing between resource nodes
    spawn_resource_node(
        &mut commands,
        &asset_server,
        &resource_registry,
        Vec2::new(-300.0, 200.0),
        &gold_id,
        100,
    );
    spawn_resource_node(
        &mut commands,
        &asset_server,
        &resource_registry,
        Vec2::new(0.0, 200.0),
        &wood_id,
        150,
    );
    spawn_resource_node(
        &mut commands,
        &asset_server,
        &resource_registry,
        Vec2::new(300.0, 200.0),
        &stone_id,
        125,
    );

    // Additional resource nodes in different locations
    spawn_resource_node(
        &mut commands,
        &asset_server,
        &resource_registry,
        Vec2::new(-200.0, -150.0),
        &gold_id,
        75,
    );
    spawn_resource_node(
        &mut commands,
        &asset_server,
        &resource_registry,
        Vec2::new(150.0, -200.0),
        &wood_id,
        100,
    );
    spawn_resource_node(
        &mut commands,
        &asset_server,
        &resource_registry,
        Vec2::new(-100.0, -250.0),
        &stone_id,
        80,
    );
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
