use std::time::Duration;

use crate::characters::player::MaxHealth;
use crate::loot::coin::handle_coin_pickup;
use crate::loot::xp::activate_all_xp_orbs;
use crate::mechanics::cooldown::LifeTime;
use crate::mechanics::damage::{damaging, BaseDamage, DealDamageHitbox, Health, HitList};
use crate::prestige::stats::Stats;
use crate::sprites::{Item, SpriteKind};
use crate::SCALE;
use crate::{characters::player::Player, cleanup, mobs::enemy::Enemy, GameRng};
use bevy::prelude::*;
use rand::prelude::*;
use test_game::LOOT_DROPS_Z;

use super::coin::spawn_coin;
use super::xp::{spawn_xp, MagnetActive};

#[derive(Component, Deref, DerefMut)]
pub struct LootId(pub u32);

fn spawn_loot(id: u32, sprite_kind: SpriteKind, x: f32, y: f32) -> impl Bundle {
    (
        cleanup::ExitGame,
        LootId(id),
        Transform::from_xyz(x, y, LOOT_DROPS_Z),
        sprite_kind,
    )
}

#[derive(Component)]
pub struct AnimationIndices {
    pub first: usize,
    pub second: usize,
    pub third: usize,
    pub fourth: usize,
}

#[derive(Component, Deref, DerefMut)]
pub struct AnimationTimer(pub Timer);

/// Spawns loot.
fn try_spawn_loot(rng: &mut ResMut<GameRng>, commands: &mut Commands, pos: Vec3) {
    let loot_id = rng.gen_range(0..10);
    if loot_id == 3 {
        commands.spawn(spawn_coin(loot_id, pos.x, pos.y));
        return;
    }
    let sprite_kind = match loot_id {
        0 => SpriteKind::Item(Item::Potion),
        1 => SpriteKind::Item(Item::ThorsHammer),
        2 => SpriteKind::Item(Item::Magnet),
        _ => return,
    };
    commands.spawn(spawn_loot(loot_id, sprite_kind, pos.x, pos.y));
}

/// Checks for dead enemies and will spawn loot accordingly.
pub fn check_for_dead_enemies(
    mut commands: Commands,
    query: Query<(&Transform, Entity, &Health), With<Enemy>>,
    mut rng: ResMut<GameRng>,
) {
    for (transform, entity, health) in query.iter() {
        if **health == 0 {
            commands.entity(entity).despawn_recursive();
            // 1/5 -> 20%
            commands.spawn(spawn_xp(
                10.,
                transform.translation.x,
                transform.translation.y,
            ));
            try_spawn_loot(&mut rng, &mut commands, transform.translation);
        }
    }
}

pub fn is_collision(obj1: Vec2, obj2: Vec2, obj1_radius: f32, obj2_radius: f32) -> bool {
    let diff = (obj1 - obj2).length();
    if diff < obj1_radius + obj2_radius {
        return true;
    }
    false
}

pub fn pickup_loot(
    mut commands: Commands,
    mut query_player: Query<(&Transform, &mut Health, &MaxHealth), With<Player>>,
    query_loot: Query<(&Transform, &LootId, Entity)>,
    mut query_xp: Query<&mut MagnetActive>,
    mut stats: ResMut<Stats>,
) {
    let (player_trans, mut health, max_health) = query_player.single_mut();
    let player_pos = player_trans.translation.xy();
    for (loot_trans, loot, ent) in &query_loot {
        let loot_position = loot_trans.translation.xy();
        const ITEM_PICKUP_RANGE: f32 = 50.;
        if is_collision(player_pos, loot_position, ITEM_PICKUP_RANGE * SCALE, 0.) {
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
                3 => {
                    handle_coin_pickup(&mut stats);
                }
                _ => unreachable!("Invalid loot id"),
            }
            commands.entity(ent).despawn_recursive();
        }
    }
}

pub fn spawn_bomb(commands: &mut Commands, pos: Vec2) {
    commands.spawn((
        damaging(BaseDamage(1000), DealDamageHitbox::Global),
        LifeTime(Duration::from_secs_f32(1.)),
        HitList::default(),
        Transform {
            translation: Vec3::new(pos.x, pos.y, LOOT_DROPS_Z),
            ..default()
        },
    ));
}
