use bevy::prelude::*;
use bevy_aseprite_ultra::prelude::*;
use bevy_ecs_ldtk::prelude::*;

/// Plugin for scene management and loading.
pub struct ScenePlugin;

#[derive(Resource)]
struct SplashTimer(Timer);

#[derive(Resource, Default)]
struct MapLoadState {
    requested: bool,
}

impl Plugin for ScenePlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(LevelSelection::index(0))
            .insert_resource(LdtkSettings {
                ..Default::default()
            })
            .init_resource::<MapLoadState>()
            .add_systems(Startup, setup_scene)
            .add_systems(Update, splash_screen_system);
    }
}

#[derive(Component)]
struct SplashScreen;

/// Sets up the initial scene with a camera and splash screen.
pub fn setup_scene(mut commands: Commands, asset_server: Res<AssetServer>) {
    info!("Setting up scene...");

    // Camera
    commands.spawn((Camera2d, Transform::from_xyz(4000.0, 3000.0, 0.0)));

    // animations in bevy ui
    commands.spawn((
        SplashScreen,
        Node {
            justify_content: JustifyContent::Center,
            ..default()
        },
        AseUiAnimation {
            aseprite: asset_server.load("splashscreen1.aseprite"),
            animation: Animation::default(),
        },
    ));

    // Insert a timer resource for splash duration (e.g., 3 seconds)
    commands.insert_resource(SplashTimer(Timer::from_seconds(3.0, TimerMode::Once)));

    info!("Splash screen spawned, waiting before loading map...");
}

/// System to handle the splash screen and map loading.
fn splash_screen_system(
    mut commands: Commands,
    time: Res<Time>,
    mut timer: ResMut<SplashTimer>,
    asset_server: Res<AssetServer>,
    splash_query: Query<Entity, With<SplashScreen>>,
    mut level_events: EventReader<LevelEvent>,
    mut map_state: ResMut<MapLoadState>,
) {
    timer.0.tick(time.delta());

    // Process any level events that indicate our level is fully loaded
    for event in level_events.read() {
        match event {
            // This matches when a level has been completely spawned
            LevelEvent::Spawned(_) => {
                if map_state.requested {
                    info!("Level fully loaded, despawning splash screen...");
                    for entity in splash_query.iter() {
                        commands.entity(entity).despawn_recursive();
                    }
                }
            }
            _ => {} // Ignore other level events
        }
    }

    // If timer is finished but map hasn't been requested yet, request the map
    if timer.0.finished() && !map_state.requested {
        info!("Splash timer finished, despawning splash screen and loading please wait screen...");
        for entity in splash_query.iter() {
            commands.entity(entity).despawn_recursive();
        }

        commands.spawn((
            SplashScreen,
            Node {
                justify_content: JustifyContent::Center,
                ..default()
            },
            AseUiAnimation {
                aseprite: asset_server.load("pleasewait1.aseprite"),
                animation: Animation::default(),
            },
        ));

        info!("Loading LDtk map: test-map.ldtk");
        let map_handle = asset_server.load("test-map.ldtk");
        info!("Map handle created: {:?}", map_handle);

        commands.spawn(LdtkWorldBundle {
            ldtk_handle: map_handle.into(),
            ..Default::default()
        });

        info!("Spawned Ldtk world bundle ...");

        map_state.requested = true;
        // The splash screen will now stay visible until the level is loaded
    }
}
