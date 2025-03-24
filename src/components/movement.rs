use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;

// Add a Movable component to entities that can move
#[derive(Component)]
pub struct Movable {
    pub speed: f32, // Grid cells per second
}

impl Default for Movable {
    fn default() -> Self {
        Self { speed: 3.0 } // Units move at 3 grid cells per second
    }
}

// Add a MoveTarget component to entities that are moving
#[derive(Component, Default)]
pub struct MoveTarget {
    pub destination: Option<GridCoords>,
    pub path: Vec<GridCoords>,
}

// Add a Moving component to entities that are currently moving
#[derive(Component)]
pub struct Moving {
    pub from: Vec3,
    pub to: Vec3,
    pub progress: f32,
}

// Add a collider component to entities that should block movement
#[derive(Component, Default)]
pub struct Collider;
