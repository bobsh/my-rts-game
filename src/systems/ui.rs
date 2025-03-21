use crate::components::ui::{
    UnitHealthText, UnitInfoPanel,
    UnitNameText, UnitSpeedText,
};
use bevy::prelude::*;

pub fn setup_ui(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
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
}
