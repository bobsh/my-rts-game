use bevy::prelude::*;
use crate::components::ui::{UnitInfoPanel, UnitNameText, UnitHealthText, UnitSpeedText};
use crate::components::unit::{Selected, UnitAttributes, Velocity};

pub fn setup_ui(mut commands: Commands, asset_server: Res<AssetServer>) {
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
