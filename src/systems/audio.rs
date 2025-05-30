use bevy::prelude::*;
use bevy_kira_audio::{Audio, AudioControl, AudioPlugin};

/// Plugin for the audio system.
pub struct AudioSystemPlugin;

impl Plugin for AudioSystemPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(AudioPlugin)
            .add_systems(Startup, play_background_music);
    }
}

/// Plays background music when the game starts.
fn play_background_music(asset_server: Res<AssetServer>, audio: Res<Audio>) {
    let music = asset_server.load("music/first_steps.mp3");
    audio.play(music).looped().with_volume(0.1);
}
