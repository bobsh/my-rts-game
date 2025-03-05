use bevy::prelude::*;

use crate::resources::{PlayerResources, ResourceRegistry};
use crate::systems::animation::{
    animate_floating_text, animate_gather_effects, animate_workers, update_worker_animations,
};
use crate::systems::gathering::{gathering_system, resource_gathering_command};
use crate::systems::map::setup_background;
use crate::systems::movement::{move_command_system, movement_system, show_destination_markers};
use crate::systems::scene::setup_scene;
use crate::systems::selection::{
    animate_selection_rings, draw_selection_boxes, highlight_selected, selection_system,
    update_selection_ring,
};
use crate::systems::ui::{
    setup_ui, update_inventory_ui, update_resources_display, update_unit_info,
};
use crate::systems::window::setup_window_icon;

pub struct RtsPlugin;

impl Plugin for RtsPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<PlayerResources>()
            .init_resource::<ResourceRegistry>()
            .add_systems(
                Startup,
                (setup_scene, setup_ui, setup_window_icon, setup_background),
            )
            .add_systems(
                Update,
                (
                    selection_system,
                    highlight_selected,
                    draw_selection_boxes,
                    animate_selection_rings,
                    update_selection_ring,
                    animate_workers,
                    move_command_system,
                    movement_system,
                    show_destination_markers,
                    update_unit_info,
                    resource_gathering_command,
                    gathering_system,
                    update_resources_display,
                    update_worker_animations,
                    animate_gather_effects,
                    animate_floating_text,
                    update_inventory_ui,
                ),
            );
    }
}
