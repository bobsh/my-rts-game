use bevy::prelude::*;
use std::collections::HashMap;
use crate::resources::ResourceId;

#[derive(Component, Debug, Clone)]
pub struct Inventory {
    // Map of resource type to amount
    resources: HashMap<ResourceId, u32>,
    // Maximum capacity (total weight or items)
    capacity: u32,
    // Current used capacity
    used_capacity: u32,
}

impl Inventory {
    pub fn new(capacity: u32) -> Self {
        Self {
            resources: HashMap::new(),
            capacity,
            used_capacity: 0,
        }
    }

    // Add resources to inventory, returns how much was actually added
    pub fn add(&mut self, resource_id: &ResourceId, amount: u32) -> u32 {
        let available_space = self.capacity - self.used_capacity;
        
        // Calculate how much we can actually add
        let amount_to_add = amount.min(available_space);
        
        if amount_to_add > 0 {
            // Add the resource
            *self.resources.entry(resource_id.clone()).or_insert(0) += amount_to_add;
            self.used_capacity += amount_to_add;
        }
        
        amount_to_add
    }
    
    // Remove resources from inventory
    pub fn remove(&mut self, resource_id: &ResourceId, amount: u32) -> u32 {
        if let Some(current_amount) = self.resources.get_mut(resource_id) {
            let amount_to_remove = amount.min(*current_amount);
            *current_amount -= amount_to_remove;
            self.used_capacity -= amount_to_remove;
            
            // Remove the entry if amount is now 0
            if *current_amount == 0 {
                self.resources.remove(resource_id);
            }
            
            amount_to_remove
        } else {
            0
        }
    }
    
    // Get the amount of a specific resource
    pub fn get_amount(&self, resource_id: &ResourceId) -> u32 {
        *self.resources.get(resource_id).unwrap_or(&0)
    }
    
    // Check if inventory is full
    pub fn is_full(&self) -> bool {
        self.used_capacity >= self.capacity
    }
    
    // Check if inventory is empty
    pub fn is_empty(&self) -> bool {
        self.used_capacity == 0
    }
    
    // Get current used capacity
    pub fn used_capacity(&self) -> u32 {
        self.used_capacity
    }
    
    // Get maximum capacity
    pub fn capacity(&self) -> u32 {
        self.capacity
    }
    
    // Get all resources in inventory
    pub fn resources(&self) -> &HashMap<ResourceId, u32> {
        &self.resources
    }
}
