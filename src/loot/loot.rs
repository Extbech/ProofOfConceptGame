use std::time::Duration;

use crate::characters::player::MaxHealth;
use crate::mechanics::cooldown::LifeTime;
use crate::mechanics::damage::{damaging, Damage, DealDamageHitbox, Health, HitList};
use crate::sprites::{Item, SpriteKind};
use crate::SCALE;
use crate::{
    characters::player::{CurrentXP, Player, XpPickUpRadius},
    cleanup,
    mobs::enemy::Enemy,
    GameRng, MovementSpeed,
};
use bevy::prelude::*;
use rand::prelude::*;
use test_game::LOOT_DROPS_Z;

#[derive(Component, Deref, Clone, Copy)]
pub struct XP(f32);

#[derive(Component, Deref, DerefMut, Clone, Copy)]
pub struct XPActive(pub bool);

pub fn spawn_xp(xp_reward: f32, x: f32, y: f32) -> impl Bundle {
    (
        cleanup::ExitGame,
        XP(xp_reward),
        AnimationTimer(Timer::from_seconds(0.1, TimerMode::Repeating)),
        AnimationIndices {
            first: 0,
            second: 1,
            third: 2,
            fourth: 3,
        },
        MovementSpeed(500.0),
        XPActive(false),
        Transform::from_xyz(x, y, LOOT_DROPS_Z),
        SpriteKind::Item(Item::XPOrb),
    )
}

#[derive(Component, Deref, DerefMut)]
pub struct LootId(u32);

pub fn spawn_loot(id: u32, sprite_kind: SpriteKind, x: f32, y: f32) -> impl Bundle {
    (
        cleanup::ExitGame,
        LootId(id),
        Transform::from_xyz(x, y, LOOT_DROPS_Z),
        sprite_kind,
    )
}

#[derive(Component)]
pub struct AnimationIndices {
    first: usize,
    second: usize,
    third: usize,
    fourth: usize,
}

#[derive(Component, Deref, DerefMut)]
pub struct AnimationTimer(Timer);

/// Spawns loot.
pub fn try_spawn_loot(rng: &mut ResMut<GameRng>, commands: &mut Commands, pos: Vec3) {
    let loot_id = rng.gen_range(0..10);
    let sprite_kind = match loot_id {
        0 => SpriteKind::Item(Item::Potion),
        1 => SpriteKind::Item(Item::ThorsHammer),
        2 => SpriteKind::Item(Item::XPMagnet),
        _ => return,
    };
    commands.spawn(spawn_loot(loot_id, sprite_kind, pos.x, pos.y));
}

/// Responsible for spawning the xp orbs and setting up the animation sequence.
pub fn spawn_xp_orb(commands: &mut Commands, pos: Vec3) {
    commands.spawn(spawn_xp(10.0, pos.x, pos.y));
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
            spawn_xp_orb(&mut commands, transform.translation);
            try_spawn_loot(&mut rng, &mut commands, transform.translation);
        }
    }
}

/// Handles the animation sequence for the xp orbs.
pub fn animate_sprite(
    time: Res<Time>,
    mut query: Query<(&AnimationIndices, &mut AnimationTimer, &mut Sprite)>,
) {
    for (indices, mut timer, mut sprite) in &mut query {
        if let Some(atlas) = &mut sprite.texture_atlas {
            timer.tick(time.delta());
            if timer.just_finished() {
                match atlas.index {
                    0 => atlas.index = indices.second,
                    1 => atlas.index = indices.third,
                    2 => atlas.index = indices.fourth,
                    3 => atlas.index = indices.first,
                    _ => unreachable!("Inavlid animation index for this entity."),
                }
            }
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

/// Handles the event where the xp orb is close enough to the player to be consumed.
pub fn xp_orbs_collision(
    mut commands: Commands,
    mut player_query: Query<(&Transform, &mut CurrentXP), With<Player>>,
    mut xp_query: Query<(&Transform, &XP, Entity), With<XP>>,
) {
    let (player_transform, mut current_xp) = player_query.single_mut();
    for (xp_transform, xp, entity) in xp_query.iter_mut() {
        if is_collision(
            player_transform.translation.xy(),
            xp_transform.translation.xy(),
            10.0,
            0.0,
        ) {
            **current_xp += **xp;
            commands.entity(entity).despawn_recursive();
            // TODO: play sound effect for xp pickup may add good game feel or it might be annoying (?)
        }
    }
}

/// Causes xp orb entities to start moving towards the player if they are within pick up radius.
/// **NB** Make sure xp_orb movement speed is greater than the players, if not theoretically the player can always outrun the orbs.
pub fn activate_xp_orb_movement(
    mut player_query: Query<(&Transform, &XpPickUpRadius), With<Player>>,
    mut xp_query: Query<(&Transform, &mut XPActive), (With<XP>, Without<Player>)>,
) {
    let (player_trasnform, pick_up_radius) = player_query.single_mut();
    for (xp_transform, mut active) in xp_query.iter_mut() {
        if is_collision(
            player_trasnform.translation.xy(),
            xp_transform.translation.xy(),
            **pick_up_radius,
            0.0,
        ) {
            **active = true;
        }
    }
}

pub fn handle_xp_orb_movement(
    mut player_query: Query<&Transform, With<Player>>,
    mut xp_query: Query<(&mut Transform, &MovementSpeed, &XPActive), (With<XP>, Without<Player>)>,
    time: Res<Time>,
) {
    let player_trasnform = player_query.single_mut();
    for (mut xp_transform, speed, active) in xp_query.iter_mut() {
        if **active {
            let xp_pos = xp_transform.translation.xy();
            let player_pos = player_trasnform.translation.xy();
            xp_transform.translation = (xp_pos
                - (xp_pos - player_pos).normalize_or_zero() * time.delta_secs() * **speed)
                .extend(xp_transform.translation.z);
        }
    }
}

pub fn activate_all_xp_orbs(q: &mut Query<&mut XPActive, With<XP>>) {
    for mut active in q {
        **active = true;
    }
}

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
                _ => unreachable!("Invalid loot id"),
            }
            commands.entity(ent).despawn_recursive();
        }
    }
}

pub fn spawn_bomb(commands: &mut Commands, pos: Vec2) {
    commands.spawn((
        damaging(Damage(100), DealDamageHitbox::Global),
        LifeTime(Duration::from_secs_f32(1.)),
        HitList::default(),
        Transform {
            translation: Vec3::new(pos.x, pos.y, LOOT_DROPS_Z),
            ..default()
        },
    ));
}
