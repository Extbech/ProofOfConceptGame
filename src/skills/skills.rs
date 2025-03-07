use bevy::prelude::*;
use std::f32::consts::TAU;

use crate::{
    characters::player::{AttackCooldown, MaxAttackCooldown, Player},
    mechanics::{
        cooldown::LifeTime,
        damage::{is_collision, Damage, Health, Radius},
        projectiles::OrbitalRadius,
    },
    mobs::enemy::Enemy,
    skills::entities::{
        orb_jutsu::spawn_orb_jutsu_entity,
        thors_lightning::{
            thors_lightning, thors_lightning_strike, LightningEffectMarker, ThorLightningMarker,
        },
    },
};

/// This func handles correct angle distance between orb projectiles.
pub fn spawn_new_orb(
    commands: &mut Commands,
    player_entity: Entity,
    query_orb: &mut Query<Entity, With<OrbitalRadius>>,
) {
    // will at least spawn 1 orb.
    let mut orb_counter = 1;
    for entity in query_orb {
        orb_counter += 1;
        commands.entity(entity).despawn_recursive();
    }
    commands.entity(player_entity).with_children(|parent| {
        for i in 0..orb_counter {
            let angle = if i == 0 {
                0.0
            } else {
                (TAU / orb_counter as f32) * i as f32
            };
            parent.spawn(spawn_orb_jutsu_entity(angle));
        }
    });
}

pub fn enable_thors_lightning_skill(commands: &mut Commands, player_entity: Entity) {
    commands.entity(player_entity).with_children(|child| {
        child.spawn(thors_lightning());
    });
}

pub fn spawn_lightning(
    mut commands: Commands,
    player_query: Query<&Transform, With<Player>>,
    mut enemy_query: Query<(&mut Health, &Transform), With<Enemy>>,
    mut lightning_query: Query<
        (&mut AttackCooldown, &MaxAttackCooldown, &Radius, &Damage),
        With<ThorLightningMarker>,
    >,
) {
    let Some((mut attack_cd, max_attack_cd, radius, damage)) = lightning_query.iter_mut().next()
    else {
        return;
    };
    let player_transform = player_query.single();
    for _ in 0..attack_cd.reset(**max_attack_cd) {
        let found = false;
        for (mut enemy_health, enemy_transform) in &mut enemy_query {
            if is_collision(
                player_transform.translation.xy(),
                enemy_transform.translation.xy(),
                0.0,
                **radius,
            ) {
                commands.spawn(thors_lightning_strike(
                    enemy_transform.translation.x,
                    enemy_transform.translation.y,
                ));
                **enemy_health = enemy_health.saturating_sub(**damage);
                break;
            }
        }
        if !found {
            attack_cd.wait();
        }
    }
}

pub fn animate_lightning(
    mut lightning_query: Query<(&LifeTime, &mut Sprite), With<LightningEffectMarker>>,
) {
    for (lifetime, mut sprite) in &mut lightning_query {
        if let Some(atlas) = &mut sprite.texture_atlas {
            if 0.1 <= lifetime.as_secs_f32() && lifetime.as_secs_f32() < 0.2 {
                atlas.index = 1;
            } else if 0.2 <= lifetime.as_secs_f32() {
                atlas.index = 0;
            }
        }
    }
}
