use crate::components::inventory::*;
use crate::components::ui::EntityInfoPanel;
use crate::components::unit::Selected;
use crate::entities::{House, Warrior, Worker};
use bevy::prelude::*;

pub struct InventoryPlugin;

impl Plugin for InventoryPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_entity_inventories)
            .add_systems(Update, update_inventory_ui);
    }
}

// Set up inventories for all entities that should have them
fn setup_entity_inventories(
    mut commands: Commands,
    workers: Query<Entity, With<Worker>>,
    warriors: Query<Entity, With<Warrior>>,
    houses: Query<Entity, With<House>>,
) {
    // Set up worker inventories (4 slots)
    for entity in &workers {
        commands
            .entity(entity)
            .insert(Inventory::new(4))
            .insert(InventorySettings::default());
    }

    // Set up warrior inventories (2 slots)
    for entity in &warriors {
        commands
            .entity(entity)
            .insert(Inventory::new(2))
            .insert(InventorySettings::default());
    }

    // Set up house inventories (20 slots)
    for entity in &houses {
        commands
            .entity(entity)
            .insert(Inventory::new(20))
            .insert(InventorySettings::default());
    }
}

// Update the UI to show inventory when an entity with inventory is selected
fn update_inventory_ui(
    selected_entities: Query<(Entity, &Inventory), With<Selected>>,
    inventory_settings: Query<&InventorySettings>,
    mut commands: Commands,
    panel_query: Query<Entity, With<EntityInfoPanel>>,
    asset_server: Res<AssetServer>,
) {
    if let Ok(panel_entity) = panel_query.get_single() {
        // If there's a selected entity with inventory
        if let Ok((entity, inventory)) = selected_entities.get_single() {
            // Create a longer-lived value to avoid temporary value issue
            let default_settings = InventorySettings::default();
            let settings = inventory_settings.get(entity).unwrap_or(&default_settings);

            // Create inventory UI within the panel
            commands.entity(panel_entity).with_children(|parent| {
                // Inventory title header
                parent.spawn((
                    Text::new("Inventory"),
                    TextFont {
                        font: asset_server.load("fonts/fira_sans/FiraSans-Bold.ttf"),
                        font_size: 16.0,
                        ..default()
                    },
                    TextColor(Color::WHITE),
                ));

                // Inventory grid - list resources
                for (i, slot) in inventory.slots.iter().enumerate() {
                    if let Some(inv_slot) = slot {
                        let resource_name = match inv_slot.resource_type {
                            ResourceType::Gold => "Gold",
                            ResourceType::Wood => "Wood",
                            ResourceType::Stone => "Stone",
                        };

                        // Example of adding an icon to resource names
                        let resource_icon = match inv_slot.resource_type {
                            ResourceType::Gold => "🪙",
                            ResourceType::Wood => "🪵",
                            ResourceType::Stone => "🪨",
                        };

                        parent.spawn((
                            Text::new(format!(
                                "Slot {}: {} {} x{}/{}",
                                i + 1,
                                resource_icon,
                                resource_name,
                                inv_slot.quantity,
                                settings.max_stack_size
                            )),
                            TextFont {
                                font: asset_server.load("fonts/fira_sans/FiraSans-Bold.ttf"),
                                font_size: 14.0,
                                ..default()
                            },
                            TextColor(Color::WHITE),
                        ));
                    } else {
                        parent.spawn((
                            Text::new(format!("Slot {}: Empty", i + 1)),
                            TextFont {
                                font: asset_server.load("fonts/fira_sans/FiraSans-Bold.ttf"),
                                font_size: 14.0,
                                ..default()
                            },
                            TextColor(Color::srgba(0.7, 0.7, 0.7, 1.0)),
                        ));
                    }
                }
            });
        }
    }
}
