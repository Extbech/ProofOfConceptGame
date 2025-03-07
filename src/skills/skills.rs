use bevy::prelude::*;
use std::{f32::consts::TAU, time::Duration};
use test_game::LOOT_DROPS_Z;

use crate::{
    characters::player::{AttackCooldown, MaxAttackCooldown, Player},
    mechanics::{
        cooldown::LifeTime,
        damage::{damaging, is_collision, Damage, Health, HitList, Radius},
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

#[derive(Component, PartialEq, Eq, Hash, Copy, Clone)]
pub enum ItemType {
    PassiveDamageIncrease,
    PassiveMovementSpeedIncrease,
    PassivePickUpRadiusIncrease,
    PassiveHealthIncrease,
    ActiveOrbitingOrb,
    ActiveThorLightning,
}

#[derive(Resource, Deref)]
pub struct ItemTooltips(pub [(ItemType, &'static str, &'static str); 6]);

impl Default for ItemTooltips {
    fn default() -> Self {
        ItemTooltips([
            (
                ItemType::PassiveDamageIncrease,
                "Damage",
                "Increase All Damage Done By 10%.",
            ),
            (
                ItemType::PassiveMovementSpeedIncrease,
                "Movement Speed",
                "Increase Movement Speed By 10%.",
            ),
            (
                ItemType::PassivePickUpRadiusIncrease,
                "Pick Up Radius",
                "Increase Pickup Radius By 10%.",
            ),
            (
                ItemType::ActiveOrbitingOrb,
                "Orb Jutsu",
                "Spawn an orbiting orb that damages enemies it comes in contact with.",
            ),
            (
                ItemType::PassiveHealthIncrease,
                "Vitality",
                "Increase max health by 1.",
            ),
            (
                ItemType::ActiveThorLightning,
                "Thor's Lightning",
                "Lightning randomly strikes nearby enemies.",
            ),
        ])
    }
}
