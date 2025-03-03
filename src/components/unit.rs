use bevy::prelude::*;

#[derive(Component, Default)]
pub struct Unit;

#[derive(Component)]
pub struct Selectable;

#[derive(Component)]
pub struct Selected;

// Enhanced worker animation that supports different states
#[derive(Component)]
pub struct WorkerAnimation {
    pub timer: Timer,
    pub state: WorkerAnimationState,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WorkerAnimationState {
    Idle,
    Walking,
    Mining,
    Woodcutting,
    Delivering,
}

// Update SelectionRing to include animation timer
#[derive(Component)]
pub struct SelectionRing {
    pub timer: Timer,
    pub base_size: f32,
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

#[derive(Component)]
pub struct UnitAttributes {
    pub name: String,
    pub health: f32,
    pub max_health: f32,
}
