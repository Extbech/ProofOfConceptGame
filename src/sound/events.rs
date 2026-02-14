use std::time::Duration;

use bevy::{
    audio::{PlaybackMode, Volume},
    prelude::*,
};

use crate::mechanics::cooldown::LifeTime;

use super::sound_volume::SoundVolume;

pub enum UiSound {
    HoverButtonSound,
    ClickButtonSound,
}

impl UiSound {
    fn get_sound_info(&self) -> (&'static str, f32) {
        match self {
            UiSound::HoverButtonSound => ("sounds/effects/ui/button-hover.mp3", 1.),
            UiSound::ClickButtonSound => ("sounds/effects/ui/button-click.mp3", 1.),
        }
    }
}

pub enum SkillSound {
    PrimaryAttack,
    OrbJutsu,
    LightningAttack,
}

impl SkillSound {
    fn get_sound_info(&self) -> (&'static str, f32) {
        match self {
            SkillSound::PrimaryAttack => ("sounds/effects/skills/pew-laser.wav", 1.),
            SkillSound::OrbJutsu => ("sounds/effects/skills/pew-laser.wav", 1.),
            SkillSound::LightningAttack => ("sounds/effects/skills/pew-laser.wav", 1.),
        }
    }
}

pub enum PlayerSound {
    Levelup,
    PlayerTakeDamage,
}

impl PlayerSound {
    fn get_sound_info(&self) -> (&'static str, f32) {
        match self {
            PlayerSound::Levelup => ("sounds/effects/player-sound/level-up.mp3", 1.),
            &PlayerSound::PlayerTakeDamage => ("sounds/effects/player-sound/take-damage.mp3", 1.),
        }
    }
}

pub enum SoundEffectKind {
    UiSound(UiSound),
    SkillSound(SkillSound),
    PlayerSound(PlayerSound),
}

impl SoundEffectKind {
    fn get_sound_info(&self) -> (&'static str, f32) {
        match self {
            SoundEffectKind::UiSound(ui) => ui.get_sound_info(),
            SoundEffectKind::SkillSound(skill) => skill.get_sound_info(),
            SoundEffectKind::PlayerSound(player_sound) => player_sound.get_sound_info(),
        }
    }
}

#[derive(Message)]
pub struct PlaySoundEffectEvent(pub SoundEffectKind);

pub fn play_sound_effect_event(
    mut event: MessageReader<PlaySoundEffectEvent>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    sound_volume: Res<SoundVolume>,
) {
    for ev in event.read() {
        let (path, life_time) = ev.0.get_sound_info();
        commands.spawn((
            AudioPlayer::<AudioSource>(asset_server.load(path)),
            PlaybackSettings {
                mode: PlaybackMode::Once,
                volume: Volume::Linear(sound_volume.sfx),
                ..Default::default()
            },
            LifeTime(Duration::from_secs_f32(life_time)),
        ));
    }
}

#[derive(Message)]
pub enum SetSoundVolume {
    Sfx(f32),
    InGameMusic(f32),
}

pub fn update_volume(
    mut event: MessageReader<SetSoundVolume>,
    mut sound_resource: ResMut<SoundVolume>,
) {
    for ev in event.read() {
        match ev {
            SetSoundVolume::Sfx(volume) => {
                sound_resource.update_sfx_volume(*volume);
            }
            SetSoundVolume::InGameMusic(volume) => {
                sound_resource.update_music_volume(*volume);
            }
        }
    }
}
