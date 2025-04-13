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
