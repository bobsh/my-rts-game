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
    pub gathering_time: f32, // Time in seconds to gather one unit
    pub icon_path: String,   // Path to the resource icon
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
        });

        registry.register(ResourceDefinition {
            id: ResourceId("wood".to_string()),
            name: "Wood".to_string(),
            color: Color::srgb(0.6, 0.4, 0.2), // Corrected color
            gathering_time: 1.5,
            icon_path: "resources/wood.png".to_string(), // Remove the "assets/" prefix
        });

        registry.register(ResourceDefinition {
            id: ResourceId("stone".to_string()),
            name: "Stone".to_string(),
            color: Color::srgb(0.7, 0.7, 0.7),
            gathering_time: 2.0, // Corrected gathering time
            icon_path: "resources/stone/stone.png".to_string(), // Remove the "assets/" prefix
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
    pub fn get(&self, resource_id: &ResourceId) -> u32 {
        self.resources.get(resource_id).copied().unwrap_or(0)
    }
}
