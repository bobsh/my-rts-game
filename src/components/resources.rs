use bevy::prelude::*;
use crate::components::inventory::ResourceType;

// Generic resource node component
#[derive(Component, Debug)]
pub struct ResourceNode {
    pub resource_type: ResourceType,
    pub quantity: u32,         // How much resource is available
    pub max_quantity: u32,     // Maximum resource quantity
    pub regenerate: bool,      // Whether the resource regenerates over time
    pub regeneration_rate: f32, // How fast it regenerates (units per second)
}

impl ResourceNode {
    pub fn new(resource_type: ResourceType, quantity: u32) -> Self {
        Self {
            resource_type,
            quantity,
            max_quantity: quantity,
            regenerate: true,
            regeneration_rate: 0.1,
        }
    }

    pub fn extract(&mut self, amount: u32) -> u32 {
        let available = self.quantity;
        let extracted = amount.min(available);
        self.quantity -= extracted;
        extracted
    }
}
