use bevy::app::{App, Plugin};
use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;

use crate::components::movement::{Movable, MoveTarget};
use crate::components::unit::Selectable;

pub struct EntitiesPlugin;

#[derive(Default, Component)]
struct Warrior;

// Only update the units that should move (Warrior and Jungleman)
#[derive(Default, Bundle, LdtkEntity)]
struct WarriorBundle {
    warrior: Warrior,
    selectable: Selectable,
    #[sprite_sheet]
    sprite_sheet: Sprite,
    #[grid_coords]
    grid_coords: GridCoords,
    movable: Movable,
    move_target: MoveTarget,
}

#[derive(Default, Component)]
struct Jungleman;

#[derive(Default, Bundle, LdtkEntity)]
struct JunglemanBundle {
    jungleman: Jungleman,
    selectable: Selectable,
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
    #[sprite_sheet]
    sprite_sheet: Sprite,
    #[grid_coords]
    grid_coords: GridCoords,
}

#[derive(Default, Component)]
struct Tree2;

#[derive(Default, Bundle, LdtkEntity)]
struct Tree2Bundle {
    tree2: Tree2,
    selectable: Selectable,
    #[sprite_sheet]
    sprite_sheet: Sprite,
    #[grid_coords]
    grid_coords: GridCoords,
}

impl Default for Movable {
    fn default() -> Self {
        Self { speed: 3.0 } // Units move at 3 grid cells per second
    }
}

impl Plugin for EntitiesPlugin {
    fn build(&self, app: &mut App) {
        app.register_ldtk_entity::<WarriorBundle>("Warrior")
            .register_ldtk_entity::<JunglemanBundle>("Jungleman")
            .register_ldtk_entity::<MineBundle>("Mine")
            .register_ldtk_entity::<QuarryBundle>("Quarry")
            .register_ldtk_entity::<Tree2Bundle>("Tree2");
    }
}
