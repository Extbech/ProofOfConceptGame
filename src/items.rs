use bevy::prelude::*;
use std::{f32::consts::TAU, time::Duration};
use test_game::{LOOT_DROPS_Z, PROJECTILES_Z};

use crate::{
    cooldown::LifeTime,
    damage::{is_collision, Damage, DamagingBundle, Health, HitList, Radius},
    enemy::Enemy,
    loot::{activate_all_xp_orbs, LootId, XPActive, XP},
    player::{AttackCooldown, MaxAttackCooldown, MaxHealth, Player},
    projectiles::{Angle, AngularVelocity, OrbitalRadius, OrbitingBundle},
};

pub fn spawn_bomb(commands: &mut Commands, pos: Vec2) {
    commands.spawn((
        DamagingBundle {
            damage: Damage(100),
            radius: Radius(1000.),
        },
        LifeTime(Duration::from_secs_f32(1.)),
        HitList(vec![]),
        SpatialBundle {
            transform: Transform {
                translation: Vec3::new(pos.x, pos.y, LOOT_DROPS_Z),
                ..default()
            },
            ..default()
        },
    ));
}

/// Orb sprite is from: https://opengameart.org/content/pixel-orbs
pub fn pickup_loot(
    mut commands: Commands,
    mut query_player: Query<(&Transform, &mut Health, &MaxHealth), With<Player>>,
    query_loot: Query<(&Transform, &LootId, Entity)>,
    mut query_xp: Query<&mut XPActive, With<XP>>,
) {
    let (player_trans, mut health, max_health) = query_player.single_mut();
    let player_pos = player_trans.translation.xy();
    for (loot_trans, loot, ent) in &query_loot {
        let loot_position = loot_trans.translation.xy();
        if is_collision(player_pos, loot_position, 100., 0.) {
            match **loot {
                0 => {
                    if **health < **max_health {
                        **health += 1;
                    } else {
                        continue;
                    }
                }
                1 => {
                    spawn_bomb(&mut commands, loot_position);
                }
                2 => {
                    activate_all_xp_orbs(&mut query_xp);
                }
                _ => unreachable!("Invalid loot id"),
            }
            commands.entity(ent).despawn_recursive();
        }
    }
}
/// This func handles correct angle distance between orb projectiles.
pub fn spawn_new_orb(
    commands: &mut Commands,
    player_entity: Entity,
    query_orb: &mut Query<Entity, With<OrbitalRadius>>,
    asset_server: &Res<AssetServer>,
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
                Angle(0.0)
            } else {
                Angle((TAU / orb_counter as f32) * i as f32)
            };
            println!("orb: {} with angle. {}", i, *angle);
            parent.spawn((
                OrbitingBundle {
                    vel: AngularVelocity(3.),
                    radius: OrbitalRadius(200.),
                    angle,
                    sprite: {
                        SpriteBundle {
                            texture: asset_server.load("loot/orb_purple.png"),
                            sprite: Sprite {
                                custom_size: Some(Vec2::new(70., 70.)),
                                ..Default::default()
                            },
                            ..default()
                        }
                    },
                    ..default()
                },
                DamagingBundle {
                    damage: Damage(2),
                    radius: Radius(50.),
                },
                HitList(vec![]), // TODO: Remove this and add a timing based system for orbiting damagers instead
            ));
        }
    });
}

pub fn enable_thors_lightning_skill(commands: &mut Commands, player_entity: Entity) {
    commands.entity(player_entity).with_children(|child| {
        child.spawn(ThorsLightningBundle {
            attack_cooldown: AttackCooldown(default()),
            max_cooldown: MaxAttackCooldown(Duration::from_secs_f32(5.0)),
            damage: Damage(3),
            range: Radius(500.0),
            marker: ThorLightningMarker,
        });
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
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
    asset_server: Res<AssetServer>,
) {
    let texture_handle: Handle<Image> = asset_server.load("effects/lightning-strike.png");
    let layout = TextureAtlasLayout::from_grid(Vec2::new(22.0, 59.0), 2, 1, None, None);
    let texture_atlas_layout = texture_atlas_layouts.add(layout);

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
                commands.spawn((
                    SpriteSheetBundle {
                        texture: texture_handle.clone(),
                        atlas: TextureAtlas {
                            layout: texture_atlas_layout.clone(),
                            index: 0,
                        },
                        transform: Transform::from_translation(Vec3::new(
                            enemy_transform.translation.x,
                            enemy_transform.translation.y,
                            PROJECTILES_Z,
                        )),
                        ..default()
                    },
                    LightningEffectMarker,
                    LifeTime::from_secs_f32(0.06),
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

#[derive(Component)]
pub struct LightningEffectMarker;

pub fn animate_lightning(
    mut lightning_query: Query<(&LifeTime, &mut TextureAtlas), With<LightningEffectMarker>>,
) {
    for (lifetime, mut atlas) in &mut lightning_query {
        if 0.2 <= lifetime.as_secs_f32() && lifetime.as_secs_f32() < 0.4 {
            atlas.index = 1;
        } else if 0.4 <= lifetime.as_secs_f32() {
            atlas.index = 0;
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
#[derive(Component)]
pub struct PassiveDamageIncrease(pub u8);

#[derive(Component)]
pub struct PassiveMovementSpeedIncrease(pub u8);

#[derive(Component)]
pub struct PassivePickUpRadiusIncrease(pub u8);

#[derive(Component)]
pub struct PassiveHealthIncrease(pub u8);

#[derive(Component)]
pub struct ActiveOrbitingOrb(pub u8);

#[derive(Component)]
pub struct ActiveThorLightning(pub u8);

#[derive(Component)]
pub struct ThorLightningMarker;

#[derive(Bundle)]
pub struct ThorsLightningBundle {
    attack_cooldown: AttackCooldown,
    max_cooldown: MaxAttackCooldown,
    damage: Damage,
    range: Radius,
    marker: ThorLightningMarker,
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
