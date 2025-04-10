use bevy::ecs::bundle::Bundle;

use crate::{
    mechanics::{
        damage::{damaging, BaseDamage, Circle, DealDamageHitbox, EntityHitCooldown},
        movement::orbiting::{Angle, AngularVelocity, OrbitalRadius},
    },
    sprites::{Skill, SpriteKind},
    tools::damage_tracking::DamageTrackerKind,
    SCALE,
};

pub fn orb_jutsu_bundle(angle: f32) -> impl Bundle {
    (
        AngularVelocity(3.),
        Angle(angle),
        OrbitalRadius(200. * SCALE),
        damaging(
            BaseDamage(10),
            DealDamageHitbox::Circle(Circle { radius: 20. }),
        ),
        EntityHitCooldown::default(),
        SpriteKind::Skill(Skill::OrbJutsu),
        DamageTrackerKind::OrbJutsu,
    )
}
