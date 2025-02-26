use bevy::prelude::*;
use bevy::window::WindowPlugin;

mod assets;
mod components;
mod resources;
mod systems;

use components::unit::{Selectable, Unit, WorkerAnimation, WorkerAnimationState, Velocity, UnitAttributes};
use components::resource::ResourceNode; // This should work now
use resources::{PlayerResources, ResourceRegistry, ResourceId, GameState};
use systems::selection::{selection_system, highlight_selected, animate_selection_rings, update_selection_ring_positions};
use systems::animation::{animate_workers, update_worker_animations, animate_gather_effects, animate_floating_text};
use systems::movement::{move_command_system, movement_system, show_destination_markers};
use systems::gathering::{resource_gathering_command, gathering_system};
use systems::ui::{setup_ui, update_unit_info, update_resources_display};

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
        .init_resource::<GameState>()
        .init_resource::<PlayerResources>()
        .init_resource::<ResourceRegistry>() // Initialize resource registry
        .add_systems(Startup, (setup, setup_ui))
        .add_systems(Update, (
            selection_system, 
            highlight_selected,
            animate_selection_rings,
            update_selection_ring_positions,
            animate_workers,
            move_command_system,
            movement_system,
            show_destination_markers,
            update_unit_info,
            resource_gathering_command,
            gathering_system,
            update_resources_display,
            update_worker_animations,  // Add this
            animate_gather_effects,    // Add this
            animate_floating_text,     // Add this
        ))
        .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>, resource_registry: Res<ResourceRegistry>) {
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

    // Spawn worker units
    spawn_worker(&mut commands, &asset_server, Vec2::new(-200.0, 0.0), "units/worker.png");
    spawn_worker(&mut commands, &asset_server, Vec2::new(0.0, 0.0), "units/worker.png");
    spawn_worker(&mut commands, &asset_server, Vec2::new(200.0, 0.0), "units/worker.png");

    // Spawn resource nodes using the registry
    let gold_id = ResourceId("gold".to_string());
    let wood_id = ResourceId("wood".to_string());
    let stone_id = ResourceId("stone".to_string());

    // Increased spacing between resource nodes
    spawn_resource_node(&mut commands, &asset_server, &resource_registry, Vec2::new(-300.0, 200.0), &gold_id, 100);
    spawn_resource_node(&mut commands, &asset_server, &resource_registry, Vec2::new(0.0, 200.0), &wood_id, 150);
    spawn_resource_node(&mut commands, &asset_server, &resource_registry, Vec2::new(300.0, 200.0), &stone_id, 125);
    
    // Additional resource nodes in different locations
    spawn_resource_node(&mut commands, &asset_server, &resource_registry, Vec2::new(-200.0, -150.0), &gold_id, 75);
    spawn_resource_node(&mut commands, &asset_server, &resource_registry, Vec2::new(150.0, -200.0), &wood_id, 100);
    spawn_resource_node(&mut commands, &asset_server, &resource_registry, Vec2::new(-100.0, -250.0), &stone_id, 80);
}

// Update worker spawning with the new animation component
fn spawn_worker(commands: &mut Commands, asset_server: &Res<AssetServer>, position: Vec2, texture_path: &str) {
    let texture = asset_server.load(texture_path);
    
    commands.spawn((
        SpriteBundle {
            texture,
            transform: Transform::from_translation(Vec3::new(position.x, position.y, 0.0))
                .with_scale(Vec3::new(0.8, 0.8, 1.0)),
            ..Default::default()
        },
        Unit,
        Selectable,
        WorkerAnimation {
            timer: Timer::from_seconds(2.0, TimerMode::Repeating),
            state: WorkerAnimationState::Idle,  // Initialize with idle state
        },
        Velocity {
            value: Vec2::ZERO,
            target: None,
            speed: 100.0,
        },
        UnitAttributes {
            name: "Worker".to_string(),
            health: 100.0,
            max_health: 100.0,
        },
    ));
}

// Updated function to spawn resource nodes
fn spawn_resource_node(
    commands: &mut Commands, 
    asset_server: &Res<AssetServer>,
    resource_registry: &Res<ResourceRegistry>,
    position: Vec2,
    resource_id: &ResourceId,
    amount: u32
) {
    // Get the resource definition from registry
    if let Some(resource_def) = resource_registry.get(resource_id) {
        let texture = asset_server.load(&resource_def.texture_path);
        let font = asset_server.load("fonts/fira_sans/FiraSans-Bold.ttf");
        
        // Spawn the resource node entity
        let resource_entity = commands.spawn((
            SpriteBundle {
                texture,
                sprite: Sprite {
                    color: resource_def.color,
                    custom_size: Some(Vec2::new(30.0, 30.0)),
                    ..default()
                },
                transform: Transform::from_translation(Vec3::new(position.x, position.y, -0.1))
                    .with_scale(Vec3::new(0.5, 0.5, 1.0)),
                ..default()
            },
            ResourceNode {
                resource_id: resource_id.clone(),
                amount_remaining: amount,
                max_amount: amount,
            },
        )).id();
        
        // Add a small label above the resource
        commands.spawn(Text2dBundle {
            text: Text::from_section(
                resource_def.name.clone(),
                TextStyle {
                    font,
                    font_size: 12.0,
                    color: resource_def.color,
                },
            ).with_alignment(TextAlignment::Center),
            transform: Transform::from_translation(Vec3::new(position.x, position.y + 20.0, 0.0)),
            ..default()
        });
    }
}
