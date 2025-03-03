use crate::resources::ResourceId;
use bevy::prelude::*;

#[derive(Component)]
pub struct UnitInfoPanel;

#[derive(Component)]
pub struct UnitNameText;

#[derive(Component)]
pub struct UnitHealthText;

#[derive(Component)]
pub struct UnitSpeedText;

#[derive(Component)]
pub struct ResourcesDisplay;

#[derive(Component)]
pub struct ResourceText(pub String);

#[derive(Component)]
pub struct InventoryUI;

#[derive(Component)]
pub struct InventorySlot {
    pub resource_id: Option<ResourceId>,
    pub entity_owner: Entity,
}
