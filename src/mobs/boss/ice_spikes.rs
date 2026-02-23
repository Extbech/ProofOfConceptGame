use std::{f32::consts::TAU, time::Duration};

use bevy::prelude::*;
use rand::Rng;
use test_game::ENEMY_Z;

use crate::{
    characters::player::{AttackCooldown, MaxAttackCooldown},
    mechanics::{
        cooldown::LifeTime,
        damage::{damaging, BaseDamage, Circle, DealDamageHitbox},
    },
    skills::skills::EnemySkills,
    sprites::{Skill, SpriteKind},
    Heading, MovementSpeed,
};

#[derive(Component, Deref, DerefMut, Clone, Copy)]
pub(super) struct IceSpikesCount(pub(super) u32);

fn ice_spikes_bundle(pos: Vec2, angle: f32) -> impl Bundle {
    (
        MovementSpeed(250.),
        Heading::from_angle(angle),
        LifeTime::from_secs_f32(4.),
        damaging(
            BaseDamage(12),
            DealDamageHitbox::Circle(Circle { radius: 15. }),
        ),
        SpriteKind::Skill(Skill::OrbJutsu),
        Transform::from_translation(Vec3::new(pos.x, pos.y, ENEMY_Z)),
        EnemySkills,
    )
}

pub(super) fn spawn_ice_spikes_spell(builder: &mut ChildSpawnerCommands) {
    builder.spawn((
        AttackCooldown(default()),
        MaxAttackCooldown(Duration::from_secs_f32(3.0)),
        IceSpikesCount(6),
        Transform::default(),
    ));
}

pub(super) fn spawn_ice_spikes(
    mut commands: Commands,
    mut query: Query<(
        &GlobalTransform,
        &IceSpikesCount,
        &mut AttackCooldown,
        &MaxAttackCooldown,
    )>,
) {
    for (transform, ice_count, mut ice_cooldown, &max_ice_cooldown) in &mut query {
        for _ in 0..(ice_cooldown.reset(*max_ice_cooldown)) {
            let start_angle: f32 = rand::thread_rng().gen_range(0.0..TAU);
            for n in 1..=**ice_count {
                let angle = start_angle + (n as f32 * (TAU / **ice_count as f32));
                commands.spawn(ice_spikes_bundle(transform.translation().xy(), angle));
            }
        }
    }
}
