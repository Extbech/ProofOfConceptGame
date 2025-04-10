mod fire_volley;

use std::{
    f32::consts::{PI, TAU},
    time::Duration,
};

use bevy::prelude::*;
use fire_volley::{spawn_fire_volley, spawn_fire_volley_spell};
use rand::prelude::*;
use test_game::ENEMY_Z;

use crate::{
    characters::player::Player,
    cleanup,
    mechanics::{
        cooldown::InGameTime,
        damage::{Circle, Cone, DealDamageHitbox, Health, TakeDamageHitbox},
    },
    sprites::{Character, SpriteKind, WIZARD_HEIGHT, WIZARD_WIDTH},
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
    )
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
        commands
            .spawn(wizard_bundle(enemy_position.x, enemy_position.y))
            .with_children(|spells| {
                spawn_fire_volley_spell(spells);
            });
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

pub(super) struct BossPlugin;

impl Plugin for BossPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (spawn_boss, spawn_fire_volley, check_for_victory).run_if(in_state(GameState::Running)),
        );
    }
}
