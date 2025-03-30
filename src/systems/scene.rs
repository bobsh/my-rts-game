use crate::components::unit::Selected;
use crate::systems::ldtk_calibration::LdtkCalibration;
use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;

pub struct ScenePlugin;

impl Plugin for ScenePlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(LevelSelection::index(0))
            .insert_resource(LdtkSettings {
                level_spawn_behavior: LevelSpawnBehavior::UseWorldTranslation {
                    load_level_neighbors: true,
                },
                ..Default::default()
            })
            .add_systems(Update, level_selection_follow_player)
            .add_systems(Startup, setup_scene)
            .add_systems(Startup, initialize_ldtk_offset);
    }
}

pub fn setup_scene(mut commands: Commands, asset_server: Res<AssetServer>) {
    // Position the camera above the house area where the worker and warrior start
    // Using coordinates based on your map layout - adjust as needed for your specific map
    commands.spawn((Camera2d, Transform::from_xyz(-800.0, -1000.0, 0.0)));

    // Load the ldtk map file
    let map_handle = asset_server.load("test-map.ldtk");

    commands.spawn(LdtkWorldBundle {
        ldtk_handle: map_handle.into(),
        ..Default::default()
    });
}

// Initialize with a conservative offset - the user can adjust using Shift+O+Arrow keys
fn initialize_ldtk_offset(mut ldtk_calibration: ResMut<LdtkCalibration>) {
    // Starting with an offset of 0.0, to be adjusted by the user
    ldtk_calibration.offset = Vec2::ZERO;
    info!("Initialized LDtk with ZERO offset. Use Shift+O+Arrow keys to adjust if needed");
}

fn level_selection_follow_player(
    players: Query<&GlobalTransform, With<Selected>>,
    levels: Query<(&LevelIid, &GlobalTransform)>,
    ldtk_projects: Query<&LdtkProjectHandle>,
    ldtk_project_assets: Res<Assets<LdtkProject>>,
    mut level_selection: ResMut<LevelSelection>,
) {
    if let Ok(player_transform) = players.get_single() {
        let ldtk_project = ldtk_project_assets
            .get(ldtk_projects.single())
            .expect("ldtk project should be loaded before player is spawned");

        for (level_iid, level_transform) in levels.iter() {
            let level = ldtk_project
                .get_raw_level_by_iid(level_iid.get())
                .expect("level should exist in only project");

            let level_bounds = Rect {
                min: Vec2::new(
                    level_transform.translation().x,
                    level_transform.translation().y,
                ),
                max: Vec2::new(
                    level_transform.translation().x + level.px_wid as f32,
                    level_transform.translation().y + level.px_hei as f32,
                ),
            };

            if level_bounds.contains(player_transform.translation().truncate()) {
                *level_selection = LevelSelection::Iid(level_iid.clone());
            }
        }
    }
}
