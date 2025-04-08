use bevy::{
    audio::{PlaybackMode, Volume},
    prelude::*,
};

use crate::cleanup;

pub fn play_game_music(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((
        cleanup::ExitGame,
        AudioPlayer::<AudioSource>(asset_server.load("sounds/music/in-game.wav")),
        PlaybackSettings {
            mode: PlaybackMode::Loop,
            volume: Volume::new(0.2),
            ..Default::default()
        },
    ));
}
