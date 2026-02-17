use bevy::{
    app::{Plugin, Update},
    state::state::OnEnter,
};

use crate::GameState;

use super::{
    events::{play_sound_effect_event, update_volume, PlaySoundEffectEvent, SetSoundVolume},
    game_music::{play_game_music, update_in_game_music_volume},
};

pub struct SoundPlugin;

impl Plugin for SoundPlugin {
    fn build(&self, app: &mut bevy::app::App) {
        app.add_message::<PlaySoundEffectEvent>()
            .add_message::<SetSoundVolume>()
            .add_systems(OnEnter(GameState::Running), play_game_music)
            .add_systems(
                Update,
                (
                    play_sound_effect_event,
                    update_volume,
                    update_in_game_music_volume,
                ),
            );
    }
}
