use bevy::prelude::*;
use crate::components::inventory::*;
use crate::components::unit::Selected;
use crate::components::unit::Selectable;
use crate::components::movement::{Movable, MoveTarget, Moving};
use bevy_ecs_ldtk::prelude::GridCoords; // Import GridCoords directly
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
    _quarries: Query<Entity, With<Quarry>>,
) {
    for (entity, mut gathering, mut inventory, settings, skills) in &mut gatherers {
        let target_exists = trees.contains(gathering.target) ||
                            mines.contains(gathering.target) ||
                            _quarries.contains(gathering.target);

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

            let (resource_type, resource_name) = if is_tree.is_some() {e() {
                (ResourceType::Wood, "wood from tree")
            } else if is_mine.is_some() {
                (ResourceType::Gold, "gold from mine")d from mine")
            } else if is_quarry.is_some() {
                (ResourceType::Stone, "stone from quarry")e from quarry")
            } else {
                (ResourceType::Stone, "unknown resource")ourceType::Stone, "unknown resource")
            };

            let _skill_value = match resource_type {            let _skill_value = match resource_type {
                ResourceType::Wood => skills.woodcutting,ting,
                ResourceType::Gold => skills.mining,
                ResourceType::Stone => skills.harvesting,ting,
            };

            if let Ok(gathering_intent) = gathering_intent_query.get(character_entity) {            if let Ok(gathering_intent) = gathering_intent_query.get(character_entity) {
                if gathering_intent.target == node_entity {
                    return;
                }
            }

            commands.entity(character_entity).insert(GatheringIntent {            commands.entity(character_entity).insert(GatheringIntent {
                target: node_entity,
                resource_type,
            });

            commands.entity(character_entity).remove::<Gathering>();            commands.entity(character_entity).remove::<Gathering>();

            if let Ok(mut move_target) = move_targets.get_mut(character_entity) {            if let Ok(mut move_target) = move_targets.get_mut(character_entity) {
                let resource_grid = GridCoords {
                    x: (pos.x / 64.0).round() as i32,
                    y: (pos.y / 64.0).round() as i32,
                };

                let character_grid = character_coords;                let approach_positions = [
rce_grid.x - 1, y: resource_grid.y },
                let approach_positions = [       GridCoords { x: resource_grid.x + 1, y: resource_grid.y },
                    GridCoords { x: resource_grid.x - 1, y: resource_grid.y },                    GridCoords { x: resource_grid.x, y: resource_grid.y - 1 },
                    GridCoords { x: resource_grid.x + 1, y: resource_grid.y },esource_grid.y + 1 },
                    GridCoords { x: resource_grid.x, y: resource_grid.y - 1 }, y: resource_grid.y - 1 },
                    GridCoords { x: resource_grid.x, y: resource_grid.y + 1 },                    GridCoords { x: resource_grid.x + 1, y: resource_grid.y - 1 },
                    GridCoords { x: resource_grid.x - 1, y: resource_grid.y - 1 },  GridCoords { x: resource_grid.x - 1, y: resource_grid.y + 1 },
                    GridCoords { x: resource_grid.x + 1, y: resource_grid.y - 1 },           GridCoords { x: resource_grid.x + 1, y: resource_grid.y + 1 },
                    GridCoords { x: resource_grid.x - 1, y: resource_grid.y + 1 },           ];
                    GridCoords { x: resource_grid.x + 1, y: resource_grid.y + 1 },
                ];                move_target.destination = Some(resource_grid);

                let destination = approach_positionsath.clear();
                    .iter()
                    .min_by_key(|pos| {
                        let dx = pos.x - character_grid.x;
                        let dy = pos.y - character_grid.y;
                        (dx * dx + dy * dy) as u32);
                    })
                    .unwrap_or(&resource_grid);            break;

                move_target.destination = Some(*destination);
                move_target.path.clear();

                info!("Setting movement destination to {:?} to approach resource at {:?}", heringIntent are close enough to start gathering
                    destination, resource_grid);
            }
ingIntent, &Skills), (Without<Gathering>, Without<Moving>)>,
            info!("Moving to gather {}", resource_name);

            break;    const GATHERING_RANGE: f32 = 150.0;
        }
    }rs {
}rm) = resources.get(intent.target) {
lation().distance(resource_transform.translation());
// This system checks if characters with GatheringIntent are close enough to start gathering
fn check_gathering_proximity(
    mut commands: Commands, skill_value = match intent.resource_type {
    characters: Query<(Entity, &GlobalTransform, &GatheringIntent, &Skills), (Without<Gathering>, Without<Moving>)>,                    ResourceType::Wood => skills.woodcutting,
    resources: Query<&GlobalTransform>,
) {                    ResourceType::Stone => skills.harvesting,
    const GATHERING_RANGE: f32 = 150.0;

    for (entity, transform, intent, skills) in &characters {
        if let Ok(resource_transform) = resources.get(intent.target) {
            let distance = transform.translation().distance(resource_transform.translation());  progress: 0.0,
                    target: intent.target,
            if distance <= GATHERING_RANGE {
                let skill_value = match intent.resource_type {skill_modifier: skill_value,
                    ResourceType::Wood => skills.woodcutting,
                    ResourceType::Gold => skills.mining,
                    ResourceType::Stone => skills.harvesting,       commands.entity(entity).remove::<GatheringIntent>();
                };
               let resource_name = match intent.resource_type {
                commands.entity(entity).insert(Gathering {                    ResourceType::Wood => "wood from tree",
                    resource_type: intent.resource_type,:Gold => "gold from mine",
                    progress: 0.0,
                    target: intent.target,
                    base_time: 3.0,
                    skill_modifier: skill_value,             info!("Started gathering {}", resource_name);
                });
istance, GATHERING_RANGE);
                commands.entity(entity).remove::<GatheringIntent>();

                let resource_name = match intent.resource_type {
                    ResourceType::Wood => "wood from tree",
                    ResourceType::Gold => "gold from mine",
                    ResourceType::Stone => "stone from quarry",
                };
(Entity, &Gathering)>,
                info!("Started gathering {}", resource_name);>,
            } else {
                info!("Too far from resource: {:.1} units away (need {:.1})", distance, GATHERING_RANGE);
            }ntity) {
        }
    }
}
f progression.woodcutting_xp >= 100.0 * skills.woodcutting {
fn update_skills_from_activities(      progression.woodcutting_xp = 0.0;
    mut characters: Query<(&mut Skills, &mut SkillProgression)>,ng += 0.1;
    gatherers: Query<(Entity, &Gathering)>,1}", entity, skills.woodcutting);
    time: Res<Time>,
) {
    for (entity, gathering) in &gatherers {
        if let Ok((mut skills, mut progression)) = characters.get_mut(entity) {
            match gathering.resource_type {f progression.mining_xp >= 100.0 * skills.mining {
                ResourceType::Wood => {      progression.mining_xp = 0.0;
                    progression.woodcutting_xp += time.delta_secs() * 0.2;           skills.mining += 0.1;
                    if progression.woodcutting_xp >= 100.0 * skills.woodcutting {               info!("Character {:?} improved mining to {:.1}", entity, skills.mining);
                        progression.woodcutting_xp = 0.0;               }
                        skills.woodcutting += 0.1;               },
                        info!("Character {:?} improved woodcutting to {:.1}", entity, skills.woodcutting);                ResourceType::Stone => {
                    }ion.harvesting_xp += time.delta_secs() * 0.2;
                },
                ResourceType::Gold => {0;
                    progression.mining_xp += time.delta_secs() * 0.2;lls.harvesting += 0.1;
                    if progression.mining_xp >= 100.0 * skills.mining {acter {:?} improved harvesting to {:.1}", entity, skills.harvesting);
                        progression.mining_xp = 0.0;
                        skills.mining += 0.1;
                        info!("Character {:?} improved mining to {:.1}", entity, skills.mining);         }
                    }
                },
                ResourceType::Stone => {}
                    progression.harvesting_xp += time.delta_secs() * 0.2;
                    if progression.harvesting_xp >= 100.0 * skills.harvesting {
                        progression.harvesting_xp = 0.0;Entity, &Skills, Option<&Inventory>), With<Selected>>,
                        skills.harvesting += 0.1;nel>>,
                        info!("Character {:?} improved harvesting to {:.1}", entity, skills.harvesting);
                    }
                },
            }GatheringIntent>,
        }
    }t_single() {
}tity(panel_entity).despawn_descendants();

