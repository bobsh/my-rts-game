use bevy::prelude::*;
use crate::components::unit::WorkerAnimation;

pub fn animate_workers(
    time: Res<Time>,
    mut query: Query<(&mut WorkerAnimation, &mut Transform)>,
) {
    for (mut animation, mut transform) in query.iter_mut() {
        animation.timer.tick(time.delta());
        
        // Simple "bobbing" animation
        let scale = 0.8 + (animation.timer.percent() * std::f32::consts::PI * 2.0).sin() * 0.05;
        transform.scale = Vec3::new(scale, scale, 1.0);
    }
}
