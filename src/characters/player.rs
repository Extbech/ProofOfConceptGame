use bevy::prelude::*;
use test_game::{PLAYER_Z, PROJECTILES_Z};

use std::time::Duration;

use crate::mechanics::cooldown::Cooldown;
use crate::mechanics::damage::{self, TakeDamageHitbox};
use crate::mechanics::damage::{damaging, BaseDamage};
use crate::mechanics::damage::{Health, HitList};
use crate::mechanics::movement::{projectile, ShouldRotate};
use crate::sound::events::{PlaySoundEffectEvent, PlayerSound, SkillSound, SoundEffectKind};
use crate::sprites::{Character, Skill, SpriteKind, PLAYER_HEIGHT, PLAYER_WIDTH};
use crate::tools::damage_tracking::DamageTrackerKind;
use crate::{cleanup, CursorTranslation, GameState, MovementSpeed};
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
            projectile_stats(PlayerDamage(10), ProjectileSpeed(450.), Range(500.)),
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

pub fn player_attack_facing_from_mouse(
    mut player: Query<(&Transform, &mut AttackDirection), With<Player>>,
    cursor_pos: Res<CursorTranslation>,
) {
    let (&player_trans, mut attack_direction) = player.single_mut().expect("no player!");
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
    mut event: MessageWriter<PlaySoundEffectEvent>,
) {
    let (
        &player_trans,
        &projectile_speed,
        mut attack_cooldown,
        &max_attack_cooldown,
        &damage,
        &range,
        attack_direction,
    ) = player.single_mut().expect("Err");
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
    base_damage: PlayerDamage,
    range: Range,
    sound_event: &mut MessageWriter<PlaySoundEffectEvent>,
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
            BaseDamage(*base_damage),
            damage::DealDamageHitbox::Circle(damage::Circle { radius: 10. }),
        ),
        SpriteKind::Skill(Skill::PrimaryAttack),
        Transform::from_xyz(player_position.x, player_position.y, PROJECTILES_Z).with_rotation(
            Quat::from_axis_angle(Vec3::new(0., 0., 1.0), diff.y.atan2(diff.x)),
        ),
        HitList::default(),
        DamageTrackerKind::PrimaryAttack,
    ));
    sound_event.write(PlaySoundEffectEvent(SoundEffectKind::Skill(
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
    mut sound_event: MessageWriter<PlaySoundEffectEvent>,
) {
    let (mut current_xp, mut required_xp, mut current_level, max_level) =
        query.single_mut().expect("err");
    if **required_xp <= **current_xp && **current_level < **max_level {
        **current_level += 1;
        **current_xp -= **required_xp;
        **required_xp += XP_SCALING_FACTOR;
        game_state.set(GameState::LevelUp);
        sound_event.write(PlaySoundEffectEvent(SoundEffectKind::Player(
            PlayerSound::Levelup,
        )));
    }
}

pub fn handle_player_death(
    player_query: Query<&Health, With<Player>>,
    mut game_state: ResMut<NextState<GameState>>,
) {
    let player_health = player_query.single().expect("Err");
    if **player_health == 0 {
        game_state.set(GameState::Loss);
    }
}