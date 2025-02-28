use bevy::prelude::*;
use std::collections::HashMap;

// Using a string identifier gives us maximum flexibility
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ResourceId(pub String);

// Resource definition with properties
#[derive(Debug, Clone)]
pub struct ResourceDefinition {
    pub id: ResourceId,
    pub name: String,
    pub color: Color,
    pub gathering_time: f32,  // Time in seconds to gather one unit
    pub icon_path: String,    // Path to the resource icon
    #[allow(dead_code)]
    pub value: u32,           // Base value/importance for later game mechanics
}

// Registry of all resource types in the game
#[derive(Resource)]
pub struct ResourceRegistry {
    resources: HashMap<ResourceId, ResourceDefinition>,
}

impl Default for ResourceRegistry {
    fn default() -> Self {
        let mut registry = Self {
            resources: HashMap::new(),
        };

        // Register default resources
        registry.register(ResourceDefinition {
            id: ResourceId("gold".to_string()),
            name: "Gold".to_string(),
            color: Color::srgb(1.0, 0.84, 0.0),
            gathering_time: 3.0,
            icon_path: "resources/gold.png".to_string(),
            value: 5,
        });

        registry.register(ResourceDefinition {
            id: ResourceId("wood".to_string()),
            name: "Wood".to_string(),
            color: Color::srgb(0.6, 0.4, 0.2), // Corrected color
            gathering_time: 1.5,
            icon_path: "resources/wood.png".to_string(), // Remove the "assets/" prefix
            value: 2,
        });

        registry.register(ResourceDefinition {
            id: ResourceId("stone".to_string()),
            name: "Stone".to_string(),
            color: Color::srgb(0.7, 0.7, 0.7),
            gathering_time: 2.0, // Corrected gathering time
            icon_path: "resources/stone/stone.png".to_string(), // Remove the "assets/" prefix
            value: 3,
        });

        registry
    }
}

impl ResourceRegistry {
    pub fn register(&mut self, definition: ResourceDefinition) {
        self.resources.insert(definition.id.clone(), definition);
    }

    pub fn get(&self, id: &ResourceId) -> Option<&ResourceDefinition> {
        self.resources.get(id)
    }

    pub fn all(&self) -> impl Iterator<Item = &ResourceDefinition> {
        self.resources.values()
    }
}

// Player's resource inventory
#[derive(Resource, Default)]
pub struct PlayerResources {
    resources: HashMap<ResourceId, u32>,
}

impl PlayerResources {
    #[allow(dead_code)]
    pub fn add(&mut self, resource_id: &ResourceId, amount: u32) {
        *self.resources.entry(resource_id.clone()).or_insert(0) += amount;
    }
    
    pub fn get(&self, resource_id: &ResourceId) -> u32 {
        self.resources.get(resource_id).copied().unwrap_or(0)
    }
    
    #[allow(dead_code)]
    pub fn has_enough(&self, resource_id: &ResourceId, amount: u32) -> bool {
        self.get(resource_id) >= amount
    }
    
    #[allow(dead_code)]
    pub fn spend(&mut self, resource_id: &ResourceId, amount: u32) -> bool {
        let current = self.get(resource_id);
        if current >= amount {
            self.resources.insert(resource_id.clone(), current - amount);
            true
        } else {
            false
        }
    }
    
    #[allow(dead_code)]
    pub fn all(&self) -> &HashMap<ResourceId, u32> {
        &self.resources
    }
}

// Game state (keep this if you need it)
#[derive(Resource)]
pub struct GameState {
    #[allow(dead_code)]
    pub paused: bool,  // Will be used for pause functionality
}

impl Default for GameState {
    fn default() -> Self {
        GameState {
            paused: false,
        }
    }
}
