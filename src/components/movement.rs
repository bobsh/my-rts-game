use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;

#[derive(Component)]
pub struct Movable {
    pub speed: f32,
}

#[derive(Component)]
pub struct MoveTarget {
    pub destination: Option<GridCoords>,
    pub path: Vec<GridCoords>, // Add this to store the path
}

impl Default for MoveTarget {
    fn default() -> Self {
        Self {
            destination: None,
            path: Vec::new(),
        }
    }
}

#[derive(Component)]
pub struct Moving {
    pub from: Vec3,
    pub to: Vec3,
    pub progress: f32,
}
