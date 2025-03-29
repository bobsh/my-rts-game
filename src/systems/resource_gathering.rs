use bevy::prelude::*;
use crate::components::inventory::*;
use crate::components::unit::Selected;
use crate::entities::{Tree, Mine, Quarry, Worker};

pub struct ResourceGatheringPlugin;

impl Plugin for ResourceGatheringPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, gather_resources)
           .add_systems(Update, start_gathering);
    }
}

// Component to track gathering progress
#[derive(Component, Debug)]
pub struct Gathering {
    pub resource_type: ResourceType,
    pub progress: f32,
    pub target: Entity,
}

// System to handle resource gathering
fn gather_resources(
    mut commands: Commands,
    time: Res<Time>,
    mut gatherers: Query<(Entity, &mut Gathering, &mut Inventory, &InventorySettings)>,
    trees: Query<Entity, With<Tree>>,
    mines: Query<Entity, With<Mine>>,
    quarries: Query<Entity, With<Quarry>>,
) {
    for (entity, mut gathering, mut inventory, settings) in &mut gatherers {
        // Add logging to track progress
        info!("Gathering in progress for entity {:?}: {:.1}/{:.1} seconds",
              entity, gathering.progress, 3.0);

        // Update gathering progress
        gathering.progress += time.delta_secs();

        // If gathering is complete (takes 3 seconds)
        if gathering.progress >= 3.0 {
            info!("Gathering complete for entity {:?}", entity);

            // Determine resource type from the target
            let resource_type = if trees.contains(gathering.target) {
                info!("Gathered Wood from tree {:?}", gathering.target);
                ResourceType::Wood
            } else if mines.contains(gathering.target) {
                info!("Gathered Gold from mine {:?}", gathering.target);
                ResourceType::Gold
            } else if quarries.contains(gathering.target) {
                info!("Gathered Stone from quarry {:?}", gathering.target);
                ResourceType::Stone
            } else {
                // Invalid target, stop gathering
                info!("Invalid gathering target {:?}, stopping gathering", gathering.target);
                commands.entity(entity).remove::<Gathering>();
                continue;
            };

            // Add resource to inventory (1 unit per gather cycle)
            let overflow = inventory.add_resource(resource_type, 1, settings.max_stack_size);
            info!("Added 1 {:?} to inventory of entity {:?}. Overflow: {}",
                  resource_type, entity, overflow);

            // Reset progress or stop if inventory is full
            if overflow == 0 {
                gathering.progress = 0.0;
                info!("Continuing to gather at {:?}", gathering.target);
            } else {
                // Inventory full, stop gathering
                info!("Inventory full, stopping gathering");
                commands.entity(entity).remove::<Gathering>();
            }
        }
    }
}

// Add more logging to the start_gathering function
fn start_gathering(
    mut commands: Commands,
    mouse_button: Res<ButtonInput<MouseButton>>,
    windows: Query<&Window>,
    camera_q: Query<(&Camera, &GlobalTransform)>,
    selected_workers: Query<Entity, (With<Selected>, With<Worker>)>,
    resource_nodes: Query<(Entity, &GlobalTransform, &Sprite), Or<(With<Tree>, With<Mine>, With<Quarry>)>>,
    trees: Query<Entity, With<Tree>>,
    mines: Query<Entity, With<Mine>>,
    _quarries: Query<Entity, With<Quarry>>,
) {
    // Only process right-clicks
    if !mouse_button.just_pressed(MouseButton::Right) {
        return;
    }

    info!("Right-click detected");

    // Check if we have a selected worker
    let Some(worker_entity) = selected_workers.iter().next() else {
        info!("No worker selected");
        return;
    };

    info!("Worker {:?} is selected", worker_entity);

    // Get click position
    let window = windows.single();
    let Some(cursor_position) = window.cursor_position() else {
        info!("Could not get cursor position");
        return;
    };

    // Get camera transform
    let (camera, camera_transform) = camera_q.single();
    let Ok(cursor_ray) = camera.viewport_to_world(camera_transform, cursor_position) else {
        info!("Could not convert cursor position to world coordinates");
        return;
    };

    let cursor_pos = cursor_ray.origin.truncate();
    info!("Clicked at world position: {:?}", cursor_pos);

    // Check if we clicked on a resource node
    for (node_entity, transform, sprite) in &resource_nodes {
        // Get sprite size
        let size = sprite.custom_size.unwrap_or(Vec2::new(64.0, 64.0));

        // Simple AABB collision detection
        let pos = transform.translation().truncate();
        info!("Resource node {:?} at position: {:?} with size: {:?}", node_entity, pos, size);

        let min_x = pos.x - size.x / 2.0;
        let max_x = pos.x + size.x / 2.0;
        let min_y = pos.y - size.y / 2.0;
        let max_y = pos.y + size.y / 2.0;

        if cursor_pos.x >= min_x && cursor_pos.x <= max_x &&
           cursor_pos.y >= min_y && cursor_pos.y <= max_y {
            info!("Clicked on resource node {:?}", node_entity);

            // Determine resource type
            let resource_type = if trees.contains(node_entity) {
                info!("It's a tree");
                ResourceType::Wood
            } else if mines.contains(node_entity) {
                info!("It's a mine");
                ResourceType::Gold
            } else {
                info!("It's a quarry");
                ResourceType::Stone
            };

            // Start gathering
            info!("Starting gathering for worker {:?} at resource node {:?}", worker_entity, node_entity);
            commands.entity(worker_entity).insert(Gathering {
                resource_type,
                progress: 0.0,
                target: node_entity,
            });

            break;
        }
    }
}
