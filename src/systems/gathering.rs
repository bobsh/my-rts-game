use bevy::prelude::*;
use bevy::input::mouse::MouseButton;
use bevy::window::PrimaryWindow;
use std::collections::HashMap;

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
                if distance < 70.0 {  // Increased from 50.0 for better click detection
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
    mut param_set: ParamSet<(
        Query<(Entity, &mut Gathering, &mut Transform, &mut Velocity)>,
        Query<(Entity, &mut ResourceNode, &Transform)>,
    )>,
    asset_server: Res<AssetServer>,
) {
    // We need to iterate separately to avoid borrow issues with multiple queries
    let mut gather_actions = Vec::new();

    // First pass: check all gatherers and collect actions
    {
        // Get information from resources first into a local data structure
        let mut resource_positions = HashMap::new();
        let mut resource_ids = HashMap::new();
        let mut resource_amounts = HashMap::new();
        
        {
            let resource_query = param_set.p1();
            for (entity, resource, transform) in resource_query.iter() {
                resource_positions.insert(entity, transform.translation);
                resource_ids.insert(entity, resource.resource_id.clone());
                resource_amounts.insert(entity, resource.amount_remaining);
            }
        }
        
        // Now process gatherers using the cached resource data
        let mut gatherer_query = param_set.p0();
        for (entity, mut gathering, mut transform, mut velocity) in gatherer_query.iter_mut() {
            match gathering.gather_state {
                GatheringState::MovingToResource => {
                    // Check if we've reached the resource
                    if let Some(_) = velocity.target {
                        if let Some(&resource_pos) = resource_positions.get(&gathering.target) {
                            let distance = transform.translation.truncate().distance(resource_pos.truncate());
                            
                            // If close enough to the resource, start harvesting
                            if distance < 60.0 {  // Larger threshold for detection
                                velocity.target = None; // Stop moving
                                gathering.gather_state = GatheringState::Harvesting;
                                
                                // Move the worker closer to the resource for better visuals
                                // Find the direction vector from resource to worker
                                let dir = (transform.translation - resource_pos).normalize();
                                // Position the worker at an ideal distance from the resource (30.0 units)
                                let ideal_pos = resource_pos + dir * 30.0;
                                transform.translation = ideal_pos;
                                
                                // Create a gathering effect
                                spawn_gather_effect(&mut commands, &asset_server, resource_pos);
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
                    
                    // First check if we're still close enough to the resource
                    let still_in_range = if let Some(&resource_pos) = resource_positions.get(&gathering.target) {
                        let distance = transform.translation.truncate().distance(resource_pos.truncate());
                        distance < 60.0  // Keep the same detection range
                    } else {
                        false // Resource no longer exists
                    };
                    
                    // If worker moved too far from resource, stop gathering
                    if !still_in_range {
                        gathering.gather_state = GatheringState::MovingToResource;
                        
                        // Set velocity to move back to the resource
                        if let Some(&resource_pos) = resource_positions.get(&gathering.target) {
                            // Find direction from resource to where worker should stand
                            let dir = (transform.translation - resource_pos).normalize();
                            // Target position is at ideal distance from resource
                            let target_pos = resource_pos + dir * 30.0;
                            velocity.target = Some(target_pos.truncate());
                        }
                        
                        continue; // Skip to next worker
                    }
                    
                    // If timer finished, collect resources
                    if gathering.gather_timer.finished() {
                        // Check if resource still exists
                        if let Some(&resource_pos) = resource_positions.get(&gathering.target) {
                            // Store gathering action for second pass
                            gather_actions.push((entity, gathering.target, gathering.gather_amount, resource_pos));
                            // Reset timer for next gathering cycle
                            gathering.gather_timer.reset();
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

    // Second pass: process all gather actions
    {
        let mut resource_query = param_set.p1();
        
        for (entity, resource_entity, gather_amount, resource_pos) in gather_actions {
            if let Ok((_, mut resource, _)) = resource_query.get_mut(resource_entity) {
                // Calculate how much we can actually gather
                let amount = gather_amount.min(resource.amount_remaining);
                let resource_id = resource.resource_id.clone();
                
                if amount > 0 {
                    // Reduce resource amount
                    resource.amount_remaining -= amount;
                    
                    // Add to player resources
                    player_resources.add(&resource_id, amount);
                    
                    // Create floating text showing gathered amount
                    spawn_resource_collected_text(
                        &mut commands, 
                        &asset_server, 
                        resource_pos,
                        amount,
                        &resource_id
                    );
                    
                    // If resource is depleted, stop gathering and remove node
                    if resource.amount_remaining == 0 {
                        commands.entity(resource_entity).despawn();
                        commands.entity(entity).remove::<Gathering>();
                    }
                }
            } else {
                // Resource no longer exists
                commands.entity(entity).remove::<Gathering>();
            }
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
