use bevy::prelude::*;
use crate::components::inventory::*;
use crate::components::unit::Selected;
use crate::components::unit::Selectable;
use crate::components::movement::{Movable, MoveTarget, Moving};
use bevy_ecs_ldtk::prelude::GridCoords;
use crate::components::ui::EntityInfoPanel;
use crate::components::skills::{Skills, SkillProgression};
use crate::entities::{Tree, Mine, Quarry};

pub struct ResourceGatheringPlugin;

impl Plugin for ResourceGatheringPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, gather_resources)
           .add_systems(Update, start_gathering)
           .add_systems(Update, check_gathering_proximity)
           .add_systems(Update, update_skills_from_activities)
           .add_systems(Update, update_character_info_ui)
           .add_systems(Update, handle_resource_transfer);
    }
}

// Component to track gathering progress
#[derive(Component, Debug)]
pub struct Gathering {
    pub resource_type: ResourceType,
    pub progress: f32,
    pub target: Entity,
    pub base_time: f32,
    pub skill_modifier: f32,
}

// New component to track gathering intent
#[derive(Component, Debug)]
pub struct GatheringIntent {
    pub target: Entity,
    pub resource_type: ResourceType,
}

// Simplified character marker
#[derive(Component, Debug, Default)]
pub struct Character;

// Character bundle - move this to entities.rs
#[derive(Bundle)]
pub struct CharacterBundle {
    pub character: Character,
    pub skills: Skills,
    pub skill_progression: SkillProgression,
    pub selectable: Selectable,
    pub movable: Movable,
    pub move_target: MoveTarget,
    pub inventory: Inventory,
    pub inventory_settings: InventorySettings,
}

// Update the gathering process to use target and skill_modifier
fn gather_resources(
    mut commands: Commands,
    time: Res<Time>,
    mut gatherers: Query<(Entity, &mut Gathering, &mut Inventory, &InventorySettings, &Skills)>,
    mut skill_progression: Query<&mut SkillProgression>,
    trees: Query<Entity, With<Tree>>,
    mines: Query<Entity, With<Mine>>,
    quarries: Query<Entity, With<Quarry>>,
) {
    for (entity, mut gathering, mut inventory, settings, skills) in &mut gatherers {
        let target_exists = trees.contains(gathering.target) ||
                            mines.contains(gathering.target) ||
                            quarries.contains(gathering.target);

        if !target_exists {
            commands.entity(entity).remove::<Gathering>();
            continue;
        }

        let progress_rate = gathering.skill_modifier * time.delta_secs();
        gathering.progress += progress_rate;

        if gathering.progress >= gathering.base_time {
            let resource_type = gathering.resource_type;

            let skill_value = match resource_type {
                ResourceType::Wood => skills.woodcutting,
                ResourceType::Gold => skills.mining,
                ResourceType::Stone => skills.harvesting,
            };

            let base_yield = 1;
            let bonus_yield = (skill_value / 3.0).floor() as u32;
            let total_yield = base_yield + bonus_yield;

            let overflow = inventory.add_resource(resource_type, total_yield, settings.max_stack_size);

            if let Ok(mut progression) = skill_progression.get_mut(entity) {
                match resource_type {
                    ResourceType::Wood => progression.woodcutting_xp += 5.0,
                    ResourceType::Gold => progression.mining_xp += 5.0,
                    ResourceType::Stone => progression.harvesting_xp += 5.0,
                }
            }

            if overflow == 0 {
                gathering.progress = 0.0;
                info!("Gathered {} {:?}", total_yield, resource_type);
            } else {
                info!("Inventory full, stopping gathering");
                commands.entity(entity).remove::<Gathering>();
            }
        }
    }
}

