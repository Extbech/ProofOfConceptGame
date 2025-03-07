use std::time::Duration;

use bevy::prelude::*;
use test_game::ENEMY_Z;
use rand::prelude::*;

use crate::{characters::player::Player, cleanup, mechanics::{cooldown::InGameTime, damage::{Circle, DealDamageHitBox, Health}}, sprites::{Character, SpriteKind, WIZARD_HEIGHT, WIZARD_WIDTH}, GameRng, Heading, MovementSpeed};

use super::enemy::Enemy;

pub fn wizard_bundle(x: f32, y: f32) -> impl Bundle {
    (
        cleanup::ExitGame,
        Enemy,
        Health(10),
        MovementSpeed(100.),
        Heading::default(),
        DealDamageHitBox::Circle(Circle{radius: Vec2::new(WIZARD_HEIGHT as f32, WIZARD_WIDTH as f32).length() / 2.}),
        Transform::from_xyz(x, y, ENEMY_Z),
        SpriteKind::Character(Character::Wizard),
    )
}

pub fn generate_random_starting_position(pos: Vec2, rng: &mut GameRng) -> Vec2 {
    let angle: f32 = rng.gen_range(0.0..(2. * std::f32::consts::PI));
    let r: f32 = rng.gen_range(500.0..1000.0);
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
    if !boss_spawned.0 && in_game_time.0 >= Duration::from_secs(60*0) {
        boss_spawned.0 = true;
        let player = query.single().translation;
        let enemy_position = generate_random_starting_position(player.xy(), &mut rng);
        commands.spawn(wizard_bundle(
            enemy_position.x,
            enemy_position.y,
        ));
    }
}
