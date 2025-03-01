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
    
    #[allow(dead_code)]
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
    
    #[allow(dead_code)]
    pub fn get_amount(&self, resource_id: &ResourceId) -> u32 {
        *self.resources.get(resource_id).unwrap_or(&0)
    }
    
    // Check if inventory is full
    pub fn is_full(&self) -> bool {
        self.used_capacity >= self.capacity
    }
    
    #[allow(dead_code)]
    pub fn is_empty(&self) -> bool {
        self.used_capacity == 0
    }
    
    #[allow(dead_code)]
    pub fn used_capacity(&self) -> u32 {
        self.used_capacity
    }
    
    #[allow(dead_code)]
    pub fn capacity(&self) -> u32 {
        self.capacity
    }
    
    // Get all resources in inventory
    pub fn resources(&self) -> &HashMap<ResourceId, u32> {
        &self.resources
    }
}

// Replace the current tests module with this more comprehensive version

#[cfg(test)]
mod tests {
    use super::*;
    use crate::resources::ResourceId;

    #[test]
    fn test_inventory_new() {
        let inventory = Inventory::new(20);
        assert_eq!(inventory.capacity(), 20);
        assert_eq!(inventory.used_capacity(), 0);
        assert!(inventory.is_empty());
        assert!(inventory.resources().is_empty());
    }

    #[test]
    fn test_inventory_capacity() {
        let inventory = Inventory::new(20);
        assert_eq!(inventory.capacity(), 20);
    }

    #[test]
    fn test_inventory_is_full() {
        let mut inventory = Inventory::new(10);
        let resource_id = ResourceId("gold".to_string());
        
        // Initially empty
        assert!(!inventory.is_full());
        
        // Add resources until full
        inventory.add(&resource_id, 10);
        assert!(inventory.is_full());
    }

    #[test]
    fn test_inventory_is_empty() {
        let mut inventory = Inventory::new(10);
        let gold = ResourceId("gold".to_string());
        
        // Initially empty
        assert!(inventory.is_empty());
        
        // Add some resources
        inventory.add(&gold, 5);
        assert!(!inventory.is_empty());
        
        // Remove all resources
        inventory.remove(&gold, 5);
        assert!(inventory.is_empty());
    }

    #[test]
    fn test_inventory_used_capacity() {
        let mut inventory = Inventory::new(30);
        let gold = ResourceId("gold".to_string());
        let wood = ResourceId("wood".to_string());
        
        assert_eq!(inventory.used_capacity(), 0);
        
        inventory.add(&gold, 10);
        assert_eq!(inventory.used_capacity(), 10);
        
        inventory.add(&wood, 15);
        assert_eq!(inventory.used_capacity(), 25);
        
        inventory.remove(&gold, 5);
        assert_eq!(inventory.used_capacity(), 20);
    }

    #[test]
    fn test_inventory_add_remove() {
        let mut inventory = Inventory::new(30);
        let gold = ResourceId("gold".to_string());
        let wood = ResourceId("wood".to_string());
        
        inventory.add(&gold, 10);
        inventory.add(&wood, 5);
        
        assert_eq!(inventory.get_amount(&gold), 10);
        assert_eq!(inventory.get_amount(&wood), 5);
        
        inventory.remove(&gold, 3);
        assert_eq!(inventory.get_amount(&gold), 7);
        
        // Test removing more than available
        inventory.remove(&wood, 10);
        assert_eq!(inventory.get_amount(&wood), 0);
    }

    #[test]
    fn test_inventory_add_beyond_capacity() {
        let mut inventory = Inventory::new(10);
        let gold = ResourceId("gold".to_string());
        
        // Try to add more than capacity
        let added = inventory.add(&gold, 15);
        
        // Should only add up to capacity
        assert_eq!(added, 10);
        assert_eq!(inventory.get_amount(&gold), 10);
        assert_eq!(inventory.used_capacity(), 10);
        assert!(inventory.is_full());
    }

    #[test]
    fn test_inventory_add_to_existing() {
        let mut inventory = Inventory::new(20);
        let gold = ResourceId("gold".to_string());
        
        inventory.add(&gold, 5);
        assert_eq!(inventory.get_amount(&gold), 5);
        
        inventory.add(&gold, 3);
        assert_eq!(inventory.get_amount(&gold), 8);
    }

    #[test]
    fn test_inventory_remove_from_empty() {
        let mut inventory = Inventory::new(20);
        let gold = ResourceId("gold".to_string());
        
        // Try to remove from empty
        let removed = inventory.remove(&gold, 5);
        
        // Should remove nothing
        assert_eq!(removed, 0);
        assert_eq!(inventory.used_capacity(), 0);
    }

    #[test]
    fn test_inventory_resources_map() {
        let mut inventory = Inventory::new(30);
        let gold = ResourceId("gold".to_string());
        let wood = ResourceId("wood".to_string());
        
        inventory.add(&gold, 10);
        inventory.add(&wood, 5);
        
        let resources = inventory.resources();
        assert_eq!(resources.len(), 2);
        assert!(resources.contains_key(&gold));
        assert!(resources.contains_key(&wood));
        assert_eq!(*resources.get(&gold).unwrap(), 10);
        assert_eq!(*resources.get(&wood).unwrap(), 5);
    }

    #[test]
    fn test_inventory_get_nonexistent() {
        let inventory = Inventory::new(10);
        let nonexistent = ResourceId("nonexistent".to_string());
        
        // Should return 0 for nonexistent resources
        assert_eq!(inventory.get_amount(&nonexistent), 0);
    }

    #[test]
    fn test_inventory_remove_all_removes_entry() {
        let mut inventory = Inventory::new(20);
        let gold = ResourceId("gold".to_string());
        
        inventory.add(&gold, 5);
        assert!(inventory.resources().contains_key(&gold));
        
        inventory.remove(&gold, 5); // Remove all
        assert!(!inventory.resources().contains_key(&gold)); // Entry should be gone
    }
}