// This system adds a GatheringIntent component and sets up movement to the resource
fn start_gathering(
    mut commands: Commands,
    mouse_button: Res<ButtonInput<MouseButton>>,
    windows: Query<&Window>,
    camera_q: Query<(&Camera, &GlobalTransform)>,
    selected_characters: Query<(Entity, &Skills, &GridCoords), With<Selected>>,
    mut move_targets: Query<&mut MoveTarget>,
    resource_nodes: Query<(Entity, &GlobalTransform, &Sprite, Option<&Tree>, Option<&Mine>, Option<&Quarry>)>,
    gathering_intent_query: Query<&GatheringIntent>,
) {
    if !mouse_button.just_pressed(MouseButton::Right) {
        return;
    }

    let window = windows.single();
    let Some(cursor_position) = window.cursor_position() else {
        return;
    };

    let (camera, camera_transform) = camera_q.single();
    let Ok(cursor_ray) = camera.viewport_to_world(camera_transform, cursor_position) else {
        return;
    };

    let cursor_pos = cursor_ray.origin.truncate();

    let Some((character_entity, skills, character_coords)) = selected_characters.iter().next() else {
        return;
    };

    for (node_entity, transform, sprite, is_tree, is_mine, is_quarry) in &resource_nodes {
        let size = sprite.custom_size.unwrap_or(Vec2::new(64.0, 64.0));
        let pos = transform.translation().truncate();

        let min_x = pos.x - size.x / 2.0;
        let max_x = pos.x + size.x / 2.0;
        let min_y = pos.y - size.y / 2.0;
        let max_y = pos.y + size.y / 2.0;

        if cursor_pos.x >= min_x && cursor_pos.x <= max_x &&
           cursor_pos.y >= min_y && cursor_pos.y <= max_y {

            let (resource_type, resource_name) = if is_tree.is_some() {
                (ResourceType::Wood, "wood from tree")
            } else if is_mine.is_some() {
                (ResourceType::Gold, "gold from mine")
            } else if is_quarry.is_some() {
                (ResourceType::Stone, "stone from quarry")
            } else {
                (ResourceType::Stone, "unknown resource")
            };

            let _skill_value = match resource_type {
                ResourceType::Wood => skills.woodcutting,
                ResourceType::Gold => skills.mining,
                ResourceType::Stone => skills.harvesting,
            };

            if let Ok(gathering_intent) = gathering_intent_query.get(character_entity) {
                if gathering_intent.target == node_entity {
                    return;
                }
            }

            commands.entity(character_entity).insert(GatheringIntent {
                target: node_entity,
                resource_type,
            });

            commands.entity(character_entity).remove::<Gathering>();

            if let Ok(mut move_target) = move_targets.get_mut(character_entity) {
                let resource_grid = GridCoords {
                    x: (pos.x / 64.0).round() as i32,
                    y: (pos.y / 64.0).round() as i32,
                };

                let character_grid = character_coords;

                // Calculate approach positions (adjacent cells around the resource)
                let approach_positions = [
                    GridCoords { x: resource_grid.x - 1, y: resource_grid.y },     // Left
                    GridCoords { x: resource_grid.x + 1, y: resource_grid.y },     // Right
                    GridCoords { x: resource_grid.x, y: resource_grid.y - 1 },     // Below
                    GridCoords { x: resource_grid.x, y: resource_grid.y + 1 },     // Above
                    GridCoords { x: resource_grid.x - 1, y: resource_grid.y - 1 }, // Bottom-left
                    GridCoords { x: resource_grid.x + 1, y: resource_grid.y - 1 }, // Bottom-right
                    GridCoords { x: resource_grid.x - 1, y: resource_grid.y + 1 }, // Top-left
                    GridCoords { x: resource_grid.x + 1, y: resource_grid.y + 1 }, // Top-right
                ];

                // Find the closest approach position
                let destination = approach_positions
                    .iter()
                    .min_by_key(|pos| {
                        let dx = pos.x - character_grid.x;
                        let dy = pos.y - character_grid.y;
                        (dx * dx + dy * dy) as u32  // Squared distance
                    })
                    .unwrap_or(&resource_grid);  // Fallback to resource position if no approach found

                move_target.destination = Some(*destination);
                move_target.path.clear();

                info!("Setting movement destination to {:?} to approach resource at {:?}",
                    destination, resource_grid);
            }

            info!("Moving to gather {}", resource_name);
            break;
        }
    }
}

// This system checks if characters with GatheringIntent are close enough to start gathering
fn check_gathering_proximity(
    mut commands: Commands,
    characters: Query<(Entity, &GlobalTransform, &GatheringIntent, &Skills), (Without<Gathering>, Without<Moving>)>,
    resources: Query<&GlobalTransform>,
) {
    const GATHERING_RANGE: f32 = 150.0;

    for (entity, transform, intent, skills) in &characters {
        if let Ok(resource_transform) = resources.get(intent.target) {
            let distance = transform.translation().distance(resource_transform.translation());

            if distance <= GATHERING_RANGE {
                let skill_value = match intent.resource_type {
                    ResourceType::Wood => skills.woodcutting,
                    ResourceType::Gold => skills.mining,
                    ResourceType::Stone => skills.harvesting,
                };

                commands.entity(entity).insert(Gathering {
                    resource_type: intent.resource_type,
                    progress: 0.0,
                    target: intent.target,
                    base_time: 3.0,
                    skill_modifier: skill_value,
                });

                commands.entity(entity).remove::<GatheringIntent>();

                let resource_name = match intent.resource_type {
                    ResourceType::Wood => "wood from tree",
                    ResourceType::Gold => "gold from mine",
                    ResourceType::Stone => "stone from quarry",
                };

                info!("Started gathering {}", resource_name);
            } else {
                info!("Too far from resource: {:.1} units away (need {:.1})", distance, GATHERING_RANGE);
            }
        }
    }
}

