use crate::components::ui::{EntityInfoPanel, EntityNameText};
use crate::components::unit::Selected;
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
            BackgroundColor(Color::rgba(0.1, 0.1, 0.1, 0.8)),
            EntityInfoPanel,
        ))
        .with_children(|parent| {
            // Entity name as the title/header
            parent.spawn((
                Text::new(""),  // Initially empty
                TextFont {
                    font: font.clone(),
                    font_size: 22.0,  // Larger font for the title
                    ..default()
                },
                TextColor(Color::WHITE),
                EntityNameText,
            ));

            // Additional info can be added here in the future
        });
}

fn update_entity_info_panel(
    selected_entities: Query<Entity, With<Selected>>,
    house_query: Query<Entity, With<Name>>,
    unit_query: Query<&Name>,
    mut entity_name_text: Query<&mut Text, With<EntityNameText>>,
    mut panel_query: Query<&mut Node, With<EntityInfoPanel>>,
) {
    // Get a mutable reference to the panel to control visibility
    if let Ok(mut panel_node) = panel_query.get_single_mut() {
        // Check if there's a selected entity
        if let Some(entity) = selected_entities.iter().next() {
            // Show the panel when something is selected
            panel_node.display = Display::Flex;

            // Try to get the entity's name
            let entity_name = match unit_query.get(entity) {
                Ok(name) => name.as_str().to_string(),
                Err(_) => {
                    // Fall back to type detection logic
                    get_entity_type_name(entity, &house_query)
                }
            };

            // Update the title text to show the entity name
            if let Ok(mut name_text) = entity_name_text.get_single_mut() {
                *name_text = Text::new(entity_name);
            }
        } else {
            // Hide the panel when nothing is selected
            panel_node.display = Display::None;
        }
    }
}

// Helper function to identify entity types without importing them directly
fn get_entity_type_name(entity: Entity, house_query: &Query<Entity, With<Name>>) -> String {
    // Try to determine what type of entity this is
    if house_query.contains(entity) {
        "House".to_string()
    } else {
        // Generic fallback
        "Entity".to_string()
    }
}
