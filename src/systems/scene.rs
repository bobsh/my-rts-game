use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;

use crate::components::unit::Selected;

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
            .add_systems(Startup, setup_scene);
    }
}

pub fn setup_scene(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((Camera2d, Transform::from_xyz(-550.0, -1050.0, 0.0)));

    // Load the ldtk map file
    let map_handle = asset_server.load("test-map.ldtk");

    commands.spawn(LdtkWorldBundle {
        ldtk_handle: map_handle.into(),
        ..Default::default()
    });
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
