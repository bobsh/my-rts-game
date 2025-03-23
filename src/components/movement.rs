use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;

#[derive(Component)]
pub struct Movable {
    pub speed: f32,
}

#[derive(Component, Default)]
pub struct MoveTarget {
    pub destination: Option<GridCoords>,
}

// For smooth animations between grid positions
#[derive(Component)]
pub struct Moving {
    pub from: Vec3,
    pub to: Vec3,
    pub progress: f32,
}
