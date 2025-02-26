use bevy::prelude::*;
use crate::resources::ResourceId;

#[derive(Component)]
pub struct ResourceNode {
    pub resource_id: ResourceId,
    pub amount_remaining: u32,
    pub max_amount: u32,
}

// Component for workers that are gathering resources
#[derive(Component)]
pub struct Gathering {
    pub target: Entity,
    pub resource_id: ResourceId,
    pub gather_timer: Timer,
    pub gather_amount: u32,
    pub return_position: Option<Vec3>, // Position to return resources to
}
