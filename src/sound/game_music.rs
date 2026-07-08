use bevy::{
    audio::{PlaybackMode, Volume},
    prelude::*,
};

use crate::cleanup;

use super::sound_volume::SoundVolume;

#[derive(Component)]
pub struct GameMusic;

/// Checks if the game music is alreading spawned, if not, it spawns the game music.
/// If it has spawned, that means it has been paused, so it resumes the game music.
pub fn play_game_music(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    sound_volume: Res<SoundVolume>,
    game_music_query: Query<&AudioSink, With<GameMusic>>,
) {
    if let Ok(sink) = game_music_query.single() {
        sink.play();
        return;
    }
    commands.spawn((
        cleanup::ExitGame,
        AudioPlayer::<AudioSource>(asset_server.load("sounds/music/in-game.wav")),
        PlaybackSettings {
            mode: PlaybackMode::Loop,
            volume: Volume::Linear(sound_volume.music),
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
        setting.volume = Volume::Linear(sound_volume.music);
    }
}

pub fn pause_ingame_music(mut query: Query<&AudioSink, With<GameMusic>>) {
    if let Ok(sink) = query.single_mut() {
        sink.pause();
    }
}
