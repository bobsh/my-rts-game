use bevy::prelude::*;
use crate::resources::ResourceId;

#[derive(Component)]
pub struct ResourceNode {
    pub resource_id: ResourceId,
    pub amount_remaining: u32,
    #[allow(dead_code)]
    pub max_amount: u32,  // Will be used for UI progress bars
}

// Updated Gathering component with states
#[derive(Component)]
pub struct Gathering {
    pub target: Entity,
    pub resource_id: ResourceId,
    pub gather_timer: Timer,
    pub gather_amount: u32,
    pub gather_state: GatheringState,
    #[allow(dead_code)]
    pub return_position: Option<Vec3>, // Will be used when we add buildings
}

// Add states to track what the worker is doing
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GatheringState {
    MovingToResource,
    Harvesting,
    ReturningResource,
    #[allow(dead_code)]
    DeliveringResource,  // Will be used when implementing building resource delivery
}
