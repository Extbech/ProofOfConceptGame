use bevy::prelude::*;

use std::time::Duration;

use crate::cooldown::{Cooldown, LifeTime};
use crate::damage::Radius;
use crate::damage::{Damage, DamagingBundle};
use crate::damage::{Health, HitList};
use crate::projectiles::{ProjectileBundle, ShouldRotate};
use crate::Heading;
use crate::{cleanup, AppState, CursorTranslation, GameState, MovementSpeed, MyGameCamera};

pub const XP_SCALING_FACTOR: f32 = 1.5;
#[derive(Component)]
pub struct Player;

#[derive(Component, Deref, Clone, Copy)]
pub struct ProjectileSpeed(f32);

#[derive(Component, Deref, DerefMut, Clone, Copy)]
pub struct MaxAttackCooldown(Duration);

#[derive(Component, Deref, DerefMut, Clone)]
pub struct AttackCooldown(Cooldown);

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
pub struct PickUpRadius(f32);

#[derive(Component, Deref, DerefMut)]
pub struct Vulnerability(Cooldown);

#[derive(Component, Deref, DerefMut, Clone, Copy)]
pub struct MaxSpeed(f32);

#[derive(Component, Deref, DerefMut)]
pub struct AttackDirection(Heading);

#[derive(Component, Deref, DerefMut, Clone, Copy)]
pub struct PlayerDamage(u32);

#[derive(Component, Deref, DerefMut, Clone, Copy)]
pub struct MaxHealth(pub u32);

#[derive(Bundle)]
pub struct ProjectileStatBundle {
    damage: PlayerDamage,
    projectile_speed: ProjectileSpeed,
    range: Range,
}

#[derive(Bundle)]
pub struct PlayerStatBundle {
    cleanup: cleanup::ExitGame,
    marker: Player,
    vulnerability: Vulnerability,
    dir: Heading,
    attack_direction: AttackDirection,
    speed: MovementSpeed,
    attack_cooldown: AttackCooldown,
    projectile_stats: ProjectileStatBundle,
    current_xp: CurrentXP,
    required_xp: RequiredXP,
    current_level: CurrentLevel,
    max_level: MaxLevel,
    max_attack_cooldown: MaxAttackCooldown,
    pick_up_radius: PickUpRadius,
    health: Health,
    max_health: MaxHealth,
}

impl PlayerStatBundle {
    pub fn new() -> Self {
        PlayerStatBundle {
            cleanup: cleanup::ExitGame,
            marker: Player,
            vulnerability: Vulnerability(Cooldown::waiting()),
            dir: default(),
            attack_direction: AttackDirection(Heading::new(Vec2::new(0., 1.))),
            speed: MovementSpeed(300.),
            attack_cooldown: AttackCooldown(default()),
            max_attack_cooldown: MaxAttackCooldown(Duration::from_secs_f32(0.5)),
            projectile_stats: ProjectileStatBundle {
                damage: PlayerDamage(1),
                projectile_speed: ProjectileSpeed(450.),
                range: Range(500.),
            },
            current_xp: CurrentXP(0.0),
            required_xp: RequiredXP(100.0),
            current_level: CurrentLevel(1),
            max_level: MaxLevel(10),
            pick_up_radius: PickUpRadius(100.0),
            health: Health(2),
            max_health: MaxHealth(2),
        }
    }
}

pub fn spawn_player_hero(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    let texture_handle: Handle<Image> = asset_server.load("models/cowboy_hero.png");
    let layout = TextureAtlasLayout::from_grid(Vec2::new(45.0, 45.0), 8, 4, None, None);
    let texture_atlas_layout = texture_atlas_layouts.add(layout);
    commands.spawn((
        PlayerStatBundle::new(),
        SpriteSheetBundle {
            texture: texture_handle,
            atlas: TextureAtlas {
                layout: texture_atlas_layout,
                index: 0,
            },
            transform: Transform::from_xyz(0.0, 0.0, 3.0),
            sprite: Sprite {
                custom_size: Some(Vec2::new(70.0, 70.0)),
                ..Default::default()
            },
            ..default()
        },
        Radius(20.),
    ));
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
    mut player: Query<(&mut Transform, &mut Heading, &mut Sprite, &AttackDirection), With<Player>>,
) {
    let (mut player_trans, mut player_dir, mut player_sprite, attack_dir) = player.single_mut();
    let player_position = &mut player_trans.translation;
    let keyboard_dir_x = if keys.pressed(KeyCode::KeyD) { 1. } else { 0. }
        - if keys.pressed(KeyCode::KeyA) { 1. } else { 0. };
    let keyboard_dir_y = if keys.pressed(KeyCode::KeyW) { 1. } else { 0. }
        - if keys.pressed(KeyCode::KeyS) { 1. } else { 0. };
    const BOUND: f32 = 1900.;
    (player_position.x, player_position.y) = player_position
        .xy()
        .clamp(-Vec2::splat(BOUND), Vec2::splat(BOUND))
        .into();
    *player_dir = Heading::new(Vec2::new(keyboard_dir_x, keyboard_dir_y));
    player_sprite.flip_x = attack_dir.x <= 0.;
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

/// Bullet Asset source: https://opengameart.org/content/assets-free-laser-bullets-pack-2020
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
        ProjectileBundle::new(
            *dir,
            MovementSpeed(*projectile_speed),
            range,
            ShouldRotate(true),
        ),
        DamagingBundle {
            damage: Damage(*damage),
            radius: Radius(20.),
        },
        SpriteBundle {
            transform: Transform::from_xyz(player_position.x, player_position.y, 1.).with_rotation(
                Quat::from_axis_angle(Vec3::new(0., 0., 1.), diff.y.atan2(diff.x)),
            ),
            texture: asset_server.load("models/bullet.png"),
            sprite: Sprite {
                custom_size: Some(Vec2::new(50., 50.)),
                ..Default::default()
            },
            ..default()
        },
        HitList(vec![]),
    ));
    commands
        .spawn(AudioBundle {
            source: asset_server.load("sounds/effects/pew-laser.wav"),
            settings: PlaybackSettings::ONCE,
        })
        .insert(LifeTime(Duration::from_secs(1)));
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
        **required_xp *= XP_SCALING_FACTOR;
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
