use bevy::prelude::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ResourceType {
    Gold,
    Wood,
    Stone,
}

// Represents a stack of items in an inventory slot
#[derive(Component, Debug, Clone)]
pub struct InventorySlot {
    pub resource_type: ResourceType,
    pub quantity: u32,
}

// Entity's inventory containing multiple slots
#[derive(Component, Debug)]
pub struct Inventory {
    pub slots: Vec<Option<InventorySlot>>,
    pub max_slots: usize,
}

// Maximum capacity for different entity types
#[derive(Component, Debug)]
pub struct InventorySettings {
    pub max_stack_size: u32,
}

impl Default for InventorySettings {
    fn default() -> Self {
        Self {
            max_stack_size: 10,
        }
    }
}

impl Inventory {
    pub fn new(max_slots: usize) -> Self {
        let mut slots = Vec::with_capacity(max_slots);
        for _ in 0..max_slots {
            slots.push(None);
        }
        Self { slots, max_slots }
    }

    // Add resources to inventory, returns amount that couldn't fit
    pub fn add_resource(&mut self, resource_type: ResourceType, quantity: u32, max_stack: u32) -> u32 {
        let mut remaining = quantity;

        // First fill existing stacks of same resource type
        for slot in &mut self.slots {
            if remaining == 0 {
                break;
            }

            if let Some(inv_slot) = slot {
                if inv_slot.resource_type == resource_type && inv_slot.quantity < max_stack {
                    let can_add = max_stack - inv_slot.quantity;
                    let to_add = remaining.min(can_add);

                    inv_slot.quantity += to_add;
                    remaining -= to_add;
                }
            }
        }

        // If we still have resources, try to find empty slots
        if remaining > 0 {
            for slot in &mut self.slots {
                if remaining == 0 {
                    break;
                }

                if slot.is_none() {
                    let to_add = remaining.min(max_stack);
                    *slot = Some(InventorySlot {
                        resource_type,
                        quantity: to_add,
                    });
                    remaining -= to_add;
                }
            }
        }

        remaining
    }

    // Remove resources from inventory, returns amount actually removed
    pub fn remove_resource(&mut self, resource_type: ResourceType, quantity: u32) -> u32 {
        let mut remaining = quantity;
        let mut removed = 0;

        for slot in &mut self.slots {
            if remaining == 0 {
                break;
            }

            if let Some(inv_slot) = slot {
                if inv_slot.resource_type == resource_type {
                    let to_remove = remaining.min(inv_slot.quantity);

                    inv_slot.quantity -= to_remove;
                    removed += to_remove;
                    remaining -= to_remove;

                    // If slot is empty, remove it
                    if inv_slot.quantity == 0 {
                        *slot = None;
                    }
                }
            }
        }

        removed
    }

    // Count total of a specific resource
    pub fn count_resource(&self, resource_type: ResourceType) -> u32 {
        self.slots
            .iter()
            .filter_map(|slot| {
                slot.as_ref().and_then(|s| {
                    if s.resource_type == resource_type {
                        Some(s.quantity)
                    } else {
                        None
                    }
                })
            })
            .sum()
    }

    // Add a method to get capacity information
    pub fn capacity_info(&self) -> (usize, usize) {
        let used = self.slots.iter().filter(|slot| slot.is_some()).count();
        (used, self.max_slots)
    }

    // Add a simple item transfer system
    pub fn transfer_to(&mut self, other: &mut Inventory, resource_type: ResourceType, quantity: u32, max_stack: u32) -> u32 {
        // First remove from this inventory
        let removed = self.remove_resource(resource_type, quantity);

        // Then add to the other inventory
        let overflow = other.add_resource(resource_type, removed, max_stack);

        // If there was overflow, add it back to original inventory
        if overflow > 0 {
            self.add_resource(resource_type, overflow, max_stack);
        }

        // Return how much was successfully transferred
        removed - overflow
    }
}
