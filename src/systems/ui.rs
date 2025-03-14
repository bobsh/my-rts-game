use crate::components::inventory::Inventory;
use crate::components::ui::{
    InventorySlot, InventoryUI, ResourceText, ResourcesDisplay, UnitHealthText, UnitInfoPanel,
    UnitNameText, UnitSpeedText,
};
use crate::components::unit::{Selected, UnitAttributes, Velocity};
use crate::resources::{PlayerResources, ResourceRegistry};
use bevy::prelude::*;

pub fn setup_ui(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    resource_registry: Res<ResourceRegistry>,
) {
    let font = asset_server.load("fonts/fira_sans/FiraSans-Bold.ttf");

    // Create a container for our unit info panel in the top right
    commands
        .spawn((
            Node {
                position_type: PositionType::Absolute,
                right: Val::Px(10.0),
                top: Val::Px(10.0),
                width: Val::Px(200.0),
                height: Val::Px(120.0),
                padding: UiRect::all(Val::Px(10.0)),
                flex_direction: FlexDirection::Column,
                ..default()
            },
            UnitInfoPanel,
        ))
        .with_children(|parent| {
            // Title - Unit Info
            parent.spawn((
                Text::new("Unit Information"),
                TextFont {
                    font: font.clone(),
                    font_size: 18.0,
                    ..default()
                },
                TextColor(Color::WHITE),
            ));

            // Name
            parent.spawn((
                Text::new("No unit selected"),
                TextFont {
                    font: font.clone(),
                    font_size: 16.0,
                    ..default()
                },
                TextColor(Color::WHITE),
                UnitNameText,
            ));

            // Health
            parent.spawn((
                Text::new(""),
                TextFont {
                    font: font.clone(),
                    font_size: 16.0,
                    ..default()
                },
                TextColor(Color::WHITE),
                UnitHealthText,
            ));

            // Speed
            parent.spawn((
                Text::new(""),
                TextFont {
                    font: font.clone(),
                    font_size: 16.0,
                    ..default()
                },
                TextColor(Color::WHITE),
                UnitSpeedText,
            ));
        });

    // Add a resources display at the top of the screen
    let mut resources_entity = commands.spawn((
        Node {
            position_type: PositionType::Absolute,
            top: Val::Px(10.0),
            left: Val::Px(10.0),
            width: Val::Px(400.0), // Make wider to accommodate more resources
            height: Val::Auto,
            padding: UiRect::all(Val::Px(8.0)),
            column_gap: Val::Px(15.0),
            flex_direction: FlexDirection::Row,
            flex_wrap: FlexWrap::Wrap, // Allow wrapping to multiple rows
            align_items: AlignItems::Center,
            ..default()
        },
        ResourcesDisplay,
    ));

    // Dynamically add UI elements for all registered resources
    resources_entity.with_children(|parent| {
        for resource_def in resource_registry.all() {
            parent.spawn((
                Text::new(format!("{}: 0", resource_def.name)),
                TextFont {
                    font: font.clone(),
                    font_size: 16.0,
                    ..default()
                },
                TextColor(resource_def.color),
                ResourceText(resource_def.id.0.clone()),
            ));
        }
    });
}

#[allow(clippy::type_complexity)]
pub fn update_unit_info(
    selected_units: Query<(&UnitAttributes, &Velocity), With<Selected>>,
    mut text_query: ParamSet<(
        Query<&mut Text, With<UnitNameText>>,
        Query<&mut Text, With<UnitHealthText>>,
        Query<&mut Text, With<UnitSpeedText>>,
    )>,
) {
    // Get the selected unit info (if any)
    let selected_info = selected_units.get_single().ok();

    // Update name text
    let mut name_query = text_query.p0();
    if let Ok(mut name_text) = name_query.get_single_mut() {
        if let Some((attributes, _)) = selected_info {
            *name_text = Text::new(format!("Name: {}", attributes.name));
        } else {
            *name_text = Text::new("No unit selected".to_string());
        }
    }

    // Update health text
    let mut health_query = text_query.p1();
    if let Ok(mut health_text) = health_query.get_single_mut() {
        if let Some((attributes, _)) = selected_info {
            // Calculate health percentage
            let health_percent = (attributes.health / attributes.max_health) * 100.0;
            *health_text = Text::new(format!(
                "Health: {:.0}/{:.0} ({:.0}%)",
                attributes.health, attributes.max_health, health_percent
            ));
        } else {
            *health_text = Text::new(String::new());
        }
    }

    // Update speed text
    let mut speed_query = text_query.p2();
    if let Ok(mut speed_text) = speed_query.get_single_mut() {
        if let Some((_, velocity)) = selected_info {
            *speed_text = Text::new(format!("Speed: {:.0}", velocity.speed));
        } else {
            *speed_text = Text::new(String::new());
        }
    }
}

pub fn update_resources_display(
    player_resources: Res<PlayerResources>,
    resource_registry: Res<ResourceRegistry>,
    mut text_query: Query<(&mut Text, &ResourceText)>,
) {
    for (mut text, resource_text) in &mut text_query {
        let resource_id = crate::resources::ResourceId(resource_text.0.clone());
        let amount = player_resources.get(&resource_id);

        // Find the resource definition to get its name
        if let Some(resource_def) = resource_registry.get(&resource_id) {
            *text = Text::new(format!("{}: {}", resource_def.name, amount));
        }
    }
}

