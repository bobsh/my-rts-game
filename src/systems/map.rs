// src/systems/map.rs

use crate::components::resource::ResourceNode;
use crate::resources::{ResourceId, ResourceRegistry};
use bevy::prelude::*;

/// Sets up the tiled grass background for the game world
pub fn setup_background(mut commands: Commands, asset_server: Res<AssetServer>) {
    // Load the grass texture
    let grass_texture = asset_server.load("terrain/grass1/grass1.png");

    // Texture size is 1024x1024
    let tile_size = 256.0_f32; // Use smaller tiles for more repetition

    // Calculate how many tiles we need to cover the screen plus some extra
    let screen_width = 1280.0_f32 + 512.0_f32; // Screen width plus buffer
    let screen_height = 720.0_f32 + 512.0_f32; // Screen height plus buffer

    let tiles_x = (screen_width / tile_size).ceil() as i32;
    let tiles_y = (screen_height / tile_size).ceil() as i32;

    // Create parent entity to hold all background tiles
    let background = commands
        .spawn((
            Transform::default(),
            Visibility::default(),
            Name::new("Background Container"),
        ))
        .id();

    // Create a grid of background tiles
    for y in -tiles_y..=tiles_y {
        for x in -tiles_x..=tiles_x {
            commands
                .spawn((
                    Sprite {
                        image: grass_texture.clone(),
                        custom_size: Some(Vec2::new(tile_size, tile_size)),
                        ..Default::default()
                    },
                    Transform::from_translation(Vec3::new(
                        x as f32 * tile_size,
                        y as f32 * tile_size,
                        -100.0, // Behind other elements
                    )),
                    Name::new(format!("Grass Tile {x},{y}")),
                ))
                .set_parent(background);
        }
    }
}

/// Spawns a resource node with the specified properties
pub fn spawn_resource_node(
    commands: &mut Commands,
    asset_server: &Res<AssetServer>,
    resource_registry: &Res<ResourceRegistry>,
    position: Vec2,
    resource_id: &ResourceId,
    amount: u32,
) {
    // Get the resource definition from registry
    if let Some(resource_def) = resource_registry.get(resource_id) {
        let texture = asset_server.load(&resource_def.icon_path); // Use icon_path
        let font = asset_server.load("fonts/fira_sans/FiraSans-Bold.ttf");

        // Spawn the resource node entity
        let _resource_entity = commands
            .spawn((
                Sprite {
                    image: texture,
                    color: resource_def.color,
                    custom_size: Some(Vec2::new(30.0, 30.0)),
                    ..default()
                },
                Transform::from_translation(Vec3::new(position.x, position.y, -0.1))
                    .with_scale(Vec3::new(0.5, 0.5, 1.0)),
                ResourceNode {
                    resource_id: resource_id.clone(),
                    amount_remaining: amount,
                },
            ))
            .id();

        // Add a small label above the resource
        commands.spawn((
            Text2d::new(resource_def.name.clone()),
            TextFont {
                font,
                font_size: 16.0,
                ..default()
            },
            TextColor(Color::WHITE),
            Transform::from_translation(Vec3::new(position.x, position.y + 20.0, 0.0)),
        ));
    }
}
