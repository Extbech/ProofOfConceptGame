use bevy::{
    ecs::{bundle::Bundle, component::Component},
    math::Vec3,
    transform::components::Transform,
};
use test_game::PROJECTILES_Z;

use crate::{
    characters::player::AttackCooldown,
    mechanics::{
        cooldown::{Cooldown, LifeTime},
        damage::{damaging, BaseDamage, Circle, DealDamageHitbox},
    },
    sprites::{Skill, SpriteKind},
    tools::damage_tracking::DamageTrackerKind,
};

pub fn thors_lightning_bundle() -> impl Bundle {
    (
        AttackCooldown(Cooldown::new(2.)),
        damaging(
            BaseDamage(10),
            DealDamageHitbox::Circle(Circle { radius: 100.0 }),
        ),
        DamageTrackerKind::Lightning,
        Transform::default(),
    )
}

#[derive(Component)]
pub struct LightningEffectMarker;

/// TODO: Should be spawned on lightning hit...
pub fn thors_lightning_strike_bundle(x: f32, y: f32) -> impl Bundle {
    (
        Transform::from_translation(Vec3::new(x, y, PROJECTILES_Z)),
        LightningEffectMarker,
        LifeTime::from_secs_f32(0.3),
        SpriteKind::Skill(Skill::LightningAttack),
    )
}
