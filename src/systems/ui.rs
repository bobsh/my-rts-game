use crate::components::ui::{EntityInfoPanel, EntityNameText};
use crate::components::unit::Selected;
use crate::components::inventory::{Inventory, InventorySettings}; // Add this import
use bevy::prelude::*;

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_ui)
            .add_systems(Update, update_entity_info_panel);
    }
}

fn setup_ui(mut commands: Commands, asset_server: Res<AssetServer>) {
    let font = asset_server.load("fonts/fira_sans/FiraSans-Bold.ttf");

    // Create a container for our entity info panel in the top right
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
                // Start with the panel hidden
                display: Display::None,
                ..default()
            },
            BackgroundColor(Color::srgba(0.1, 0.1, 0.1, 0.8)),
            EntityInfoPanel,
        ))
        .with_children(|parent| {
            // Entity name as the title/header
            parent.spawn((
                Text::new(""), // Initially empty
                TextFont {
                    font: font.clone(),
                    font_size: 22.0, // Larger font for the title
                    ..default()
                },
                TextColor(Color::WHITE),
                EntityNameText,
            ));

            // Additional info can be added here in the future
        });
}

// Update your UI system to include inventory visualization:

fn update_entity_info_panel(
    selected_entities: Query<(Entity, Option<&Name>, Option<&Inventory>), With<Selected>>,
    _house_query: Query<Entity, With<Name>>,
    mut panel_query: Query<&mut Node, With<EntityInfoPanel>>,
    mut entity_name_text: Query<&mut Text, With<EntityNameText>>,
    _inventory_settings: Query<&InventorySettings>,
) {
    // Get a mutable reference to the panel to control visibility
    if let Ok(mut panel_node) = panel_query.get_single_mut() {
        // Check if there's a selected entity
        if let Ok((_entity, name, _inventory)) = selected_entities.get_single() {
            // Show the panel when something is selected
            panel_node.display = Display::Flex;

            // Update the title text to show the entity name
            if let Ok(mut name_text) = entity_name_text.get_single_mut() {
                let entity_name = if let Some(name) = name {
                    name.as_str().to_string()
                } else {
                    "Entity".to_string()
                };

                *name_text = Text::new(entity_name);
            }

            // Inventory will be displayed by the inventory system

        } else {
            // Hide the panel when nothing is selected
            panel_node.display = Display::None;
        }
    }
}

// Helper function to identify entity types without importing them directly
#[allow(dead_code)]
fn get_entity_type_name(entity: Entity, house_query: &Query<Entity, With<Name>>) -> String {
    // Try to determine what type of entity this is
    if house_query.contains(entity) {
        "House".to_string()
    } else {
        // Generic fallback
        "Entity".to_string()
    }
}
