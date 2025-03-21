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

#[derive(Component, Default)]
pub struct Velocity {
    pub value: Vec2,
    pub target: Option<Vec2>,
    pub speed: f32,
}

#[derive(Component)]
pub struct MoveMarker {
    pub timer: Timer,
}
