use std::time::Duration;

use bevy::{
    asset::AssetServer,
    audio::{AudioPlayer, AudioSource, PlaybackSettings},
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
    fn get_sound_info(&self) -> (&'static str, f32) {
        match self {
            UiSound::HoverButtonSound => ("sounds/effects/skills/pew-laser.wav", 1.),
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

pub enum DamageSound {
    PlayerTakeDamage,
}

impl DamageSound {
    fn get_sound_info(&self) -> (&'static str, f32) {
        match self {
            DamageSound::PlayerTakeDamage => ("sounds/effects/skills/pew-laser.wav", 1.),
        }
    }
}

pub enum SoundEffectKind {
    UiSound(UiSound),
    SkillSound(SkillSound),
    DamageSound(DamageSound),
}

impl SoundEffectKind {
    fn get_sound_info(&self) -> (&'static str, f32) {
        match self {
            SoundEffectKind::UiSound(ui) => ui.get_sound_info(),
            SoundEffectKind::SkillSound(skill) => skill.get_sound_info(),
            SoundEffectKind::DamageSound(damage) => damage.get_sound_info(),
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
        let (path, life_time) = ev.0.get_sound_info();
        commands.spawn((
            AudioPlayer::<AudioSource>(asset_server.load(path)),
            PlaybackSettings::ONCE,
            LifeTime(Duration::from_secs_f32(life_time)),
        ));
    }
}
