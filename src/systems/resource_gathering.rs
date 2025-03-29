use bevy::prelude::*;
use crate::components::inventory::*;
use crate::components::unit::Selected;
use crate::components::unit::Selectable;
use crate::components::movement::{Movable, MoveTarget};
use crate::components::ui::EntityInfoPanel;
use crate::components::skills::{Skills, SkillProgression};
use crate::entities::{Tree, Mine, Quarry};

pub struct ResourceGatheringPlugin;

impl Plugin for ResourceGatheringPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, gather_resources)
           .add_systems(Update, start_gathering)
           .add_systems(Update, update_skills_from_activities)
           .add_systems(Update, update_character_info_ui)
           .add_systems(Update, handle_resource_transfer); // Add this line
    }
}

// Component to track gathering progress
#[derive(Component, Debug)]
pub struct Gathering {
    pub resource_type: ResourceType,
    pub progress: f32,
    pub target: Entity,
    pub base_time: f32,          // Base time in seconds
    pub skill_modifier: f32,    // Higher skill = faster gathering
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
    mut commands: Commands, // Make mutable
    time: Res<Time>,
    mut gatherers: Query<(Entity, &mut Gathering, &mut Inventory, &InventorySettings, &Skills)>,
    mut skill_progression: Query<&mut SkillProgression>,
    trees: Query<Entity, With<Tree>>,
    mines: Query<Entity, With<Mine>>,
    _quarries: Query<Entity, With<Quarry>>, // Prefix with underscore
) {
    for (entity, mut gathering, mut inventory, settings, skills) in &mut gatherers {
        // Verify target still exists
        let target_exists = trees.contains(gathering.target) ||
                            mines.contains(gathering.target) ||
                            _quarries.contains(gathering.target);

        if !target_exists {
            info!("Resource node {:?} no longer exists, stopping gathering", gathering.target);
            commands.entity(entity).remove::<Gathering>();
            continue;
        }

        // Use skill modifier for gathering speed
        let progress_rate = gathering.skill_modifier * time.delta_secs();
        gathering.progress += progress_rate;

        // Track progress
        if gathering.progress >= gathering.base_time {
            // Resource type from target
            let resource_type = gathering.resource_type;

            // Base resources + skill bonus (rounded down)
            let skill_value = match resource_type {
                ResourceType::Wood => skills.woodcutting,
                ResourceType::Gold => skills.mining,
                ResourceType::Stone => skills.harvesting,
            };

            let base_yield = 1;
            let bonus_yield = (skill_value / 3.0).floor() as u32;
            let total_yield = base_yield + bonus_yield;

            // Add to inventory
            let overflow = inventory.add_resource(resource_type, total_yield, settings.max_stack_size);

            // Update XP
            if let Ok(mut progression) = skill_progression.get_mut(entity) {
                match resource_type {
                    ResourceType::Wood => progression.woodcutting_xp += 5.0,
                    ResourceType::Gold => progression.mining_xp += 5.0,
                    ResourceType::Stone => progression.harvesting_xp += 5.0,
                }
            }

            // Reset or stop
            if overflow == 0 {
                gathering.progress = 0.0;
                info!("Gathered {} {:?}, continuing to gather", total_yield, resource_type);
            } else {
                info!("Inventory full, stopping gathering");
                commands.entity(entity).remove::<Gathering>();
            }
        }
    }
}

// Replace Worker-specific gathering with skill-based gathering
fn start_gathering(
    mut commands: Commands,
    mouse_button: Res<ButtonInput<MouseButton>>,
    windows: Query<&Window>,
    camera_q: Query<(&Camera, &GlobalTransform)>,
    selected_characters: Query<(Entity, &Skills), With<Selected>>,
    all_characters: Query<(Entity, Option<&Selected>), With<Skills>>,
    resource_nodes: Query<(Entity, &GlobalTransform, &Sprite), Or<(With<Tree>, With<Mine>, With<Quarry>)>>,
    trees: Query<Entity, With<Tree>>,
    mines: Query<Entity, With<Mine>>,
    _quarries: Query<Entity, With<Quarry>>, // Prefix with underscore
) {
    // Only process right-clicks
    if !mouse_button.just_pressed(MouseButton::Right) {
        return;
    }

    info!("Right-click detected");

    // Debug logging - list all characters and their selection state
    info!("All characters and their selection status:");
    for (entity, selected) in all_characters.iter() {
        info!("Character {:?} - Selected: {}", entity, selected.is_some());
    }

    // Check if we have a selected character
    let Some((character_entity, skills)) = selected_characters.iter().next() else {
        info!("No character selected");
        return;
    };

    info!("Character {:?} is selected", character_entity);

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

            // Determine resource type and relevant skill
            let (resource_type, skill_value) = if trees.contains(node_entity) {
                info!("It's a tree");
                (ResourceType::Wood, skills.woodcutting)
            } else if mines.contains(node_entity) {
                info!("It's a mine");
                (ResourceType::Gold, skills.mining)
            } else {
                info!("It's a quarry");
                (ResourceType::Stone, skills.harvesting)
            };

            // Start gathering with skill-based parameters
            info!("Starting gathering for character {:?} at resource node {:?}", character_entity, node_entity);
            commands.entity(character_entity).insert(Gathering {
                resource_type,
                progress: 0.0,
                target: node_entity,
                base_time: 3.0,
                skill_modifier: skill_value,
            });

            break;
        }
    }
}

