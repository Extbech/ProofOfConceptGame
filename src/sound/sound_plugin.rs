use bevy::{
    app::{Plugin, Update},
    state::state::OnEnter,
};

use crate::GameState;

use super::{
    events::{play_sound_effect_event, PlaySoundEffectEvent},
    game_music::play_game_music,
};

pub struct SoundPlugin;

impl Plugin for SoundPlugin {
    fn build(&self, app: &mut bevy::app::App) {
        app.add_event::<PlaySoundEffectEvent>()
            .add_systems(OnEnter(GameState::Running), play_game_music)
            .add_systems(Update, (play_sound_effect_event,));
    }
}