fn update_character_info_ui(lls, inventory)) = selected_entities.get_single() {
    selected_entities: Query<(Entity, &Skills, Option<&Inventory>), With<Selected>>,.with_children(|parent| {
    panel_query: Query<Entity, With<EntityInfoPanel>>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    gathering_query: Query<&Gathering>,_server.load("fonts/fira_sans/FiraSans-Bold.ttf"),
    gathering_intent_query: Query<&GatheringIntent>,  font_size: 18.0,
) {
    if let Ok(panel_entity) = panel_query.get_single() { },
        commands.entity(panel_entity).despawn_descendants();                    TextColor(Color::WHITE),

        if let Ok((entity, skills, inventory)) = selected_entities.get_single() {
            commands.entity(panel_entity).with_children(|parent| {
                parent.spawn((
                    Text::new("Character Info"), TextFont {
                    TextFont {                        font: asset_server.load("fonts/fira_sans/FiraSans-Bold.ttf"),
                        font: asset_server.load("fonts/fira_sans/FiraSans-Bold.ttf"),ize: 16.0,
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
                    TextColor(Color::WHITE),/FiraSans-Bold.ttf"), font_size: 14.0, ..default() },
                ));

                parent.spawn((
                    Text::new(format!("Mining: {:.1}", skills.mining)),.spawn((
                    TextFont { font: asset_server.load("fonts/fira_sans/FiraSans-Bold.ttf"), font_size: 14.0, ..default() },                    Text::new(format!("Harvesting: {:.1}", skills.harvesting)),
                    TextColor(Color::WHITE),t: asset_server.load("fonts/fira_sans/FiraSans-Bold.ttf"), font_size: 14.0, ..default() },
                ));

                parent.spawn((
                    Text::new(format!("Woodcutting: {:.1}", skills.woodcutting)),ring_query.get(entity) {
                    TextFont { font: asset_server.load("fonts/fira_sans/FiraSans-Bold.ttf"), font_size: 14.0, ..default() },t = (gathering.progress / gathering.base_time) * 100.0;
                    TextColor(Color::WHITE),source_name = match gathering.resource_type {
                ));
 ResourceType::Wood => "Wood",
                parent.spawn((
                    Text::new(format!("Harvesting: {:.1}", skills.harvesting)),
                    TextFont { font: asset_server.load("fonts/fira_sans/FiraSans-Bold.ttf"), font_size: 14.0, ..default() },
                    TextColor(Color::WHITE),
                ));: {:.1}%", resource_name, progress_percent)),
  TextFont {
                if let Ok(gathering) = gathering_query.get(entity) {                            font: asset_server.load("fonts/fira_sans/FiraSans-Bold.ttf"),
                    let progress_percent = (gathering.progress / gathering.base_time) * 100.0;ize: 14.0,
                    let resource_name = match gathering.resource_type {
                        ResourceType::Gold => "Gold",
                        ResourceType::Wood => "Wood",
                        ResourceType::Stone => "Stone",
                    };) = gathering_intent_query.get(entity) {
source_name = match intent.resource_type {
                    parent.spawn((
                        Text::new(format!("Gathering {}: {:.1}%", resource_name, progress_percent)), ResourceType::Wood => "Wood",
                        TextFont {       ResourceType::Stone => "Stone",
                            font: asset_server.load("fonts/fira_sans/FiraSans-Bold.ttf"),                    };
                            font_size: 14.0,
                            ..default()
                        }, gather {}", resource_name)),
                        TextColor(Color::srgb(0.0, 1.0, 0.0)),                        TextFont {
                    ));asset_server.load("fonts/fira_sans/FiraSans-Bold.ttf"),
                } else if let Ok(intent) = gathering_intent_query.get(entity) {
                    let resource_name = match intent.resource_type {ult()
                        ResourceType::Gold => "Gold",
                        ResourceType::Wood => "Wood",b(1.0, 1.0, 0.0)),
                        ResourceType::Stone => "Stone",
                    };

                    parent.spawn((Some(inv) = inventory {
                        Text::new(format!("Moving to gather {}", resource_name)),                    let used_slots = inv.slots.iter().filter(|slot| slot.is_some()).count();
                        TextFont {
                            font: asset_server.load("fonts/fira_sans/FiraSans-Bold.ttf"),
                            font_size: 14.0,
                            ..default(), used_slots, total_slots)),
                        },
                        TextColor(Color::srgb(1.0, 1.0, 0.0)),_sans/FiraSans-Bold.ttf"),
                    ));nt_size: 16.0,
                }                            ..default()

                if let Some(inv) = inventory {
                    let used_slots = inv.slots.iter().filter(|slot| slot.is_some()).count();
                    let total_slots = inv.max_slots;
ot) in inv.slots.iter().enumerate() {
                    parent.spawn((                        if let Some(inv_slot) = slot {
                        Text::new(format!("Inventory ({}/{})", used_slots, total_slots)),ame = match inv_slot.resource_type {
                        TextFont {
                            font: asset_server.load("fonts/fira_sans/FiraSans-Bold.ttf"),
                            font_size: 16.0,Stone",
                            ..default()
                        },
                        TextColor(Color::WHITE),con = match inv_slot.resource_type {
                    ));

                    for (i, slot) in inv.slots.iter().enumerate() {
                        if let Some(inv_slot) = slot {
                            let resource_name = match inv_slot.resource_type {
                                ResourceType::Gold => "Gold",       parent.spawn((
                                ResourceType::Wood => "Wood",        Text::new(format!("{} {} x{}", resource_icon, resource_name, inv_slot.quantity)),
                                ResourceType::Stone => "Stone",xtFont { font: asset_server.load("fonts/fira_sans/FiraSans-Bold.ttf"), font_size: 14.0, ..default() },
                            };

                            let resource_icon = match inv_slot.resource_type {
                                ResourceType::Gold => "🪙",
                                ResourceType::Wood => "🪵",ew(format!("Slot {}: Empty", i + 1)),
                                ResourceType::Stone => "🪨",      TextFont { font: asset_server.load("fonts/fira_sans/FiraSans-Bold.ttf"), font_size: 14.0, ..default() },
                            };, 1.0)),
     ));
                            parent.spawn((       }
                                Text::new(format!("{} {} x{}", resource_icon, resource_name, inv_slot.quantity)),     }
                                TextFont { font: asset_server.load("fonts/fira_sans/FiraSans-Bold.ttf"), font_size: 14.0, ..default() },       } else {
                                TextColor(Color::WHITE),               parent.spawn((
                            ));                       Text::new("No inventory available"),
                        } else {                        TextFont {
                            parent.spawn((font: asset_server.load("fonts/fira_sans/FiraSans-Bold.ttf"),
                                Text::new(format!("Slot {}: Empty", i + 1)),    font_size: 14.0,
                                TextFont { font: asset_server.load("fonts/fira_sans/FiraSans-Bold.ttf"), font_size: 14.0, ..default() },
                                TextColor(Color::srgba(0.7, 0.7, 0.7, 1.0)),
                            ));
                        }
                    }
                } else {
                    parent.spawn((     }
                        Text::new("No inventory available"),
                        TextFont {
                            font: asset_server.load("fonts/fira_sans/FiraSans-Bold.ttf"),
                            font_size: 14.0,
                            ..default()
                        },
                        TextColor(Color::srgba(0.9, 0.3, 0.3, 1.0)),
                    ));
                }ttings, &GlobalTransform), Without<Selected>>,
            });
        }m)>,
    }
}ton::Right) {
nventory, _selected_settings)) = selected_entity.get_single_mut() {
fn handle_resource_transfer(
    _commands: Commands,n) = window.cursor_position() {
    keyboard: Res<ButtonInput<KeyCode>>,ingle();
    mouse_button: Res<ButtonInput<MouseButton>>,ay) = camera.viewport_to_world(camera_transform, cursor_position) {
    mut selected_entity: Query<(Entity, &mut Inventory, &InventorySettings), With<Selected>>,
    mut entities_with_inventory: Query<(Entity, &mut Inventory, &InventorySettings, &GlobalTransform), Without<Selected>>, settings, transform) in &mut entities_with_inventory {
    windows: Query<&Window>,ransform.translation().truncate();
    camera_q: Query<(&Camera, &GlobalTransform)>,nce = cursor_pos.distance(entity_pos);
) {stance < 100.0 {
    if keyboard.pressed(KeyCode::KeyT) && mouse_button.just_pressed(MouseButton::Right) {   if selected_inventory.count_resource(ResourceType::Wood) > 0 {
        if let Ok((_selected_entity, mut selected_inventory, _selected_settings)) = selected_entity.get_single_mut() {           let amount = selected_inventory.transfer_to(
            let window = windows.single();                   &mut inventory,
            if let Some(cursor_position) = window.cursor_position() {                       ResourceType::Wood,
                let (camera, camera_transform) = camera_q.single();                           1,
                if let Ok(cursor_ray) = camera.viewport_to_world(camera_transform, cursor_position) {                               settings.max_stack_size
                    let cursor_pos = cursor_ray.origin.truncate();                               );
                    for (entity, mut inventory, settings, transform) in &mut entities_with_inventory {                                info!("Transferred {} Wood to entity {:?}", amount, entity);























}    }        }            }                }                    }                        }                            }                                }                                    break;                                if amount > 0 {                                info!("Transferred {} Wood to entity {:?}", amount, entity);                                );                                    settings.max_stack_size                                    1,                                    ResourceType::Wood,                                    &mut inventory,                                let amount = selected_inventory.transfer_to(                            if selected_inventory.count_resource(ResourceType::Wood) > 0 {                        if distance < 100.0 {                        let distance = cursor_pos.distance(entity_pos);                        let entity_pos = transform.translation().truncate();                                if amount > 0 {
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
