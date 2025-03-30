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
pub struct Warrior;

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
    inventory: Inventory,
    inventory_settings: InventorySettings,
    skills: Skills,
    skill_progression: SkillProgression,
}

#[derive(Default, Component)]
pub struct Worker;

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
    inventory: Inventory,
    inventory_settings: InventorySettings,
}

// You'll need to implement Default for Inventory to make this work:
impl Default for Inventory {
    fn default() -> Self {
        Self::new(4) // Default 4 slots for workers
    }
}

#[derive(Default, Component)]
pub struct Mine;

#[derive(Default, Bundle, LdtkEntity)]
struct MineBundle {
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
        app.register_ldtk_entity::<WarriorBundle>("Warrior")
            .register_ldtk_entity::<WorkerBundle>("Worker")
            .register_ldtk_entity::<MineBundle>("Mine")
            .register_ldtk_entity::<QuarryBundle>("Quarry")
            .register_ldtk_entity::<TreeBundle>("Tree")
            .register_ldtk_entity::<HouseBundle>("House");
    }
}
