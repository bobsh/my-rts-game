use bevy::prelude::*;

#[derive(Component, Default)]
pub struct Unit;

#[derive(Component)]
pub struct Selectable;

#[derive(Component)]
pub struct Selected;

#[derive(Component)]
pub struct WorkerAnimation {
    pub timer: Timer,
}

#[derive(Component)]
pub struct SelectionRing;

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
