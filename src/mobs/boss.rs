use std::{f32::consts::{PI, TAU}, time::Duration};

use bevy::prelude::*;
use rand::prelude::*;
use test_game::ENEMY_Z;

use crate::{
    characters::player::{AttackCooldown, MaxAttackCooldown, Player},
    cleanup,
    mechanics::{
        cooldown::{Cooldown, InGameTime, LifeTime},
        damage::{damaging, BaseDamage, Circle, Cone, DealDamageHitbox, Health, TakeDamageHitbox},
    },
    sprites::{Character, Skill, SpriteKind, WIZARD_HEIGHT, WIZARD_WIDTH},
    GameRng, GameState, Heading, MovementSpeed,
};

use super::enemy::Enemy;

#[derive(Component)]
pub struct EndGameIfDead;

pub fn wizard_bundle(x: f32, y: f32) -> impl Bundle {
    (
        cleanup::ExitGame,
        Enemy,
        Health(200),
        MovementSpeed(0.),
        Heading::default(),
        TakeDamageHitbox(Circle {
            radius: Vec2::new(WIZARD_HEIGHT as f32, WIZARD_WIDTH as f32).length() / 2.,
        }),
        DealDamageHitbox::Cone(Cone {
            mid_angle: Vec2::new(50., 50.),
            angular_width: (1. / 8.) * PI,
        }),
        Transform::from_xyz(x, y, ENEMY_Z),
        SpriteKind::Character(Character::Wizard),
        EndGameIfDead,
        FireVolleyCount(4),
        AttackCooldown(default()),
        MaxAttackCooldown(Duration::from_secs_f32(3.0)),
    )
}

#[derive(Component, Deref, DerefMut, Clone, Copy)]
pub (super) struct FireVolleyCount(pub (super) u32);

pub(super) fn fire_volley_bundle(x: f32, y: f32) -> impl Bundle {
    (
        LifeTime::from_secs_f32(2.),
        damaging(
            BaseDamage(10),
            DealDamageHitbox::Circle(Circle { radius: 20. }),
        ),
        SpriteKind::Skill(Skill::OrbJutsu),
        Transform::from_translation(Vec3::new(x, y, ENEMY_Z)),
    )
}

pub(super) fn spawn_fire_volley(mut commands: Commands, mut boss_query: Query<(&Health, &Transform, &FireVolleyCount, &mut AttackCooldown, &MaxAttackCooldown), With<EndGameIfDead>>) {
    for (health, transform, fv_count, mut fv_cooldown, fv_max_cooldown) in &mut boss_query {
        if **health > 0 && fv_cooldown.is_ready(**fv_max_cooldown) {
            for _ in 0..(fv_cooldown.reset_and_wait(**fv_max_cooldown)) {
                let start_angle: f32 = rand::thread_rng().gen_range(0.0..TAU);
                for n in 1..=**fv_count {
                    let angle = start_angle + (n as f32 * (TAU / **fv_count as f32));
                    let x = 20.0 * angle.sin() + transform.translation.x;
                    let y = 20.0 * angle.cos() + transform.translation.y;
                    commands.spawn(fire_volley_bundle(x, y));
                }
            }
        }
    }
}

/// TODO: This should be a system that spawns the boss at a random position around the player.
fn generate_random_starting_position(pos: Vec2, rng: &mut GameRng) -> Vec2 {
    let angle: f32 = rng.gen_range(0.0..TAU);
    let r: f32 = rng.gen_range(50.0..100.0);
    let x = r * angle.sin();
    let y = r * angle.cos();
    Vec2::new(pos.x + x, pos.y + y)
}

#[derive(Resource, Default)]
pub struct BossSpawned(bool);

pub fn spawn_boss(
    mut commands: Commands,
    query: Query<&Transform, With<Player>>,
    _time: Res<Time>,
    mut boss_spawned: ResMut<BossSpawned>,
    mut rng: ResMut<GameRng>,
    in_game_time: Res<InGameTime>,
) {
    if !boss_spawned.0 && in_game_time.0 >= Duration::from_secs(60 * 0) {
        boss_spawned.0 = true;
        let player = query.single().translation;
        let enemy_position = generate_random_starting_position(player.xy(), &mut rng);
        commands.spawn(wizard_bundle(enemy_position.x, enemy_position.y));
    }
}

pub fn check_for_victory(
    mut game_state: ResMut<NextState<GameState>>,
    query: Query<&Health, With<EndGameIfDead>>,
) {
    if let Some(health) = query.iter().next() {
        if **health == 0 {
            game_state.set(GameState::Win);
        }
    }
}

pub fn reset_boss_spawn(mut res: ResMut<BossSpawned>) {
    res.0 = false;
}
