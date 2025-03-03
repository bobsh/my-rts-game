use bevy::prelude::*;
use bevy::window::WindowPlugin;
use bevy::winit::WinitWindows;

mod components;
mod resources;
mod systems;

use components::unit::{Selectable, Unit, WorkerAnimation, WorkerAnimationState, Velocity, UnitAttributes};
use resources::{PlayerResources, ResourceRegistry, ResourceId, GameState};
use systems::selection::{selection_system, highlight_selected, animate_selection_rings, update_selection_ring, draw_selection_boxes};
use systems::animation::{animate_workers, update_worker_animations, animate_gather_effects, animate_floating_text};
use systems::movement::{move_command_system, movement_system, show_destination_markers};
use systems::gathering::{resource_gathering_command, gathering_system};
use systems::ui::{setup_ui, update_unit_info, update_resources_display, update_inventory_ui};
use systems::map::{setup_background, spawn_resource_node}; // Add this import

use components::inventory::Inventory;

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
        .init_resource::<ResourceRegistry>()
        .add_systems(Startup, (setup, setup_ui, setup_window_icon, setup_background))
        .add_systems(Update, (
            selection_system,
            highlight_selected,
            draw_selection_boxes,
            animate_selection_rings,
            update_selection_ring,
            animate_workers,
            move_command_system,
            movement_system,
            show_destination_markers,
            update_unit_info,
            resource_gathering_command,
            gathering_system,
            update_resources_display,
            update_worker_animations,
            animate_gather_effects,
            animate_floating_text,
            update_inventory_ui,
        ))
        .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>, resource_registry: Res<ResourceRegistry>) {
    commands.spawn(Camera2dBundle::default());

    // Spawn worker units
    spawn_worker(&mut commands, &asset_server, Vec2::new(-200.0, 0.0), "units/jungleman/jungleman.png".to_string(), 1);
    spawn_worker(&mut commands, &asset_server, Vec2::new(0.0, 0.0), "units/jungleman/jungleman.png".to_string(), 2);
    spawn_worker(&mut commands, &asset_server, Vec2::new(200.0, 0.0), "units/jungleman/jungleman.png".to_string(), 3);

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
fn spawn_worker(commands: &mut Commands, asset_server: &Res<AssetServer>, position: Vec2, texture_path: String, i: usize) {
    let texture = asset_server.load(&texture_path);

    let _worker_entity = commands
        .spawn((
            SpriteBundle {
                texture,
                transform: Transform::from_translation(Vec3::new(position.x, position.y, 0.0))
                    .with_scale(Vec3::new(0.8, 0.8, 1.0)),
                ..Default::default()
            },
            Name::new(format!("Worker {}", i)),
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
                name: format!("Worker {}", i),
                health: 100.0,
                max_health: 100.0,
            },
            Inventory::new(20), // Each worker can carry 20 resource units
        ))
        .id();
}

// Add this system to your startup systems
fn setup_window_icon(
    windows: Query<Entity, With<bevy::window::PrimaryWindow>>,
    winit_windows: NonSend<WinitWindows>,
) {
    let window_entity = windows.single();

    // Get the actual winit window
    let Some(primary) = winit_windows.get_window(window_entity) else {
        return;
    };

    // Load the icon
    let icon_path = "assets/icons/quillbrainstars/quillbrainstars-64x64.png"; // Use PNG for runtime
    let icon_bytes = std::fs::read(icon_path).unwrap_or_else(|_| {
        println!("Failed to load icon");
        Vec::new()
    });

    // Create the icon
    if let Ok(image) = image::load_from_memory(&icon_bytes) {
        let rgba = image.into_rgba8();
        let (width, height) = rgba.dimensions();
        let rgba_bytes = rgba.into_raw();

        if let Ok(icon) = winit::window::Icon::from_rgba(rgba_bytes, width, height) {
            primary.set_window_icon(Some(icon));
            println!("Set window icon successfully!");
        }
    }
}
