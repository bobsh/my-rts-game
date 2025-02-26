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

// Generic component to identify resource text elements
#[derive(Component)]
pub struct ResourceText(pub String); // Store resource ID
