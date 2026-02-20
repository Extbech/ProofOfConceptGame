use bevy::prelude::*;
use std::f32::consts::TAU;

use crate::{
    mechanics::{cooldown::LifeTime, movement::orbiting::OrbitalRadius},
    skills::bundles::{
        orb_jutsu::orb_jutsu_bundle,
        thors_lightning::{thors_lightning_bundle, LightningEffectMarker},
    },
};

#[derive(Component)]
pub struct EnemySkills;

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
        commands.entity(entity).despawn();
    }
    commands.entity(player_entity).with_children(|parent| {
        for i in 0..orb_counter {
            let angle = if i == 0 {
                0.0
            } else {
                (TAU / orb_counter as f32) * i as f32
            };
            parent.spawn(orb_jutsu_bundle(angle));
        }
    });
}

pub fn enable_thors_lightning_skill(commands: &mut Commands, player_entity: Entity) {
    commands.entity(player_entity).with_children(|child| {
        child.spawn(thors_lightning_bundle());
    });
}
/*
pub fn spawn_lightning(
    mut commands: Commands,
    mut damage_tracker: ResMut<DamageTracker>,
    player_query: Query<&Transform, With<Player>>,
    mut enemy_query: Query<(&mut Health, &Transform), With<Enemy>>,
    asset_server: Res<AssetServer>,
    mut lightning_query: Query<
        (
            &mut AttackCooldown,
            &MaxAttackCooldown,
            &DealDamageHitBox,
            &Damage,
            &DamageTrackerKind,
            &GlobalTransform,
        ),
        With<ThorLightningMarker>,
    >,
) {
    let Some((mut attack_cd, max_attack_cd, hitbox, damage, damage_tracker_kind, tf)) =
        lightning_query.iter_mut().next()
    else {
        return;
    };
    for _ in 0..attack_cd.reset(**max_attack_cd) {
        let found = false;
        for (mut enemy_health, enemy_transform) in &mut enemy_query {
            if overlapping(
                *hitbox,
                tf.translation().xy(),
                TakeDamageHitbox(Circle { radius: 0. }),
                enemy_transform.translation.xy(),
            ) {
                commands.spawn(thors_lightning_strike_bundle(
                    enemy_transform.translation.x,
                    enemy_transform.translation.y,
                    *damage,
                ));
                spawn_damage_text(
                    &mut commands,
                    damage,
                    &asset_server,
                    enemy_transform.translation.xy(),
                );
                **enemy_health = enemy_health.saturating_sub(**damage);
                damage_tracker.update(*damage_tracker_kind, **damage);
                break;
            }
        }
        if !found {
            attack_cd.wait();
        }
    }
}
*/
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
