use bevy::prelude::*;

/// UI component for displaying the entity's information.
#[derive(Component)]
pub struct EntityInfoPanel;

/// UI component for displaying the entity's name.
#[derive(Component)]
pub struct EntityNameText;

/// UI component for displaying the entity's name.
#[derive(Component, Debug, Clone, PartialEq)]
pub struct UiState {
    pub entity: Entity,
    pub name: String,
    pub inventory_slots_used: usize,
    pub total_inventory_slots: usize,
    pub mining_skill: f32,
    pub woodcutting_skill: f32,
    pub harvesting_skill: f32,
    pub gathering_progress: Option<(String, f32)>,
    pub gathering_intent: Option<String>,
}

impl Default for UiState {
    fn default() -> Self {
        Self {
            entity: Entity::PLACEHOLDER,
            name: "".to_string(),
            inventory_slots_used: 0,
            total_inventory_slots: 0,
            mining_skill: 0.0,
            woodcutting_skill: 0.0,
            harvesting_skill: 0.0,
            gathering_progress: None,
            gathering_intent: None,
        }
    }
}
