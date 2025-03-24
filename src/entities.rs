use bevy::app::{App, Plugin};
use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;

use crate::components::movement::{Collider, Movable, MoveTarget};
use crate::components::unit::Selectable;

pub struct EntitiesPlugin;

#[derive(Default, Component)]
struct Warrior;

#[derive(Default, Bundle, LdtkEntity)]
struct WarriorBundle {
    warrior: Warrior,
    selectable: Selectable,
    collider: Collider,
    #[sprite_sheet]
    sprite_sheet: Sprite,
    #[grid_coords]
    grid_coords: GridCoords,
    movable: Movable,
    move_target: MoveTarget,
}

#[derive(Default, Component)]
struct Worker;

#[derive(Default, Bundle, LdtkEntity)]
struct WorkerBundle {
    worker: Worker,
    selectable: Selectable,
    collider: Collider,
    #[sprite_sheet]
    sprite_sheet: Sprite,
    #[grid_coords]
    grid_coords: GridCoords,
    movable: Movable,
    move_target: MoveTarget,
}

#[derive(Default, Component)]
struct Mine;

#[derive(Default, Bundle, LdtkEntity)]
struct MineBundle {
    mine: Mine,
    selectable: Selectable,
    collider: Collider,
    #[sprite_sheet]
    sprite_sheet: Sprite,
    #[grid_coords]
    grid_coords: GridCoords,
}

#[derive(Default, Component)]
struct Quarry;

#[derive(Default, Bundle, LdtkEntity)]
struct QuarryBundle {
    quarry: Quarry,
    selectable: Selectable,
    collider: Collider,
    #[sprite_sheet]
    sprite_sheet: Sprite,
    #[grid_coords]
    grid_coords: GridCoords,
}

#[derive(Default, Component)]
struct Tree;

#[derive(Default, Bundle, LdtkEntity)]
struct TreeBundle {
    tree: Tree,
    selectable: Selectable,
    collider: Collider,
    #[sprite_sheet]
    sprite_sheet: Sprite,
    #[grid_coords]
    grid_coords: GridCoords,
}

#[derive(Default, Component)]
struct House;

#[derive(Default, Bundle, LdtkEntity)]
struct HouseBundle {
    house: House,
    selectable: Selectable,
    collider: Collider,
    #[sprite_sheet]
    sprite_sheet: Sprite,
    #[grid_coords]
    grid_coords: GridCoords,
}

impl Plugin for EntitiesPlugin {
    fn build(&self, app: &mut App) {
        app.register_ldtk_entity::<WarriorBundle>("Warrior")
            .register_ldtk_entity::<WorkerBundle>("Worker")
            .register_ldtk_entity::<MineBundle>("Mine")
            .register_ldtk_entity::<QuarryBundle>("Quarry")
            .register_ldtk_entity::<TreeBundle>("Tree")
            .register_ldtk_entity::<HouseBundle>("House");
    }
}
