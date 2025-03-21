use std::time::Duration;

use bevy::{
    asset::AssetServer,
    audio::{AudioPlayer, AudioSource, PlaybackMode, PlaybackSettings, Volume},
    ecs::{
        event::{Event, EventReader},
        system::{Commands, Res},
    },
};

use crate::mechanics::cooldown::LifeTime;

pub enum UiSound {
    HoverButtonSound,
    ClickButtonSound,
}

impl UiSound {
    fn get_sound_info(&self) -> (&'static str, f32, f32) {
        match self {
            UiSound::HoverButtonSound => ("sounds/effects/ui/button-hover.mp3", 1., 0.1),
            UiSound::ClickButtonSound => ("sounds/effects/ui/button-click.mp3", 1., 1.),
        }
    }
}

pub enum SkillSound {
    PrimaryAttack,
    OrbJutsu,
    LightningAttack,
}

impl SkillSound {
    fn get_sound_info(&self) -> (&'static str, f32, f32) {
        match self {
            SkillSound::PrimaryAttack => ("sounds/effects/skills/pew-laser.wav", 1., 1.),
            SkillSound::OrbJutsu => ("sounds/effects/skills/pew-laser.wav", 1., 1.),
            SkillSound::LightningAttack => ("sounds/effects/skills/pew-laser.wav", 1., 1.),
        }
    }
}

pub enum PlayerSound {
    Levelup,
    PlayerTakeDamage,
}

impl PlayerSound {
    fn get_sound_info(&self) -> (&'static str, f32, f32) {
        match self {
            PlayerSound::Levelup => ("sounds/effects/player-sound/level-up.mp3", 1., 1.),
            &PlayerSound::PlayerTakeDamage => {
                ("sounds/effects/player-sound/take-damage.mp3", 1., 1.)
            }
        }
    }
}

pub enum SoundEffectKind {
    UiSound(UiSound),
    SkillSound(SkillSound),
    PlayerSound(PlayerSound),
}

impl SoundEffectKind {
    fn get_sound_info(&self) -> (&'static str, f32, f32) {
        match self {
            SoundEffectKind::UiSound(ui) => ui.get_sound_info(),
            SoundEffectKind::SkillSound(skill) => skill.get_sound_info(),
            SoundEffectKind::PlayerSound(player_sound) => player_sound.get_sound_info(),
        }
    }
}

#[derive(Event)]
pub struct PlaySoundEffectEvent(pub SoundEffectKind);

pub fn play_sound_effect_event(
    mut event: EventReader<PlaySoundEffectEvent>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    for ev in event.read() {
        let (path, life_time, volume) = ev.0.get_sound_info();
        commands.spawn((
            AudioPlayer::<AudioSource>(asset_server.load(path)),
            PlaybackSettings {
                mode: PlaybackMode::Once,
                volume: Volume::new(volume),
                ..Default::default()
            },
            LifeTime(Duration::from_secs_f32(life_time)),
        ));
    }
}
