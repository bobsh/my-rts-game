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
    pub texture_path: String,
    pub gathering_time: f32,  // Time in seconds to gather one unit
    pub value: u32,           // Base value/importance
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
            color: Color::rgb(1.0, 0.84, 0.0),
            texture_path: "resources/gold.png".to_string(),
            gathering_time: 3.0,
            value: 5,
        });

        registry.register(ResourceDefinition {
            id: ResourceId("wood".to_string()),
            name: "Wood".to_string(),
            color: Color::rgb(0.65, 0.4, 0.2),
            texture_path: "resources/wood.png".to_string(),
            gathering_time: 1.5,
            value: 2,
        });

        registry.register(ResourceDefinition {
            id: ResourceId("stone".to_string()),
            name: "Stone".to_string(),
            color: Color::rgb(0.7, 0.7, 0.7),
            texture_path: "resources/stone.png".to_string(),
            gathering_time: 2.5,
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
    pub fn add(&mut self, resource_id: &ResourceId, amount: u32) {
        *self.resources.entry(resource_id.clone()).or_insert(0) += amount;
    }
    
    pub fn get(&self, resource_id: &ResourceId) -> u32 {
        self.resources.get(resource_id).copied().unwrap_or(0)
    }
    
    pub fn has_enough(&self, resource_id: &ResourceId, amount: u32) -> bool {
        self.get(resource_id) >= amount
    }
    
    pub fn spend(&mut self, resource_id: &ResourceId, amount: u32) -> bool {
        let current = self.get(resource_id);
        if current >= amount {
            self.resources.insert(resource_id.clone(), current - amount);
            true
        } else {
            false
        }
    }
    
    pub fn all(&self) -> &HashMap<ResourceId, u32> {
        &self.resources
    }
}

// Game state (keep this if you need it)
#[derive(Resource)]
pub struct GameState {
    pub paused: bool,
}

impl Default for GameState {
    fn default() -> Self {
        GameState {
            paused: false,
        }
    }
}
