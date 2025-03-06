use bevy::prelude::*;
use test_game::{PLAYER_Z, PROJECTILES_Z};

use std::time::Duration;

use crate::mechanics::cooldown::{Cooldown, LifeTime};
use crate::mechanics::damage::Damage;
use crate::mechanics::damage::{damaging, Radius};
use crate::mechanics::damage::{Health, HitList};
use crate::mechanics::projectiles::{projectile, ShouldRotate};
use crate::{cleanup, AppState, CursorTranslation, GameState, MovementSpeed, MyGameCamera};
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

fn player(layout: Handle<TextureAtlasLayout>, texture: Handle<Image>) -> impl Bundle {
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
            RequiredXP(100.0),
            CurrentLevel(1),
            MaxLevel(100),
            XpPickUpRadius(100.0 * SCALE),
            Health(2),
        ),
        MaxHealth(2),
        Transform::from_xyz(0.0, 0.0, PLAYER_Z),
        Radius(Vec2::new(PLAYER_HEIGHT, PLAYER_WIDTH).length() / 2.),
        Sprite {
            image: texture,
            texture_atlas: Some(TextureAtlas {
                layout: layout,
                index: 0,
            }),
            ..default()
        },
    )
}

const PLAYER_HEIGHT: f32 = 20.;
const PLAYER_WIDTH: f32 = 16.;

pub fn spawn_player_hero(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    let texture_handle: Handle<Image> = asset_server.load("characters/viking.png");
    let layout = TextureAtlasLayout::from_grid(
        UVec2::new(PLAYER_WIDTH as u32, PLAYER_HEIGHT as u32),
        4,
        1,
        None,
        None,
    );
    let texture_atlas_layout = texture_atlas_layouts.add(layout);
    commands.spawn(player(texture_atlas_layout, texture_handle));
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
    asset_server: Res<AssetServer>,
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
                &asset_server,
                attack_direction,
                projectile_speed,
                damage,
                range,
            );
        }
    } else {
        attack_cooldown.wait();
    }
}

fn player_shoot(
    commands: &mut Commands,
    player_position: Vec2,
    asset_server: &Res<AssetServer>,
    dir: &Heading,
    projectile_speed: ProjectileSpeed,
    damage: PlayerDamage,
    range: Range,
) {
    let diff = player_position - **dir;
    commands.spawn((
        projectile(
            *dir,
            MovementSpeed(*projectile_speed),
            range,
            ShouldRotate(true),
        ),
        damaging(Damage(*damage), Radius(10.)),
        (
            Sprite {
                image: asset_server.load("skills/axe.png"),
                ..default()
            },
            Transform::from_xyz(player_position.x, player_position.y, PROJECTILES_Z).with_rotation(
                Quat::from_axis_angle(Vec3::new(0., 0., 1.0), diff.y.atan2(diff.x)),
            ),
        ),
        HitList::default(),
    ));
    commands.spawn((
        AudioPlayer::<AudioSource>(asset_server.load("sounds/effects/pew-laser.wav")),
        PlaybackSettings::ONCE,
        LifeTime(Duration::from_secs(1)),
    ));
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
) {
    let (mut current_xp, mut required_xp, mut current_level, max_level) = query.single_mut();
    if **required_xp <= **current_xp && **current_level < **max_level {
        **current_level += 1;
        **current_xp -= **required_xp;
        **required_xp += XP_SCALING_FACTOR;
        game_state.set(GameState::LevelUp);
    }
}

pub fn handle_player_death(
    player_query: Query<&Health, With<Player>>,
    mut app_state: ResMut<NextState<AppState>>,
    mut game_state: ResMut<NextState<GameState>>,
) {
    let player_health = player_query.single();
    if **player_health == 0 {
        app_state.set(AppState::MainMenu);
        game_state.set(GameState::Paused);
    }
}
