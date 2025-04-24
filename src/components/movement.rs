use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;

/// A component that indicates the entity is a movable object.
#[derive(Component, Debug)]
pub struct Movable {
    pub speed: f32,
}

impl Default for Movable {
    fn default() -> Self {
        Self { speed: 3.0 }
    }
}

/// A component that indicates the entity is a target.
#[derive(Component, Debug, Default)]
pub struct MoveTarget {
    pub destination: Option<GridCoords>,
    pub path: Vec<GridCoords>,
}

/// A component that indicates the entity is moving.
#[derive(Component, Debug)]
pub struct Moving {
    pub from: Vec3,
    pub to: Vec3,
    pub progress: f32,
}

/// A component that indicates the entity is a collisidable object.
#[derive(Component, Default)]
pub struct Collider;
