use crate::resources::ResourceId;
use bevy::prelude::*;

#[derive(Component)]
pub struct ResourceNode {
    pub resource_id: ResourceId,
    pub amount_remaining: u32,
}

// Updated Gathering component with states
#[derive(Component)]
pub struct Gathering {
    pub target: Entity,
    pub resource_id: ResourceId,
    pub gather_timer: Timer,
    pub gather_amount: u32,
    pub gather_state: GatheringState,
}

// Add states to track what the worker is doing
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GatheringState {
    MovingToResource,
    Harvesting,
    ReturningResource,
}
