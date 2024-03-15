mod enemy;
mod loot;
mod map;
mod player;
mod projectiles;
mod cooldown;

use std::time::Duration;

use bevy::{input::ButtonInput, prelude::*, window::PrimaryWindow};
use bevy_ecs_tilemap::TilemapPlugin;
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use enemy::{
    handle_enemy_collision, spawn_enemies, update_enemies, SpawnCooldown, SpawnRate
};
use player::{
    spawn_player_hero, AttackCooldown, Damage, MaxAttackCooldown, Player, ProjectileSpeed, Range
};
use projectiles::{projectile_movement, ProjectileBundle, RemDistance};
use rand::{rngs::SmallRng, SeedableRng};

use loot::{animate_sprite, check_for_dead_enemies};
fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .add_plugins(TilemapPlugin)
        .add_plugins(WorldInspectorPlugin::new())
        .add_systems(Startup, (setup, map::setup_map, spawn_player_hero))
        .add_systems(
            Update,
            (
                keyboard_input,
                sync_player_and_camera_pos,
                projectile_movement,
                cooldown::tick_cooldown::<AttackCooldown>,
                cooldown::tick_cooldown_res::<SpawnCooldown>,
                spawn_enemies,
                handle_enemy_collision,
                update_enemies,
                check_for_dead_enemies,
                animate_sprite,
            ),
        )
        .run();
}

#[derive(Component)]
struct MyGameCamera;

/// normalized vector pointing in some direction. Is always nonzero
#[derive(Component, Clone, Copy, Deref, DerefMut)]
pub struct Direction {
    v: Vec2,
}

impl Direction {
    fn try_new(x: f32, y: f32) -> Option<Self> {
        Some(Direction {
            v: Vec2::new(x, y).try_normalize()?,
        })
    }
}

impl Default for Direction {
    fn default() -> Self {
        Direction {
            v: Vec2::new(0., 1.),
        }
    }
}

#[derive(Resource, Deref, DerefMut)]
struct GameRng(SmallRng);

#[derive(Component, Deref, DerefMut, Clone, Copy)]
struct MovementSpeed(f32);

fn setup(mut commands: Commands, window: Query<&mut Window, With<PrimaryWindow>>) {
    commands.spawn((
        {
            let mut cam = Camera2dBundle::default();
            cam.transform = cam.transform.with_scale(Vec3::new(1., 1., 1.));
            cam
        },
        MyGameCamera,
    ));
    commands.insert_resource(SpawnRate(Duration::from_secs_f32(1.)));
    commands.insert_resource(GameRng(SmallRng::from_entropy()));
    commands.insert_resource(SpawnCooldown(default()));
    app_window_config(window);
}

fn sync_player_and_camera_pos(
    player: Query<&Transform, With<Player>>,
    mut cam: Query<&mut Transform, (With<MyGameCamera>, Without<Player>)>,
) {
    let player = player.single();
    let mut cam = cam.single_mut();
    cam.translation.x = player.translation.x;
    cam.translation.y = player.translation.y;
}

fn keyboard_input(
    time: Res<Time>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    keys: Res<ButtonInput<KeyCode>>,
    mut player: Query<
        (
            &mut Transform,
            &mut Direction,
            &MovementSpeed,
            &ProjectileSpeed,
            &mut AttackCooldown,
            &MaxAttackCooldown,
            &Damage,
            &Range,
            &mut Sprite,
        ),
        With<Player>,
    >,
) {
    let (
        mut player_trans,
        mut player_dir,
        &player_speed,
        &projectile_speed,
        mut attack_cooldown,
        &max_attack_cooldown,
        &damage,
        &range,
        mut player_sprite,
    ) = player.single_mut();
    let player_position = &mut player_trans.translation;
    let keyboard_dir_x = if keys.pressed(KeyCode::KeyA) { 0. } else { 1. }
        - if keys.pressed(KeyCode::KeyD) { 0. } else { 1. };
    let keyboard_dir_y = if keys.pressed(KeyCode::KeyS) { 0. } else { 1. }
        - if keys.pressed(KeyCode::KeyW) { 0. } else { 1. };

    let keyboard_dir = Vec2::new(keyboard_dir_x, keyboard_dir_y).normalize_or_zero();
    const BOUND: f32 = 1900.;
    (player_position.x, player_position.y) = (player_position.xy()
        + *player_speed * time.delta_seconds() * keyboard_dir)
        .clamp(-Vec2::splat(BOUND), Vec2::splat(BOUND))
        .into();
    *player_dir = Direction::try_new(keyboard_dir.x, keyboard_dir.y).unwrap_or(*player_dir);
    if keys.pressed(KeyCode::KeyJ) {
        for _ in 0..(attack_cooldown.reset(*max_attack_cooldown)) {
            commands.spawn(ProjectileBundle::new(
                SpriteBundle {
                    transform: Transform::from_xyz(player_position.x, player_position.y, 1.),
                    texture: asset_server.load("models/bullet.png"),
                    sprite: Sprite {
                        custom_size: Some(Vec2::new(25., 25.)),
                        ..Default::default()
                    },
                    ..default()
                },
                *player_dir,
                MovementSpeed(*projectile_speed),
                damage,
                RemDistance(*range),
            ));
            commands.spawn(AudioBundle {
                source: asset_server.load("sounds/effects/pew-laser.wav"),
                settings: PlaybackSettings::ONCE,
            });
        }
    } else {
        attack_cooldown.wait();
    }
    if keyboard_dir_x == 1. {
        player_sprite.flip_x = false;
    } else if keyboard_dir_x == -1. {
        player_sprite.flip_x = true;
    }
}

fn app_window_config(mut window: Query<&mut Window, With<PrimaryWindow>>) {
    let mut curr_window = window.single_mut();
    curr_window.title = "Vampire Survivors Copy Cat".to_string();
}
