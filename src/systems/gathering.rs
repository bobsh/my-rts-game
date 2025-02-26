use bevy::prelude::*;
use bevy::input::mouse::MouseButton;
use bevy::window::PrimaryWindow;

use crate::components::unit::{Selected, Velocity};
use crate::components::resource::{ResourceNode, Gathering, GatheringState};
use crate::resources::{PlayerResources, ResourceRegistry, ResourceId};
// Import the components from animation
use crate::systems::animation::{GatherEffect, FloatingText};

// This system handles right-click on resources to start gathering
pub fn resource_gathering_command(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    camera_query: Query<(&Camera, &GlobalTransform)>,
    mouse_button_input: Res<Input<MouseButton>>,
    selected_units: Query<(Entity, &Transform), With<Selected>>,
    resource_nodes: Query<(Entity, &Transform, &ResourceNode)>,
    resource_registry: Res<ResourceRegistry>,
) {
    // Only process right-clicks
    if !mouse_button_input.just_pressed(MouseButton::Right) {
        return;
    }

    // Get the cursor position
    let window = window_query.single();
    let (camera, camera_transform) = camera_query.single();
    
    if let Some(cursor_position) = window.cursor_position() {
        if let Some(world_position) = camera.viewport_to_world(camera_transform, cursor_position) {
            let cursor_pos = world_position.origin.truncate();
            
            // Check if we clicked on a resource node
            for (resource_entity, resource_transform, resource) in resource_nodes.iter() {
                let resource_pos = resource_transform.translation.truncate();
                let distance = cursor_pos.distance(resource_pos);
                
                // If we clicked close enough to the resource
                if distance < 50.0 {  // Adjust radius as needed
                    // Get resource definition to determine gathering time
                    let gather_time = if let Some(resource_def) = resource_registry.get(&resource.resource_id) {
                        resource_def.gathering_time
                    } else {
                        2.0 // Default if resource type not found
                    };
                
                    // Command all selected units to gather
                    for (unit_entity, _) in selected_units.iter() {
                        commands.entity(unit_entity).insert(Gathering {
                            target: resource_entity,
                            resource_id: resource.resource_id.clone(),
                            gather_timer: Timer::from_seconds(gather_time, TimerMode::Once),
                            gather_amount: 5,
                            gather_state: GatheringState::MovingToResource,
                            return_position: None,
                        });
                        
                        // Set velocity to move to resource
                        commands.entity(unit_entity).insert(Velocity {
                            value: Vec2::ZERO,
                            target: Some(resource_transform.translation.truncate()),
                            speed: 80.0, // Slightly slower when gathering
                        });
                    }
                    
                    // We found a resource node to gather from, no need to check others
                    break;
                }
            }
        }
    }
}

// Enhanced gathering system
pub fn gathering_system(
    time: Res<Time>,
    mut commands: Commands,
    mut player_resources: ResMut<PlayerResources>,
    mut gatherers: Query<(Entity, &mut Gathering, &Transform, &mut Velocity)>,
    mut resource_nodes: Query<(&mut ResourceNode, &Transform)>,
    asset_server: Res<AssetServer>,
) {
    for (entity, mut gathering, transform, mut velocity) in gatherers.iter_mut() {
        match gathering.gather_state {
            GatheringState::MovingToResource => {
                // Check if we've reached the resource
                if let Some(_) = velocity.target {
                    if let Ok((_, resource_transform)) = resource_nodes.get(gathering.target) {
                        let distance = transform.translation.truncate().distance(resource_transform.translation.truncate());
                        
                        // If close enough to the resource, start harvesting
                        if distance < 40.0 {
                            velocity.target = None; // Stop moving
                            gathering.gather_state = GatheringState::Harvesting;
                            
                            // Create a gathering effect
                            spawn_gather_effect(&mut commands, &asset_server, resource_transform.translation);
                        }
                    } else {
                        // Target resource no longer exists
                        commands.entity(entity).remove::<Gathering>();
                    }
                }
            },
            
            GatheringState::Harvesting => {
                // Progress the gathering timer
                gathering.gather_timer.tick(time.delta());
                
                // If timer finished, collect resources
                if gathering.gather_timer.finished() {
                    // First get the transform (immutable borrow)
                    let resource_transform_opt = if let Ok((_, resource_transform)) = resource_nodes.get(gathering.target) {
                        Some(resource_transform.translation)
                    } else {
                        None
                    };
                    
                    // Now handle the mutable borrow separately
                    if let Ok((mut resource, _)) = resource_nodes.get_mut(gathering.target) {
                        // Calculate how much we can actually gather
                        let gather_amount = gathering.gather_amount.min(resource.amount_remaining);
                        let resource_id = resource.resource_id.clone();
                        
                        if gather_amount > 0 {
                            // Reduce resource amount
                            resource.amount_remaining -= gather_amount;
                            
                            // Add to player resources
                            player_resources.add(&resource_id, gather_amount);
                            
                            // Create floating text showing gathered amount if we have the transform
                            if let Some(resource_pos) = resource_transform_opt {
                                spawn_resource_collected_text(
                                    &mut commands, 
                                    &asset_server, 
                                    resource_pos,
                                    gather_amount,
                                    &resource_id
                                );
                            }
                            
                            // Reset timer for next gathering cycle
                            gathering.gather_timer.reset();
                            
                            // If resource is depleted, stop gathering and remove node
                            if resource.amount_remaining == 0 {
                                commands.entity(gathering.target).despawn();
                                commands.entity(entity).remove::<Gathering>();
                                return;
                            }
                        }
                    } else {
                        // Resource no longer exists
                        commands.entity(entity).remove::<Gathering>();
                    }
                }
            },
            
            // Implement these states when you add buildings
            GatheringState::ReturningResource => {},
            GatheringState::DeliveringResource => {},
        }
    }
}

