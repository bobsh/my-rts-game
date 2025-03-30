use bevy::app::{App, Plugin};
use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;

use crate::components::inventory::{Inventory, InventorySettings};
use crate::components::movement::{Collider, Movable, MoveTarget};
use crate::components::resources::ResourceNode;
use crate::components::skills::{SkillProgression, Skills};
use crate::components::unit::Selectable;

pub struct EntitiesPlugin;

#[derive(Default, Component)]
pub struct Character;

#[derive(Default, Bundle, LdtkEntity)]
struct CharacterBundle {
    character: Character,
    selectable: Selectable,
    collider: Collider,
    #[sprite_sheet]
    sprite_sheet: Sprite,
    #[grid_coords]
    grid_coords: GridCoords,
    movable: Movable,
    move_target: MoveTarget,
    inventory: Inventory,
    inventory_settings: InventorySettings,
    skills: Skills,
    skill_progression: SkillProgression,
}

#[derive(Default, Component)]
pub struct Mine;

#[derive(Default, Bundle, LdtkEntity)]
struct MineBundle {
    collider: Collider,
    mine: Mine,
    resource_node: ResourceNode,
    selectable: Selectable,
    #[sprite_sheet]
    sprite_sheet: Sprite,
    #[grid_coords]
    grid_coords: GridCoords,
}

#[derive(Default, Component)]
pub struct Quarry;

#[derive(Default, Bundle, LdtkEntity)]
struct QuarryBundle {
    collider: Collider,
    quarry: Quarry,
    resource_node: ResourceNode,
    selectable: Selectable,
    #[sprite_sheet]
    sprite_sheet: Sprite,
    #[grid_coords]
    grid_coords: GridCoords,
}

#[derive(Default, Component)]
pub struct Tree;

#[derive(Default, Bundle, LdtkEntity)]
struct TreeBundle {
    collider: Collider,
    resource_node: ResourceNode,
    selectable: Selectable,
    tree: Tree,
    #[sprite_sheet]
    sprite_sheet: Sprite,
    #[grid_coords]
    grid_coords: GridCoords,
}

#[derive(Default, Component)]
pub struct House;

#[derive(Default, Bundle, LdtkEntity)]
struct HouseBundle {
    house: House,
    selectable: Selectable,
    collider: Collider,
    #[sprite_sheet]
    sprite_sheet: Sprite,
    #[grid_coords]
    grid_coords: GridCoords,
    inventory: Inventory,
    inventory_settings: InventorySettings,
}

impl Plugin for EntitiesPlugin {
    fn build(&self, app: &mut App) {
        app.register_ldtk_entity::<CharacterBundle>("Character")
            .register_ldtk_entity::<MineBundle>("Mine")
            .register_ldtk_entity::<QuarryBundle>("Quarry")
            .register_ldtk_entity::<TreeBundle>("Tree")
            .register_ldtk_entity::<HouseBundle>("House");
    }
}
