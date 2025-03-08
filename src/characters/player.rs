use bevy::prelude::*;
use test_game::{PLAYER_Z, PROJECTILES_Z};

use std::time::Duration;

use crate::mechanics::cooldown::Cooldown;
use crate::mechanics::damage::damaging;
use crate::mechanics::damage::{self, Damage, TakeDamageHitbox};
use crate::mechanics::damage::{Health, HitList};
use crate::mechanics::projectiles::{projectile, ShouldRotate};
use crate::prestige::events::SaveGameStatsEventToMemory;
use crate::sound::events::{PlaySoundEffectEvent, PlayerSound, SkillSound, SoundEffectKind};
use crate::sprites::{Character, Skill, SpriteKind, PLAYER_HEIGHT, PLAYER_WIDTH};
use crate::tools::damage_tracking::DamageTrackerKind;
use crate::{cleanup, CursorTranslation, GameState, MovementSpeed, MyGameCamera};
use crate::{Heading, SCALE};

pub const XP_SCALING_FACTOR: f32 = 25.0;

#[derive(Component)]
pub struct Player;

#[derive(Component, Deref, Clone, Copy)]
pub struct ProjectileSpeed(f32);

#[derive(Component, Deref, DerefMut, Clone, Copy)]
pub struct MaxAttackCooldown(pub Duration);

#[derive(Component, Deref, DerefMut, Clone)]
pub struct AttackCooldown(pub Cooldown);

#[derive(Component, Deref, DerefMut, Clone, Copy)]
pub struct Range(pub f32);

#[derive(Component, Deref, DerefMut, Clone, Copy)]
pub struct CurrentXP(f32);

#[derive(Component, Deref, DerefMut, Clone, Copy)]
pub struct RequiredXP(f32);

#[derive(Component, Deref, DerefMut, Clone, Copy)]
pub struct CurrentLevel(usize);

#[derive(Component, Deref, Clone, Copy)]
pub struct MaxLevel(usize);

#[derive(Component, Deref, DerefMut, Clone, Copy)]
pub struct XpPickUpRadius(f32);

#[derive(Component, Deref, DerefMut, Default)]
pub struct Vulnerability(Cooldown);

#[derive(Component, Deref, DerefMut, Clone, Copy)]
pub struct MaxSpeed(f32);

#[derive(Component, Deref, DerefMut)]
pub struct AttackDirection(Heading);

#[derive(Component, Deref, DerefMut, Clone, Copy)]
pub struct PlayerDamage(u32);

#[derive(Component, Deref, DerefMut, Clone, Copy)]
pub struct MaxHealth(pub u32);

fn projectile_stats(
    damage: PlayerDamage,
    projectile_speed: ProjectileSpeed,
    range: Range,
) -> impl Bundle {
    (damage, projectile_speed, range)
}

fn player_bundle() -> impl Bundle {
    (
        (
            cleanup::ExitGame,
            Player,
            Vulnerability(default()),
            Heading::default(),
            AttackDirection(Heading::new(Vec2::new(0., 1.))),
            MovementSpeed(300.),
            AttackCooldown(default()),
            MaxAttackCooldown(Duration::from_secs_f32(0.5)),
            projectile_stats(PlayerDamage(1), ProjectileSpeed(450.), Range(500.)),
            CurrentXP(0.0),
            RequiredXP(10.0),
            CurrentLevel(1),
            MaxLevel(100),
            XpPickUpRadius(100.0 * SCALE),
            Health(2),
        ),
        MaxHealth(2),
        Transform::from_xyz(0.0, 0.0, PLAYER_Z),
        TakeDamageHitbox(damage::Circle {
            radius: Vec2::new(PLAYER_HEIGHT as f32, PLAYER_WIDTH as f32).length() / 2.,
        }),
        SpriteKind::Character(Character::Warrior),
    )
}

pub fn spawn_player_hero(mut commands: Commands) {
    commands.spawn(player_bundle());
}

pub fn sync_player_and_camera_pos(
    player: Query<&Transform, With<Player>>,
    mut cam: Query<&mut Transform, (With<MyGameCamera>, Without<Player>)>,
) {
    let player = player.single();
    let mut cam = cam.single_mut();
    cam.translation.x = player.translation.x;
    cam.translation.y = player.translation.y;
}

