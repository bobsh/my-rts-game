use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;

#[derive(Component, Debug)]
pub struct Movable {
    pub speed: f32,
}

impl Default for Movable {
    fn default() -> Self {
        Self { speed: 3.0 }
    }
}

#[derive(Component, Debug, Default)]
pub struct MoveTarget {
    pub destination: Option<GridCoords>,
    pub path: Vec<GridCoords>,
}

#[derive(Component, Debug)]
pub struct Moving {
    pub from: Vec3,
    pub to: Vec3,
    pub progress: f32,
}

#[derive(Component, Default)]
pub struct Collider;

// New component to track if an entity is busy doing something else
#[derive(Component)]
#[allow(dead_code)]
pub struct Busy {
    pub reason: BusyReason,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[allow(dead_code)]
pub enum BusyReason {
    Gathering,
    Building,
    Fighting,
}