// Add the new inventory UI update function
pub fn update_inventory_ui(
    mut commands: Commands,
    selected_units: Query<(Entity, &Inventory), With<Selected>>,
    inventory_ui_query: Query<Entity, With<InventoryUI>>,
    mut inventory_slots: Query<(&mut ImageNode, &mut InventorySlot, &Children), With<Button>>,
    mut text_query: Query<&mut Text>,
    resource_registry: Res<ResourceRegistry>,
    asset_server: Res<AssetServer>,
) {
    // Only show inventory for the first selected unit
    let selected_unit = if let Some((entity, inventory)) = selected_units.iter().next() {
        Some((entity, inventory))
    } else {
        None
    };

    // Remove existing inventory UI if no units selected
    if selected_unit.is_none() {
        for ui_entity in inventory_ui_query.iter() {
            commands.entity(ui_entity).despawn_recursive();
        }
        return;
    }

    let (selected_entity, inventory) = selected_unit.unwrap();

    // If the inventory UI doesn't exist yet, create it
    if inventory_ui_query.is_empty() {
        // Create the main UI container
        let ui_entity = commands
            .spawn((
                Node {
                    position_type: PositionType::Absolute,
                    bottom: Val::Px(10.0),
                    right: Val::Px(10.0),
                    width: Val::Px(120.0),
                    height: Val::Px(80.0),
                    padding: UiRect::all(Val::Px(8.0)),
                    flex_direction: FlexDirection::Column,
                    ..default()
                },
                InventoryUI,
            ))
            .id();

        // Add a title
        let title = commands
            .spawn((
                Text::new("Inventory"),
                TextFont {
                    font: asset_server.load("fonts/fira_sans/FiraSans-Bold.ttf"),
                    font_size: 16.0,
                    ..default()
                },
                TextColor(Color::WHITE),
            ))
            .id();
        commands.entity(ui_entity).add_children(&[title]);

        // Create a grid container for the slots
        let grid_entity = commands
            .spawn(Node {
                margin: UiRect::all(Val::Px(5.0)),
                width: Val::Percent(100.0),
                height: Val::Px(70.0),
                display: Display::Flex,
                flex_direction: FlexDirection::Row,
                flex_wrap: FlexWrap::Wrap,
                ..default()
            })
            .id();

        commands.entity(ui_entity).add_children(&[grid_entity]);

        // Create slot placeholders - avoid nested with_children calls
        let mut slot_entities = Vec::new();
        let mut text_entities = Vec::new();

        // First, create all slots
        for _ in 0..8 {
            let slot = commands
                .spawn((
                    Node {
                        width: Val::Px(24.0),
                        height: Val::Px(24.0),
                        margin: UiRect::all(Val::Px(2.0)),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    Button {},
                    ImageNode {
                        image: asset_server.load("empty_slot.png"),
                        ..default()
                    },
                    InventorySlot {
                        resource_id: None,
                        entity_owner: selected_entity,
                    },
                ))
                .id();

            // Create text for the quantity
            let text = commands
                .spawn((
                    Text::new(""),
                    TextFont {
                        font: asset_server.load("fonts/fira_sans/FiraSans-Bold.ttf"),
                        font_size: 10.0,
                        ..default()
                    },
                    TextColor(Color::WHITE),
                ))
                .id();

            slot_entities.push(slot);
            text_entities.push(text);
        }

        // Now add all slots to the grid
        commands.entity(grid_entity).add_children(&slot_entities);

        // Then add text to each slot
        for (slot, text) in slot_entities.iter().zip(text_entities.iter()) {
            commands.entity(*slot).add_children(&[*text]);
        }
    }

    // Update existing inventory slots
    for (mut ui_image, mut slot, children) in &mut inventory_slots {
        // Only update slots belonging to the selected entity
        if slot.entity_owner != selected_entity {
            slot.entity_owner = selected_entity;
            slot.resource_id = None;
            ui_image.image = asset_server.load("empty_slot.png");

            // Update quantity text
            if let Some(&child) = children.first() {
                if let Ok(mut text) = text_query.get_mut(child) {
                    *text = Text::new(String::new());
                }
            }
        }
    }

    // Update slot contents based on inventory
    let mut slot_index = 0;
    for (resource_id, amount) in inventory.resources() {
        // Only show up to 8 resources
        if slot_index >= 8 {
            break;
        }

        // Get the slot at the current index
        if let Some((mut ui_image, mut slot, children)) = inventory_slots.iter_mut().nth(slot_index)
        {
            if slot.resource_id.as_ref() != Some(resource_id) {
                slot.resource_id = Some(resource_id.clone());

                // Get resource icon
                if let Some(resource_def) = resource_registry.get(resource_id) {
                    ui_image.image = asset_server.load(&resource_def.icon_path);
                } else {
                    ui_image.image = asset_server.load("unknown.png");
                }
            }

            // Update quantity text
            if let Some(&child) = children.first() {
                if let Ok(mut text) = text_query.get_mut(child) {
                    *text = Text::new(amount.to_string());
                }
            }

            slot_index += 1;
        }
    }

    // Clear unused slots
    for i in slot_index..8 {
        if let Some((mut ui_image, mut slot, children)) = inventory_slots.iter_mut().nth(i) {
            if slot.resource_id.is_some() {
                slot.resource_id = None;
                ui_image.image = asset_server.load("empty_slot.png");

                // Clear quantity text
                if let Some(&child) = children.first() {
                    if let Ok(mut text) = text_query.get_mut(child) {
                        *text = Text::new(String::new());
                    }
                }
            }
        }
    }
}
