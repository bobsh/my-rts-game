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
                ..default()
            },
            BackgroundColor(Color::srgba(0.1, 0.1, 0.1, 0.8)),
            EntityInfoPanel,
        ))
        .with_children(|parent| {
            // Title - Entity Info
            parent.spawn((
                Text::new("Entity Information"),
                TextFont {
                    font: font.clone(),
                    font_size: 18.0,
                    ..default()
                },
                TextColor(Color::WHITE),
            ));

            // Name of the selected entity
            parent.spawn((
                Text::new("No entity selected"),
                TextFont {
                    font: font.clone(),
                    font_size: 16.0,
                    ..default()
                },
                TextColor(Color::WHITE),
                EntityNameText,
            ));

            // We can add more info elements here in the future
        });
}

fn update_entity_info_panel(
    selected_entities: Query<Entity, With<Selected>>,
    // Try to get entity type components
    house_query: Query<Entity, With<Name>>,
    unit_query: Query<&Name>,
    mut entity_name_text: Query<&mut Text, With<EntityNameText>>,
) {
    // Get the entity name text component
    if let Ok(mut name_text) = entity_name_text.get_single_mut() {
        // Check if there's a selected entity
        if let Some(entity) = selected_entities.iter().next() {
            // Try to get the entity's name
            let entity_name = match unit_query.get(entity) {
                Ok(name) => name.as_str().to_string(),
                Err(_) => {
                    // Fall back to type detection logic
                    get_entity_type_name(entity, &house_query)
                }
            };

            // In Bevy 0.15, we update Text directly
            *name_text = Text::new(format!("Selected: {}", entity_name));
        } else {
            // No entity selected
            *name_text = Text::new("No entity selected");
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
