use bevy::prelude::*;

/// A unit component, a character or npc.
#[derive(Component, Default)]
pub struct Unit;

/// Idicates it can be selected, when applied to an entity.
#[derive(Component, Default)]
pub struct Selectable;

/// Indicates the entity is selected.
#[derive(Component)]
pub struct Selected;

/// Indicates the entity is selected and has a selection ring.
#[derive(Component)]
pub struct SelectionRing {
    pub owner: Entity, // Add this to track which entity this ring belongs to
}
