use bevy::prelude::*;
use crate::components::ui::{UnitInfoPanel, UnitNameText, UnitHealthText, UnitSpeedText, ResourcesDisplay, ResourceText};
use crate::components::unit::{Selected, UnitAttributes, Velocity};
use crate::resources::{PlayerResources, ResourceRegistry};

pub fn setup_ui(mut commands: Commands, asset_server: Res<AssetServer>, resource_registry: Res<ResourceRegistry>) {
    let font = asset_server.load("fonts/fira_sans/FiraSans-Bold.ttf");
    
    // Create a container for our unit info panel in the top right
    commands
        .spawn((
            NodeBundle {
                style: Style {
                    position_type: PositionType::Absolute,
                    right: Val::Px(10.0),
                    top: Val::Px(10.0),
                    width: Val::Px(200.0),
                    height: Val::Px(120.0),
                    padding: UiRect::all(Val::Px(10.0)),
                    flex_direction: FlexDirection::Column,
                    ..default()
                },
                background_color: BackgroundColor(Color::rgba(0.1, 0.1, 0.1, 0.8)),
                ..default()
            },
            UnitInfoPanel,
        ))
        .with_children(|parent| {
            // Title - Unit Info
            parent.spawn(TextBundle::from_section(
                "Unit Information",
                TextStyle {
                    font: font.clone(),
                    font_size: 18.0,
                    color: Color::WHITE,
                },
            ));
            
            // Name
            parent.spawn((
                TextBundle::from_section(
                    "No unit selected",
                    TextStyle {
                        font: font.clone(),
                        font_size: 16.0,
                        color: Color::WHITE,
                    },
                ),
                UnitNameText,
            ));
            
            // Health
            parent.spawn((
                TextBundle::from_section(
                    "",
                    TextStyle {
                        font: font.clone(),
                        font_size: 16.0,
                        color: Color::WHITE,
                    },
                ),
                UnitHealthText,
            ));
            
            // Speed
            parent.spawn((
                TextBundle::from_section(
                    "",
                    TextStyle {
                        font: font.clone(),
                        font_size: 16.0,
                        color: Color::WHITE,
                    },
                ),
                UnitSpeedText,
            ));
        });
    
    // Add a resources display at the top of the screen
    let mut resources_entity = commands
        .spawn((
            NodeBundle {
                style: Style {
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
                background_color: BackgroundColor(Color::rgba(0.1, 0.1, 0.1, 0.8)),
                ..default()
            },
            ResourcesDisplay,
        ));
        
    // Dynamically add UI elements for all registered resources
    resources_entity.with_children(|parent| {
        for resource_def in resource_registry.all() {
            parent.spawn((
                TextBundle::from_section(
                    format!("{}: 0", resource_def.name),
                    TextStyle {
                        font: font.clone(),
                        font_size: 16.0,
                        color: resource_def.color,
                    },
                ),
                ResourceText(resource_def.id.0.clone()),
            ));
        }
    });
}

// Fixed update_unit_info function using ParamSet to avoid conflicts
pub fn update_unit_info(
    selected_units: Query<(&UnitAttributes, &Velocity), With<Selected>>,
    mut text_query: ParamSet<(
        Query<&mut Text, With<UnitNameText>>,
        Query<&mut Text, With<UnitHealthText>>,
        Query<&mut Text, With<UnitSpeedText>>
    )>,
) {
    // Get the selected unit info (if any)
    let selected_info = selected_units.get_single().ok();
    
    // Update name text
    let mut name_query = text_query.p0();
    if let Ok(mut name_text) = name_query.get_single_mut() {
        if let Some((attributes, _)) = selected_info {
            name_text.sections[0].value = format!("Name: {}", attributes.name);
        } else {
            name_text.sections[0].value = "No unit selected".to_string();
        }
    }
    
    // Update health text
    let mut health_query = text_query.p1();
    if let Ok(mut health_text) = health_query.get_single_mut() {
        if let Some((attributes, _)) = selected_info {
            // Calculate health percentage
            let health_percent = (attributes.health / attributes.max_health) * 100.0;
            health_text.sections[0].value = format!("Health: {:.0}/{:.0} ({:.0}%)", 
                attributes.health, attributes.max_health, health_percent);
        } else {
            health_text.sections[0].value = "".to_string();
        }
    }
    
    // Update speed text
    let mut speed_query = text_query.p2();
    if let Ok(mut speed_text) = speed_query.get_single_mut() {
        if let Some((_, velocity)) = selected_info {
            speed_text.sections[0].value = format!("Speed: {:.0}", velocity.speed);
        } else {
            speed_text.sections[0].value = "".to_string();
        }
    }
}

// Update resource display system
pub fn update_resources_display(
    player_resources: Res<PlayerResources>,
    resource_registry: Res<ResourceRegistry>,
    mut text_query: Query<(&mut Text, &ResourceText)>,
) {
    for (mut text, resource_text) in text_query.iter_mut() {
        let resource_id = crate::resources::ResourceId(resource_text.0.clone());
        let amount = player_resources.get(&resource_id);
        
        // Find the resource definition to get its name
        if let Some(resource_def) = resource_registry.get(&resource_id) {
            text.sections[0].value = format!("{}: {}", resource_def.name, amount);
        }
    }
}
