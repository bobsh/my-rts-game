use bevy::prelude::*;
use crate::components::skills::{Skills, SkillProgression};
use crate::components::inventory::{Inventory, ResourceType, InventorySettings};
use crate::components::unit::Selected;

pub struct ConstructionPlugin;

impl Plugin for ConstructionPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, start_construction)
           .add_systems(Update, process_construction)
           .add_systems(Update, update_construction_ui);
    }
}

// Component to track construction progress
#[derive(Component, Debug)]
pub struct Constructing {
    pub building_type: BuildingType,
    pub progress: f32,
    pub required_time: f32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BuildingType {
    House,
    Workshop,
    Wall,
}

impl BuildingType {
    pub fn get_cost(&self) -> Vec<(ResourceType, u32)> {
        match self {
            BuildingType::House => vec![
                (ResourceType::Wood, 5),
                (ResourceType::Stone, 3),
            ],
            BuildingType::Workshop => vec![
                (ResourceType::Wood, 10),
                (ResourceType::Stone, 5),
                (ResourceType::Gold, 2),
            ],
            BuildingType::Wall => vec![
                (ResourceType::Stone, 5),
            ],
        }
    }
}

// Start construction when pressing B key and clicking
fn start_construction(
    mut commands: Commands,
    keyboard: Res<ButtonInput<KeyCode>>,
    mouse_button: Res<ButtonInput<MouseButton>>,
    selected_builders: Query<(Entity, &Skills, &mut Inventory, &InventorySettings), With<Selected>>,
    windows: Query<&Window>,
    camera_q: Query<(&Camera, &GlobalTransform)>,
) {
    if keyboard.pressed(KeyCode::KeyB) && mouse_button.just_pressed(MouseButton::Left) {
        // Get the selected builder
        if let Ok((builder_entity, skills, mut inventory, settings)) = selected_builders.get_single_mut() {
            // Get construction skill
            let construction_skill = skills.construction;

            // Get cursor position for placement
            let window = windows.single();
            if let Some(cursor_position) = window.cursor_position() {
                let (camera, camera_transform) = camera_q.single();
                if let Ok(cursor_ray) = camera.viewport_to_world(camera_transform, cursor_position) {
                    let cursor_pos = cursor_ray.origin.truncate();

                    // Determine building type (using keyboard modifier keys)
                    let building_type = if keyboard.pressed(KeyCode::ShiftLeft) {
                        BuildingType::Workshop
                    } else if keyboard.pressed(KeyCode::ControlLeft) {
                        BuildingType::Wall
                    } else {
                        BuildingType::House
                    };

                    // Check if builder has required resources
                    let cost = building_type.get_cost();
                    let has_resources = cost.iter().all(|(resource_type, amount)| {
                        inventory.count_resource(*resource_type) >= *amount
                    });

                    if has_resources {
                        // Consume resources
                        for (resource_type, amount) in cost {
                            inventory.remove_resource(resource_type, amount);
                        }

                        // Start construction - faster with higher skill
                        let base_time = 10.0;
                        let skill_modifier = 0.7 + (0.3 * construction_skill); // 1.0 skill = normal, 5.0 = twice as fast
                        let required_time = base_time / skill_modifier;

                        commands.entity(builder_entity).insert(Constructing {
                            building_type,
                            progress: 0.0,
                            required_time,
                        });

                        info!("Started construction of {:?}", building_type);
                    } else {
                        info!("Not enough resources for {:?}", building_type);
                    }
                }
            }
        }
    }
}

// Process ongoing construction
fn process_construction(
    mut commands: Commands,
    time: Res<Time>,
    mut builders: Query<(Entity, &mut Constructing, &mut Skills, &mut SkillProgression)>,
    asset_server: Res<AssetServer>,
) {
    for (entity, mut constructing, mut skills, mut progression) in &mut builders {
        constructing.progress += time.delta_secs();

        // Construction complete
        if constructing.progress >= constructing.required_time {
            // Spawn the building at builder location
            // (This is a simplified version - you'd want to use the cursor position from before)

            info!("Construction of {:?} complete!", constructing.building_type);

            // Gain construction XP
            progression.construction_xp += 10.0;
            if progression.construction_xp >= 100.0 * skills.construction {
                progression.construction_xp = 0.0;
                skills.construction += 0.1;
                info!("Character improved construction to {:.1}", skills.construction);
            }

            // Remove construction component
            commands.entity(entity).remove::<Constructing>();
        }
    }
}

// Update construction UI
fn update_construction_ui(
    construction_query: Query<(Entity, &Constructing), With<Selected>>,
    mut panel_query: Query<Entity, With<EntityInfoPanel>>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    if let Ok(panel_entity) = panel_query.get_single() {
        if let Ok((entity, constructing)) = construction_query.get_single() {
            let progress_percent = (constructing.progress / constructing.required_time) * 100.0;

            commands.entity(panel_entity).with_children(|parent| {
                parent.spawn((
                    Text::new(format!("Building {:?}: {:.1}%",
                        constructing.building_type,
                        progress_percent
                    )),
                    TextFont {
                        font: asset_server.load("fonts/fira_sans/FiraSans-Bold.ttf"),
                        font_size: 16.0,
                        ..default()
                    },
                    TextColor(Color::WHITE),
                ));
            });
        }
    }
}
