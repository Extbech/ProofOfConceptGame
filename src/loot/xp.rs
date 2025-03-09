use bevy::{
    ecs::{
        bundle::Bundle,
        component::Component,
        entity::Entity,
        query::{With, Without},
        system::{Commands, Query, Res},
    },
    hierarchy::DespawnRecursiveExt,
    math::Vec3Swizzles,
    prelude::{Deref, DerefMut},
    sprite::Sprite,
    time::{Time, Timer, TimerMode},
    transform::components::Transform,
};
use test_game::LOOT_DROPS_Z;

use crate::{
    characters::player::{CurrentXP, Player, XpPickUpRadius},
    cleanup,
    sprites::{Item, SpriteKind},
    MovementSpeed,
};

use super::loot::{is_collision, AnimationIndices, AnimationTimer};

#[derive(Component, Deref, Clone, Copy)]
pub struct XP(f32);

#[derive(Component, Deref, DerefMut, Clone, Copy)]
pub struct MagnetActive(pub bool);

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
        MagnetActive(false),
        Transform::from_xyz(x, y, LOOT_DROPS_Z),
        SpriteKind::Item(Item::XPOrb),
    )
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
    mut xp_query: Query<(&Transform, &mut MagnetActive), (With<XP>, Without<Player>)>,
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
    mut xp_query: Query<(&mut Transform, &MovementSpeed, &MagnetActive), Without<Player>>,
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

pub fn activate_all_xp_orbs(q: &mut Query<&mut MagnetActive>) {
    for mut active in q {
        **active = true;
    }
}
