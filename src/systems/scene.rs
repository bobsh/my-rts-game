use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;

pub struct ScenePlugin;

#[derive(Resource)]
struct SplashTimer(Timer);

impl Plugin for ScenePlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(LevelSelection::index(0))
            .insert_resource(LdtkSettings {
                ..Default::default()
            })
            .add_systems(Startup, setup_scene)
            .add_systems(Update, splash_screen_system);
    }
}

#[derive(Component)]
struct SplashScreen;

pub fn setup_scene(mut commands: Commands, asset_server: Res<AssetServer>) {
    info!("Setting up scene...");

    // Camera
    commands.spawn((Camera2d, Transform::from_xyz(4000.0, 3000.0, 0.0)));

    // Splash screen
    let splash_handle: Handle<Image> = asset_server.load("splashscreen1.gif");
    commands.spawn((
        Sprite {
            custom_size: Some(Vec2::ZERO),
            image: splash_handle,
            ..default()
        },
        SplashScreen,
    ));

    // Insert a timer resource for splash duration (e.g., 3 seconds)
    commands.insert_resource(SplashTimer(Timer::from_seconds(3.0, TimerMode::Once)));

    info!("Splash screen spawned, waiting before loading map...");
}

fn splash_screen_system(
    mut commands: Commands,
    time: Res<Time>,
    mut timer: ResMut<SplashTimer>,
    asset_server: Res<AssetServer>,
    mut splash_query: Query<Entity, With<SplashScreen>>,
    mut has_loaded_map: Local<bool>,
) {
    timer.0.tick(time.delta());

    if timer.0.finished() && !*has_loaded_map {
        // Despawn splash screen
        for entity in splash_query.iter_mut() {
            commands.entity(entity).despawn_recursive();
        }

        // Load LDtk map
        info!("Loading LDtk map: test-map.ldtk");
        let map_handle = asset_server.load("test-map.ldtk");
        info!("Map handle created: {:?}", map_handle);

        commands.spawn(LdtkWorldBundle {
            ldtk_handle: map_handle.into(),
            ..Default::default()
        });

        info!("Scene setup complete.");
        *has_loaded_map = true;
    }
}
