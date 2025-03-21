use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;

pub fn setup_scene(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((
        Camera2d,
        Transform::from_xyz(1280.0, 720.0, 0.0),
    ));

    // Load the ldtk map file
    let map_handle = asset_server.load("test-map.ldtk");

    commands.spawn(LdtkWorldBundle {
        ldtk_handle: map_handle.into(),
        ..Default::default()
    });
}