fn update_skills_from_activities(
    mut characters: Query<(&mut Skills, &mut SkillProgression)>,
    gatherers: Query<(Entity, &Gathering)>,
    time: Res<Time>,
) {
    for (entity, gathering) in &gatherers {
        if let Ok((mut skills, mut progression)) = characters.get_mut(entity) {
            match gathering.resource_type {
                ResourceType::Wood => {
                    progression.woodcutting_xp += time.delta_secs() * 0.2;
                    if progression.woodcutting_xp >= 100.0 * skills.woodcutting {
                        progression.woodcutting_xp = 0.0;
                        skills.woodcutting += 0.1;
                        info!("Character {:?} improved woodcutting to {:.1}", entity, skills.woodcutting);
                    }
                },
                ResourceType::Gold => {
                    progression.mining_xp += time.delta_secs() * 0.2;
                    if progression.mining_xp >= 100.0 * skills.mining {
                        progression.mining_xp = 0.0;
                        skills.mining += 0.1;
                        info!("Character {:?} improved mining to {:.1}", entity, skills.mining);
                    }
                },
                ResourceType::Stone => {
                    progression.harvesting_xp += time.delta_secs() * 0.2;
                    if progression.harvesting_xp >= 100.0 * skills.harvesting {
                        progression.harvesting_xp = 0.0;
                        skills.harvesting += 0.1;
                        info!("Character {:?} improved harvesting to {:.1}", entity, skills.harvesting);
                    }
                },
            }
        }
    }
}