// System to update skills based on activities
fn update_skills_from_activities(
    mut characters: Query<(&mut Skills, &mut SkillProgression)>,
    gatherers: Query<(Entity, &Gathering)>,
    time: Res<Time>,
) {
    // Track who's doing what activity
    for (entity, gathering) in &gatherers {
        if let Ok((mut skills, mut progression)) = characters.get_mut(entity) {
            // Add XP based on activity - use delta_secs() in Bevy 0.15
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

// System to update character info UI
fn update_character_info_ui(
    selected_entities: Query<(Entity, &Skills, Option<&Inventory>), With<Selected>>,
    panel_query: Query<Entity, With<EntityInfoPanel>>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    gathering_query: Query<&Gathering>,
) {
    if let Ok(panel_entity) = panel_query.get_single() {
        // First, clear any existing UI in the panel
        commands.entity(panel_entity).despawn_descendants();

        if let Ok((entity, skills, inventory)) = selected_entities.get_single() {
            info!("Updating UI for entity: {:?}, has inventory: {}", entity, inventory.is_some());

            commands.entity(panel_entity).with_children(|parent| {
                // Title
                parent.spawn((
                    Text::new("Character Info"),
                    TextFont {
                        font: asset_server.load("fonts/fira_sans/FiraSans-Bold.ttf"),
                        font_size: 18.0,
                        ..default()
                    },
                    TextColor(Color::WHITE),
                ));

                // Skills section
                parent.spawn((
                    Text::new("Skills:"),
                    TextFont {
                        font: asset_server.load("fonts/fira_sans/FiraSans-Bold.ttf"),
                        font_size: 16.0,
                        ..default()
                    },
                    TextColor(Color::WHITE),
                ));

                // List each skill
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

                // Add this to your UI update when showing a character that's gathering
                if let Ok(gathering) = gathering_query.get(entity) {
                    let progress_percent = (gathering.progress / gathering.base_time) * 100.0;
                    parent.spawn((
                        Text::new(format!("Gathering: {:.1}%", progress_percent)),
                        TextFont {
                            font: asset_server.load("fonts/fira_sans/FiraSans-Bold.ttf"),
                            font_size: 14.0,
                            ..default()
                        },
                        TextColor(Color::WHITE),
                    ));
                }

                // Inventory section (if character has one)
                if let Some(inv) = inventory {
                    // First determine capacity statistics
                    let used_slots = inv.slots.iter().filter(|slot| slot.is_some()).count();
                    let total_slots = inv.max_slots;

                    // Inventory title with capacity
                    parent.spawn((
                        Text::new(format!("Inventory ({}/{})", used_slots, total_slots)),
                        TextFont {
                            font: asset_server.load("fonts/fira_sans/FiraSans-Bold.ttf"),
                            font_size: 16.0,
                            ..default()
                        },
                        TextColor(Color::WHITE),
                    ));

                    // Display inventory contents
                    for (i, slot) in inv.slots.iter().enumerate() {
                        if let Some(inv_slot) = slot {
                            let resource_name = match inv_slot.resource_type {
                                ResourceType::Gold => "Gold",
                                ResourceType::Wood => "Wood",
                                ResourceType::Stone => "Stone",
                            };

                            // Use emoji or custom icons for resources
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
                            // Show empty slots too
                            parent.spawn((
                                Text::new(format!("Slot {}: Empty", i + 1)),
                                TextFont { font: asset_server.load("fonts/fira_sans/FiraSans-Bold.ttf"), font_size: 14.0, ..default() },
                                TextColor(Color::srgba(0.7, 0.7, 0.7, 1.0)),
                            ));
                        }
                    }
                } else {
                    // Show a message if no inventory
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

// Add a new system for transferring resources between entities
fn handle_resource_transfer(
    _commands: Commands, // Prefix with underscore
    keyboard: Res<ButtonInput<KeyCode>>,
    mouse_button: Res<ButtonInput<MouseButton>>,
    mut selected_entity: Query<(Entity, &mut Inventory, &InventorySettings), With<Selected>>, // Make mutable
    mut entities_with_inventory: Query<(Entity, &mut Inventory, &InventorySettings, &GlobalTransform)>,
    windows: Query<&Window>,
    camera_q: Query<(&Camera, &GlobalTransform)>,
) {
    // Transfer items when holding T and right-clicking
    if keyboard.pressed(KeyCode::KeyT) && mouse_button.just_pressed(MouseButton::Right) {
        // Check if we have a selected entity
        if let Ok((selected_entity, mut selected_inventory, _selected_settings)) = selected_entity.get_single_mut() {
            // Get click position
            let window = windows.single();
            if let Some(cursor_position) = window.cursor_position() {
                let (camera, camera_transform) = camera_q.single();
                if let Ok(cursor_ray) = camera.viewport_to_world(camera_transform, cursor_position) {
                    let cursor_pos = cursor_ray.origin.truncate();

                    // Find entities near click position
                    for (entity, mut inventory, settings, transform) in &mut entities_with_inventory {
                        // Skip selected entity
                        if entity == selected_entity {
                            continue;
                        }

                        // Check if entity is close enough (within 100 units)
                        let entity_pos = transform.translation().truncate();
                        let distance = cursor_pos.distance(entity_pos);

                        if distance < 100.0 {
                            // Check if target has resources to transfer (or has space if selected has resources)
                            if selected_inventory.count_resource(ResourceType::Wood) > 0 {
                                // Transfer wood from selected to target
                                let amount = selected_inventory.transfer_to(
                                    &mut inventory,
                                    ResourceType::Wood,
                                    1, // Transfer 1 at a time
                                    settings.max_stack_size
                                );

                                if amount > 0 {
                                    info!("Transferred {} Wood to entity {:?}", amount, entity);
                                }
                            }
                            // Add similar logic for other resource types

                            break;
                        }
                    }
                }
            }
        }
    }
}
