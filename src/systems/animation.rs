use bevy::prelude::*;
use crate::components::unit::{WorkerAnimation, WorkerAnimationState};
use crate::components::resource::{Gathering, GatheringState};
use crate::components::unit::Velocity;
use crate::resources::{ResourceRegistry, ResourceId};

// Basic worker animation
pub fn animate_workers(
    time: Res<Time>,
    mut query: Query<(&mut WorkerAnimation, &mut Transform)>,
) {
    for (mut animation, mut transform) in query.iter_mut() {
        // Only animate if timer is finished
        if animation.timer.tick(time.delta()).just_finished() {
            // Different animations based on state
            match animation.state {
                WorkerAnimationState::Idle => {
                    // Simple idle animation - slight bobbing
                    transform.scale = if transform.scale.x > 0.8 {
                        Vec3::new(0.75, 0.8, 1.0)
                    } else {
                        Vec3::new(0.8, 0.8, 1.0)
                    };
                }
                WorkerAnimationState::Walking => {
                    // Walking animation - left/right lean
                    transform.rotation = if transform.rotation.z < 0.0 {
                        Quat::from_rotation_z(0.05)
                    } else {
                        Quat::from_rotation_z(-0.05)
                    };
                }
                WorkerAnimationState::Mining => {
                    // Mining animation - up/down motion
                    transform.translation.y += if transform.translation.y > transform.translation.y - 5.0 {
                        -5.0
                    } else {
                        5.0
                    };
                }
                WorkerAnimationState::Woodcutting => {
                    // Woodcutting animation - back and forth swinging
                    transform.rotation = if transform.rotation.z < 0.0 {
                        Quat::from_rotation_z(0.1)
                    } else {
                        Quat::from_rotation_z(-0.1)
                    };
                }
                WorkerAnimationState::Delivering => {
                    // Delivering animation - slight up/down motion
                    transform.translation.y += if transform.translation.y > transform.translation.y - 2.0 {
                        -2.0
                    } else {
                        2.0
                    };
                }
            }
        }
    }
}

// Update worker animations based on their gathering state
pub fn update_worker_animations(
    mut query: Query<(&mut WorkerAnimation, Option<&Gathering>, &Velocity)>,
    time: Res<Time>,
    resource_registry: Res<ResourceRegistry>,
) {
    for (mut worker_anim, gathering, velocity) in query.iter_mut() {
        // First update the timer
        worker_anim.timer.tick(time.delta());
        
        // Determine the animation state based on what the worker is doing
        let new_state = if let Some(gathering) = gathering {
            match gathering.gather_state {
                GatheringState::MovingToResource => WorkerAnimationState::Walking,
                GatheringState::Harvesting => {
                    // Different animation based on resource type
                    if let Some(resource_def) = resource_registry.get(&gathering.resource_id) {
                        match resource_def.id.0.as_str() {
                            "wood" => WorkerAnimationState::Woodcutting,
                            _ => WorkerAnimationState::Mining, // Default for gold, stone, etc.
                        }
                    } else {
                        WorkerAnimationState::Mining
                    }
                },
                GatheringState::ReturningResource => WorkerAnimationState::Walking,
                GatheringState::DeliveringResource => WorkerAnimationState::Delivering,
            }
        } else if velocity.target.is_some() {
            // If not gathering but moving
            WorkerAnimationState::Walking
        } else {
            // Otherwise idle
            WorkerAnimationState::Idle
        };
        
        // Update the animation state if it changed
        if worker_anim.state != new_state {
            worker_anim.state = new_state;
            
            // Adjust animation speed based on state
            let timer_duration = match new_state {
                WorkerAnimationState::Mining => 0.5, // Faster for mining
                WorkerAnimationState::Woodcutting => 0.8, // Slower for wood cutting
                WorkerAnimationState::Walking => 0.3, // Fast for walking
                WorkerAnimationState::Delivering => 0.4,
                WorkerAnimationState::Idle => 2.0, // Slow for idle
            };
            
            // Reset timer with new duration
            worker_anim.timer = Timer::from_seconds(timer_duration, TimerMode::Repeating);
        }
    }
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

// Make sure these components are public
// Add pub to make them accessible from other files
#[derive(Component)]
pub struct GatherEffect {
    pub timer: Timer,
}

// Component for floating text
#[derive(Component)]
pub struct FloatingText {
    pub timer: Timer,
    pub velocity: Vec2,
    pub resource_id: ResourceId,
}