fn update_character_info_ui(
    selected_entities: Query<(Entity, &Skills, Option<&Inventory>), With<Selected>>,
    panel_query: Query<Entity, With<EntityInfoPanel>>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    gathering_query: Query<&Gathering>,
    gathering_intent_query: Query<&GatheringIntent>,
) {
    if let Ok(panel_entity) = panel_query.get_single() {
        commands.entity(panel_entity).despawn_descendants();

        if let Ok((entity, skills, inventory)) = selected_entities.get_single() {
            commands.entity(panel_entity).with_children(|parent| {
                parent.spawn((
                    Text::new("Character Info"),
                    TextFont {
                        font: asset_server.load("fonts/fira_sans/FiraSans-Bold.ttf"),
                        font_size: 18.0,
                        ..default()
                    },
                    TextColor(Color::WHITE),
                ));

                parent.spawn((
                    Text::new("Skills:"),
                    TextFont {
                        font: asset_server.load("fonts/fira_sans/FiraSans-Bold.ttf"),
                        font_size: 16.0,
                        ..default()
                    },
                    TextColor(Color::WHITE),
                ));

                parent.spawn((
                    Text::new(format!("Mining: {:.1}", skills.mining)),
                    TextFont { font: asset_server.load("fonts/fira_sans/FiraSans-Bold.ttf"), font_size: 14.0, ..default() },
                    TextColor(Color::WHITE),
                ));

                parent.spawn((
                    Text::new(format!("Woodcutting: {:.1}", skills.woodcutting)),
                    TextFont { font: asset_server.load("fonts/fira_sans/FiraSans-Bold.ttf"), font_size: 14.0, ..default() },
                    TextColor(Color::WHITE),
                ));

                parent.spawn((
                    Text::new(format!("Harvesting: {:.1}", skills.harvesting)),
                    TextFont { font: asset_server.load("fonts/fira_sans/FiraSans-Bold.ttf"), font_size: 14.0, ..default() },
                    TextColor(Color::WHITE),
                ));

                if let Ok(gathering) = gathering_query.get(entity) {
                    let progress_percent = (gathering.progress / gathering.base_time) * 100.0;
                    let resource_name = match gathering.resource_type {
                        ResourceType::Gold => "Gold",
                        ResourceType::Wood => "Wood",
                        ResourceType::Stone => "Stone",
                    };

                    parent.spawn((
                        Text::new(format!("Gathering {}: {:.1}%", resource_name, progress_percent)),
                        TextFont {
                            font: asset_server.load("fonts/fira_sans/FiraSans-Bold.ttf"),
                            font_size: 14.0,
                            ..default()
                        },
                        TextColor(Color::srgb(0.0, 1.0, 0.0)),
                    ));
                } else if let Ok(intent) = gathering_intent_query.get(entity) {
                    let resource_name = match intent.resource_type {
                        ResourceType::Gold => "Gold",
                        ResourceType::Wood => "Wood",
                        ResourceType::Stone => "Stone",
                    };

                    parent.spawn((
                        Text::new(format!("Moving to gather {}", resource_name)),
                        TextFont {
                            font: asset_server.load("fonts/fira_sans/FiraSans-Bold.ttf"),
                            font_size: 14.0,
                            ..default()
                        },
                        TextColor(Color::srgb(1.0, 1.0, 0.0)),
                    ));
                }

                if let Some(inv) = inventory {
                    let used_slots = inv.slots.iter().filter(|slot| slot.is_some()).count();
                    let total_slots = inv.max_slots;

                    parent.spawn((
                        Text::new(format!("Inventory ({}/{})", used_slots, total_slots)),
                        TextFont {
                            font: asset_server.load("fonts/fira_sans/FiraSans-Bold.ttf"),
                            font_size: 16.0,
                            ..default()
                        },
                        TextColor(Color::WHITE),
                    ));

                    for (i, slot) in inv.slots.iter().enumerate() {
                        if let Some(inv_slot) = slot {
                            let resource_name = match inv_slot.resource_type {
                                ResourceType::Gold => "Gold",
                                ResourceType::Wood => "Wood",
                                ResourceType::Stone => "Stone",
                            };

                            let resource_icon = match inv_slot.resource_type {
                                ResourceType::Gold => "🪙",
                                ResourceType::Wood => "🪵",
                                ResourceType::Stone => "🪨",
                            };

                            parent.spawn((
                                Text::new(format!("{} {} x{}", resource_icon, resource_name, inv_slot.quantity)),
                                TextFont { font: asset_server.load("fonts/fira_sans/FiraSans-Bold.ttf"), font_size: 14.0, ..default() },
                                TextColor(Color::WHITE),
                            ));
                        } else {
                            parent.spawn((
                                Text::new(format!("Slot {}: Empty", i + 1)),
                                TextFont { font: asset_server.load("fonts/fira_sans/FiraSans-Bold.ttf"), font_size: 14.0, ..default() },
                                TextColor(Color::srgba(0.7, 0.7, 0.7, 1.0)),
                            ));
                        }
                    }
                } else {
                    parent.spawn((
                        Text::new("No inventory available"),
                        TextFont {
                            font: asset_server.load("fonts/fira_sans/FiraSans-Bold.ttf"),
                            font_size: 14.0,
                            ..default()
                        },
                        TextColor(Color::srgba(0.9, 0.3, 0.3, 1.0)),
                    ));
                }
            });
        }
    }
}

fn handle_resource_transfer(
    _commands: Commands,
    keyboard: Res<ButtonInput<KeyCode>>,
    mouse_button: Res<ButtonInput<MouseButton>>,
    mut selected_entity: Query<(Entity, &mut Inventory, &InventorySettings), With<Selected>>,
    mut entities_with_inventory: Query<(Entity, &mut Inventory, &InventorySettings, &GlobalTransform), Without<Selected>>,
    windows: Query<&Window>,
    camera_q: Query<(&Camera, &GlobalTransform)>,
) {
    if keyboard.pressed(KeyCode::KeyT) && mouse_button.just_pressed(MouseButton::Right) {
        if let Ok((_selected_entity, mut selected_inventory, _selected_settings)) = selected_entity.get_single_mut() {
            let window = windows.single();
            if let Some(cursor_position) = window.cursor_position() {
                let (camera, camera_transform) = camera_q.single();
                if let Ok(cursor_ray) = camera.viewport_to_world(camera_transform, cursor_position) {
                    let cursor_pos = cursor_ray.origin.truncate();
                    for (entity, mut inventory, settings, transform) in &mut entities_with_inventory {
                        let entity_pos = transform.translation().truncate();
                        let distance = cursor_pos.distance(entity_pos);
                        if distance < 100.0 {
                            if selected_inventory.count_resource(ResourceType::Wood) > 0 {
                                let amount = selected_inventory.transfer_to(
                                    &mut inventory,
                                    ResourceType::Wood,
                                    1,
                                    settings.max_stack_size
                                );
                                info!("Transferred {} Wood to entity {:?}", amount, entity);
                                if amount > 0 {
                                    break;
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
