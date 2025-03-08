use bevy::app::{Plugin, Update};

use super::events::{play_sound_effect_event, PlaySoundEffectEvent};

pub struct SoundPlugin;

impl Plugin for SoundPlugin {
    fn build(&self, app: &mut bevy::app::App) {
        app.add_event::<PlaySoundEffectEvent>()
            .add_systems(Update, (play_sound_effect_event,));
    }
}
