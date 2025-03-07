use bevy::ecs::bundle::Bundle;

use crate::{
    mechanics::{
        damage::{damaging, Damage, EntityHitCooldown, Radius},
        projectiles::{Angle, AngularVelocity, OrbitalRadius},
    },
    sprites::sprites::{Skill, SpriteKind},
    SCALE,
};

pub fn spawn_orb_jutsu_entity(angle: f32) -> impl Bundle {
    (
        AngularVelocity(3.),
        Angle(angle),
        OrbitalRadius(200. * SCALE),
        damaging(Damage(2), Radius(20.)),
        EntityHitCooldown::default(),
        SpriteKind::Skill(Skill::OrbJutsu),
    )
}
