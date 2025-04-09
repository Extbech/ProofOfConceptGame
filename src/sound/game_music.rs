use bevy::{
    audio::{PlaybackMode, Volume},
    prelude::*,
};

use crate::cleanup;

use super::sound_volume::SoundVolume;

#[derive(Component)]
pub struct GameMusic;

pub fn play_game_music(mut commands: Commands, asset_server: Res<AssetServer>, sound_volume: Res<SoundVolume>) {
    commands.spawn((
        cleanup::ExitGame,
        AudioPlayer::<AudioSource>(asset_server.load("sounds/music/in-game.wav")),
        PlaybackSettings {
            mode: PlaybackMode::Loop,
            volume: Volume::new(sound_volume.music),
            ..Default::default()
        },
        GameMusic,
    ));
}

pub fn update_in_game_music_volume(
    sound_volume: Res<SoundVolume>,
    mut query: Query<&mut PlaybackSettings, With<GameMusic>>,
) {
    for mut setting in &mut query {
        setting.volume = Volume::new(sound_volume.music);
    }
}