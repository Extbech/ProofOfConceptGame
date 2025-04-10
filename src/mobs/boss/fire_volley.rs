use std::f32::consts::TAU;

use bevy::prelude::*;
use rand::Rng;
use test_game::ENEMY_Z;

use crate::{
    characters::player::AttackCooldown,
    mechanics::{
        cooldown::{Cooldown, LifeTime},
        damage::{damaging, BaseDamage, Circle, DealDamageHitbox},
        movement::orbiting::{Angle, AngularVelocity},
    },
    mobs::enemy::Enemy,
    sprites::{Skill, SpriteKind},
    Heading, MovementSpeed,
};

#[derive(Component, Deref, DerefMut, Clone, Copy)]
pub(super) struct FireVolleyCount(pub(super) u32);

fn fire_volley_bundle(pos: Vec2, angle: f32) -> impl Bundle {
    (
        MovementSpeed(200.),
        AngularVelocity(1.),
        Heading::from_angle(angle),
        LifeTime::from_secs_f32(3.),
        damaging(
            BaseDamage(10),
            DealDamageHitbox::Circle(Circle { radius: 20. }),
        ),
        SpriteKind::Skill(Skill::FireBall),
        Transform::from_translation(Vec3::new(pos.x, pos.y, ENEMY_Z)),
        Enemy,
    )
}

pub(super) fn spawn_fire_volley_spell(builder: &mut ChildBuilder) {
    builder.spawn((
        AttackCooldown(Cooldown::new(5.)),
        FireVolleyCount(4),
        Transform::default(),
    ));
}

pub(super) fn spawn_fire_volley(
    mut commands: Commands,
    mut query: Query<(&GlobalTransform, &FireVolleyCount, &mut AttackCooldown)>,
) {
    for (transform, fv_count, mut fv_cooldown) in &mut query {
        for _ in 0..(fv_cooldown.reset()) {
            let start_angle: f32 = rand::thread_rng().gen_range(0.0..TAU);
            for n in 1..=**fv_count {
                let angle = start_angle + (n as f32 * (TAU / **fv_count as f32));
                let x = 20.0 * angle.sin() + transform.translation().x;
                let y = 20.0 * angle.cos() + transform.translation().y;
                commands.spawn(fire_volley_bundle(transform.translation().xy(), angle));
            }
        }
    }
}