pub fn player_movement(
    keys: Res<ButtonInput<KeyCode>>,
    mut player: Query<(&mut Transform, &mut Heading, &mut Sprite), With<Player>>,
) {
    let (mut player_trans, mut player_dir, mut player_sprite) = player.single_mut();
    let player_position = &mut player_trans.translation;
    let keyboard_dir_x = if keys.pressed(KeyCode::KeyD) { 1. } else { 0. }
        - if keys.pressed(KeyCode::KeyA) { 1. } else { 0. };
    let keyboard_dir_y = if keys.pressed(KeyCode::KeyW) { 1. } else { 0. }
        - if keys.pressed(KeyCode::KeyS) { 1. } else { 0. };
    const BOUND: f32 = 1900.;

    if keyboard_dir_x != 0. {
        if let Some(atlas) = &mut player_sprite.texture_atlas {
            atlas.index = 3;
        }
        player_sprite.flip_x = keyboard_dir_x == -1.;
    } else if keyboard_dir_y == 1. {
        if let Some(atlas) = &mut player_sprite.texture_atlas {
            atlas.index = 2;
        }
    } else if keyboard_dir_y == -1. {
        if let Some(atlas) = &mut player_sprite.texture_atlas {
            atlas.index = 1;
        }
    }
    (player_position.x, player_position.y) = player_position
        .xy()
        .clamp(-Vec2::splat(BOUND), Vec2::splat(BOUND))
        .into();
    *player_dir = Heading::new(Vec2::new(keyboard_dir_x, keyboard_dir_y));
}

pub fn player_attack_facing_from_mouse(
    mut player: Query<(&Transform, &mut AttackDirection), With<Player>>,
    cursor_pos: Res<CursorTranslation>,
) {
    let (&player_trans, mut attack_direction) = player.single_mut();
    let player_position = &mut player_trans.translation.xy();
    *attack_direction = AttackDirection(Heading::new(**cursor_pos - *player_position));
}

/// System for shooting where the direction of the projectiles go from the player towards the cursor
pub fn player_shooting(
    mut commands: Commands,
    keys: Res<ButtonInput<MouseButton>>,
    mut player: Query<
        (
            &Transform,
            &ProjectileSpeed,
            &mut AttackCooldown,
            &MaxAttackCooldown,
            &PlayerDamage,
            &Range,
            &AttackDirection,
        ),
        With<Player>,
    >,
    mut event: EventWriter<PlaySoundEffectEvent>,
) {
    let (
        &player_trans,
        &projectile_speed,
        mut attack_cooldown,
        &max_attack_cooldown,
        &damage,
        &range,
        attack_direction,
    ) = player.single_mut();
    let player_position = &mut player_trans.translation.xy();
    if keys.pressed(MouseButton::Left) {
        for _ in 0..(attack_cooldown.reset(*max_attack_cooldown)) {
            player_shoot(
                &mut commands,
                *player_position,
                attack_direction,
                projectile_speed,
                damage,
                range,
                &mut event,
            );
        }
    } else {
        attack_cooldown.wait();
    }
}

fn player_shoot(
    commands: &mut Commands,
    player_position: Vec2,
    dir: &Heading,
    projectile_speed: ProjectileSpeed,
    damage: PlayerDamage,
    range: Range,
    sound_event: &mut EventWriter<PlaySoundEffectEvent>,
) {
    let diff = player_position - **dir;
    commands.spawn((
        projectile(
            *dir,
            MovementSpeed(*projectile_speed),
            range,
            ShouldRotate(true),
        ),
        damaging(
            Damage(*damage),
            damage::DealDamageHitBox::Circle(damage::Circle { radius: 10. }),
        ),
        SpriteKind::Skill(Skill::PrimaryAttack),
        Transform::from_xyz(player_position.x, player_position.y, PROJECTILES_Z).with_rotation(
            Quat::from_axis_angle(Vec3::new(0., 0., 1.0), diff.y.atan2(diff.x)),
        ),
        HitList::default(),
        DamageTrackerKind::PrimaryAttack,
    ));
    sound_event.send(PlaySoundEffectEvent(SoundEffectKind::SkillSound(
        SkillSound::PrimaryAttack,
    )));
}

pub fn handle_player_xp(
    mut query: Query<
        (
            &mut CurrentXP,
            &mut RequiredXP,
            &mut CurrentLevel,
            &MaxLevel,
        ),
        With<Player>,
    >,
    mut game_state: ResMut<NextState<GameState>>,
    mut save_game_stats_event: EventWriter<SaveGameStatsEventToMemory>,
    mut sound_event: EventWriter<PlaySoundEffectEvent>,
) {
    let (mut current_xp, mut required_xp, mut current_level, max_level) = query.single_mut();
    if **required_xp <= **current_xp && **current_level < **max_level {
        **current_level += 1;
        **current_xp -= **required_xp;
        **required_xp += XP_SCALING_FACTOR;
        game_state.set(GameState::LevelUp);
        save_game_stats_event.send(SaveGameStatsEventToMemory);
        sound_event.send(PlaySoundEffectEvent(SoundEffectKind::PlayerSound(
            PlayerSound::Levelup,
        )));
    }
}

pub fn handle_player_death(
    player_query: Query<&Health, With<Player>>,
    mut game_state: ResMut<NextState<GameState>>,
) {
    let player_health = player_query.single();
    if **player_health == 0 {
        game_state.set(GameState::Loss);
    }
}
