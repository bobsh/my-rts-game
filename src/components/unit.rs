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