// Helper function to spawn a visual effect when gathering
fn spawn_gather_effect(commands: &mut Commands, asset_server: &Res<AssetServer>, position: Vec3) {
    // You could load different effect textures based on resource type
    let effect_texture = asset_server.load("effects/gather_effect.png");
    
    commands.spawn((
        SpriteBundle {
            texture: effect_texture,
            sprite: Sprite {
                color: Color::rgba(1.0, 1.0, 1.0, 0.7),
                custom_size: Some(Vec2::new(20.0, 20.0)),
                ..default()
            },
            transform: Transform::from_translation(position + Vec3::new(0.0, 10.0, 0.1)),
            ..default()
        },
        GatherEffect {
            timer: Timer::from_seconds(0.5, TimerMode::Once),
        },
    ));
}

// Helper function to spawn floating text when resources are collected
fn spawn_resource_collected_text(
    commands: &mut Commands, 
    asset_server: &Res<AssetServer>, 
    position: Vec3, 
    amount: u32,
    resource_id: &ResourceId
) {
    let font = asset_server.load("fonts/fira_sans/FiraSans-Bold.ttf");
    
    commands.spawn((
        Text2dBundle {
            text: Text::from_section(
                format!("+{}", amount),
                TextStyle {
                    font,
                    font_size: 16.0,
                    color: Color::WHITE,
                },
            ).with_alignment(TextAlignment::Center),
            transform: Transform::from_translation(position + Vec3::new(0.0, 20.0, 0.1)),
            ..default()
        },
        FloatingText {
            timer: Timer::from_seconds(1.0, TimerMode::Once),
            velocity: Vec2::new(0.0, 20.0),
            resource_id: resource_id.clone(),
        },
    ));
}

// System to animate gather effects
pub fn animate_gather_effects(
    time: Res<Time>,
    mut commands: Commands,
    mut query: Query<(Entity, &mut GatherEffect, &mut Transform, &mut Sprite)>,
) {
    for (entity, mut effect, mut transform, mut sprite) in query.iter_mut() {
        effect.timer.tick(time.delta());
        
        // Fade out and scale up as timer progresses
        let progress = effect.timer.percent();
        sprite.color.set_a(1.0 - progress);
        transform.scale = Vec3::splat(1.0 + progress * 0.5);
        
        // Remove when timer is finished
        if effect.timer.finished() {
            commands.entity(entity).despawn();
        }
    }
}

// System to animate floating text
pub fn animate_floating_text(
    time: Res<Time>,
    mut commands: Commands,
    mut query: Query<(Entity, &mut FloatingText, &mut Transform, &mut Text)>,
    resource_registry: Res<ResourceRegistry>,
) {
    for (entity, mut floating_text, mut transform, mut text) in query.iter_mut() {
        floating_text.timer.tick(time.delta());
        
        // Move the text upward
        let delta = floating_text.velocity * time.delta_seconds();
        transform.translation.x += delta.x;
        transform.translation.y += delta.y;
        
        // Fade out as timer progresses
        let progress = floating_text.timer.percent();
        if let Some(resource_def) = resource_registry.get(&floating_text.resource_id) {
            text.sections[0].style.color = resource_def.color.with_a(1.0 - progress);
        } else {
            text.sections[0].style.color = text.sections[0].style.color.with_a(1.0 - progress);
        }
        
        // Remove when timer is finished
        if floating_text.timer.finished() {
            commands.entity(entity).despawn();
        }
    }
}
