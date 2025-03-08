use std::time::Duration;

use bevy::{
    ecs::{bundle::Bundle, component::Component},
    math::Vec3,
    transform::components::Transform,
    utils::default,
};
use test_game::PROJECTILES_Z;

use crate::{
    characters::player::{AttackCooldown, MaxAttackCooldown},
    mechanics::{
        cooldown::LifeTime,
        damage::{Circle, Damage, DealDamageHitBox},
    },
    sprites::{Skill, SpriteKind},
    tools::damage_tracking::DamageTrackerKind,
};

pub fn thors_lightning_bundle() -> impl Bundle {
    (
        AttackCooldown(default()),
        MaxAttackCooldown(Duration::from_secs_f32(5.0)),
        Damage(3),
        DealDamageHitBox::Circle(Circle { radius: 100.0 }),
        DamageTrackerKind::Lightning,
        Transform::default(),
    )
}

#[derive(Component)]
pub struct LightningEffectMarker;

pub fn thors_lightning_strike_bundle(x: f32, y: f32) -> impl Bundle {
    (
        Transform::from_translation(Vec3::new(x, y, PROJECTILES_Z)),
        LightningEffectMarker,
        LifeTime::from_secs_f32(0.3),
        SpriteKind::Skill(Skill::LightningAttack),
    )
}
