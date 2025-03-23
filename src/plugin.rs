use bevy::app::{App, Plugin, Startup, Update};
use bevy_ecs_ldtk::prelude::*;
use bevy_inspector_egui::quick::WorldInspectorPlugin;

use crate::entities::EntitiesPlugin;
use crate::systems::camera::CameraPlugin;
use crate::systems::movement::MovementPlugin;
use crate::systems::scene::setup_scene;
use crate::systems::selection::{
    draw_selection_boxes, highlight_selected, selection_system, update_selection_ring,
};
use crate::systems::ui::setup_ui;
use crate::systems::window::{setup_window, setup_window_icon};

pub struct RtsPlugin;

impl Plugin for RtsPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(LdtkPlugin)
            .add_plugins(WorldInspectorPlugin::new())
            .insert_resource(LevelSelection::index(0))
            .add_plugins(EntitiesPlugin)
            .add_plugins(MovementPlugin)
            .add_plugins(CameraPlugin)
            .add_systems(
                Startup,
                (setup_ui, setup_window_icon, setup_window, setup_scene),
            )
            .add_systems(
                Update,
                (
                    selection_system,
                    highlight_selected,
                    draw_selection_boxes,
                    update_selection_ring,
                ),
            );
    }
}
