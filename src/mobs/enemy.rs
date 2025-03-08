use crate::mechanics::cooldown::{Cooldown, InGameTime};
use crate::mechanics::damage::{Circle, DealDamageHitbox, Health, TakeDamageHitbox};
use crate::sprites::{Character, SpriteKind, ENEMY_HEIGHT, ENEMY_WIDTH};
use crate::tools::rng::GameRng;
use crate::{cleanup, MovementSpeed};
use crate::{Heading, Player};
use bevy::prelude::*;
use std::time::Duration;
use test_game::ENEMY_Z;

#[derive(Resource, Deref, DerefMut)]
pub struct SpawnRate(pub Duration);

#[derive(Resource, Deref, DerefMut)]
pub struct SpawnCooldown(pub Cooldown);

#[derive(Component)]
pub struct Enemy;

pub fn jotun_bundle(health: u32, x: f32, y: f32) -> impl Bundle {
    let radius = Vec2::new(ENEMY_HEIGHT as f32, ENEMY_WIDTH as f32).length() / 2.;
    (
        cleanup::ExitGame,
        Enemy,
        Health(health),
        MovementSpeed(100.),
        Heading::default(),
        DealDamageHitbox::Circle(Circle { radius }),
        TakeDamageHitbox(Circle { radius }),
        Transform::from_xyz(x, y, ENEMY_Z),
        SpriteKind::Character(Character::Jotun),
    )
}

pub fn update_enemies(
    q_pl: Query<&Transform, With<Player>>,
    mut q_enmy: Query<(&Transform, &mut Heading, &mut Sprite), (With<Enemy>, Without<Player>)>,
) {
    let player_position = q_pl.single().translation.xy();
    for (enmy_trans, mut heading, mut sprite) in &mut q_enmy {
        let enemy_pos = enmy_trans.translation.xy();
        *heading = Heading::new(-(enemy_pos - player_position));
        if let Some(atlas) = &mut sprite.texture_atlas {
            atlas.index = [
                Vec2::new(0., -1.),
                Vec2::new(0., 1.),
                Vec2::new(1., 0.),
                Vec2::new(-1., 0.),
            ]
            .into_iter()
            .enumerate()
            .max_by(|(_, v1), (_, v2)| v1.dot(**heading).partial_cmp(&v2.dot(**heading)).unwrap())
            .unwrap()
            .0;
        }
    }
}

pub fn generate_random_starting_position(pos: Vec2, rng: &mut GameRng) -> Vec2 {
    pos + rng.rand_vec(500., 1000.)
}

pub fn spawn_enemies(
    mut commands: Commands,
    query: Query<&Transform, With<Player>>,
    _time: Res<Time>,
    mut spawncooldown: ResMut<SpawnCooldown>,
    spawnrate: Res<SpawnRate>,
    mut rng: ResMut<GameRng>,
    in_game_time: Res<InGameTime>,
) {
    return;
    for _ in 0..spawncooldown.reset(**spawnrate) {
        let player = query.single().translation;
        let enemy_position = generate_random_starting_position(player.xy(), &mut rng);
        commands.spawn(jotun_bundle(
            get_enemy_health(&in_game_time),
            enemy_position.x,
            enemy_position.y,
        ));
    }
}

pub fn get_enemy_health(in_game_time: &Res<InGameTime>) -> u32 {
    10 + (in_game_time.time().as_secs_f32() / 30.0).floor() as u32
}
