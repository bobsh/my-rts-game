use bevy::prelude::*;

#[derive(Component, Default)]
pub struct Unit;

#[derive(Component, Default)]
pub struct Selectable;

#[derive(Component)]
pub struct Selected;

// Update SelectionRing to include animation timer
#[derive(Component)]
pub struct SelectionRing {
    pub owner: Entity, // Add this to track which entity this ring belongs to
}
