use bevy::prelude::*;
use bevy::input::mouse::MouseButton;
use bevy::window::PrimaryWindow;

use crate::components::unit::{Selected, Velocity}; // Remove Unit if it's unused
use crate::components::resource::{ResourceNode, Gathering};
use crate::resources::{PlayerResources, ResourceRegistry}; // Remove ResourceId if it's unused

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
                            gather_amount: 5,  // Gather 5 resources at once
                            return_position: None, // Will be set when we find a drop-off point
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

// This system handles the actual gathering process
pub fn gathering_system(
    time: Res<Time>,
    mut commands: Commands,
    mut player_resources: ResMut<PlayerResources>,
    mut gatherers: Query<(Entity, &mut Gathering, &Transform, &mut Velocity)>,
    mut resource_nodes: Query<&mut ResourceNode>,
) {
    for (entity, mut gathering, transform, mut velocity) in gatherers.iter_mut() {
        // If we're not at the resource yet, keep moving toward it
        if let Some(target) = velocity.target {
            let distance = target.distance(transform.translation.truncate());
            
            // If close enough to the resource, start gathering
            if distance < 20.0 {
                velocity.target = None; // Stop moving
                
                // Progress the gathering timer
                gathering.gather_timer.tick(time.delta());
                
                // If timer finished, gather resources
                if gathering.gather_timer.finished() {
                    if let Ok(mut resource) = resource_nodes.get_mut(gathering.target) {
                        // Calculate how much we can actually gather
                        let gather_amount = gathering.gather_amount.min(resource.amount_remaining);
                        
                        if gather_amount > 0 {
                            // Reduce resource amount
                            resource.amount_remaining -= gather_amount;
                            
                            // Add to player resources
                            player_resources.add(&resource.resource_id, gather_amount);
                            
                            // Reset timer for next gathering cycle
                            gathering.gather_timer.reset();
                            
                            // If resource is depleted, stop gathering and remove node
                            if resource.amount_remaining == 0 {
                                commands.entity(gathering.target).despawn();
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
    }
}
