use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;

pub struct ScenePlugin;

impl Plugin for ScenePlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(LevelSelection::index(0))
            .insert_resource(LdtkSettings {
                ..Default::default()
            })
            .add_systems(Startup, setup_scene);
    }
}

pub fn setup_scene(mut commands: Commands, asset_server: Res<AssetServer>) {
    info!("Setting up scene..."); // Log start

    // Position the camera above the house area where the worker and warrior start
    // Using coordinates based on your map layout - adjust as needed for your specific map
    commands.spawn((Camera2d, Transform::from_xyz(4000.0, 3000.0, 0.0)));

    info!("Loading LDtk map: test-map.ldtk"); // Log before loading
    let map_handle = asset_server.load("test-map.ldtk");
    info!("Map handle created: {:?}", map_handle); // Log after loading

    commands.spawn(LdtkWorldBundle {
        ldtk_handle: map_handle.into(),
        ..Default::default()
    });

    info!("Scene setup complete."); // Log end
}
