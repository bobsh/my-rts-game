use bevy::app::{App, Plugin};
use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;

pub struct EntitiesPlugin;

#[derive(Default, Bundle, LdtkEntity)]
struct WarriorBundle {
    #[sprite_sheet]
    sprite_sheet: Sprite,
}

#[derive(Default, Bundle, LdtkEntity)]
struct JunglemanBundle {
    #[sprite_sheet]
    sprite_sheet: Sprite,
}

#[derive(Default, Bundle, LdtkEntity)]
struct MineBundle {
    #[sprite_sheet]
    sprite_sheet: Sprite,
}

#[derive(Default, Bundle, LdtkEntity)]
struct QuarryBundle {
    #[sprite_sheet]
    sprite_sheet: Sprite,
}

#[derive(Default, Bundle, LdtkEntity)]
struct Tree2Bundle {
    #[sprite_sheet]
    sprite_sheet: Sprite,
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
