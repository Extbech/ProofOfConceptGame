use crate::cooldown::{Cooldown, InGameTime};
use crate::damage::{Health, Radius};
use crate::{cleanup, GameRng, MovementSpeed};
use crate::{Heading, Player};
use bevy::prelude::*;
use rand::prelude::*;
use std::time::Duration;
use test_game::ENEMY_Z;

#[derive(Resource, Deref, DerefMut)]
pub struct SpawnRate(pub Duration);

#[derive(Resource, Deref, DerefMut)]
pub struct SpawnCooldown(pub Cooldown);

#[derive(Component)]
pub struct Enemy;

#[derive(Bundle)]
pub struct EnemyBundle {
    cleanup: cleanup::ExitGame,
    marker: Enemy,
    health: Health,
    speed: MovementSpeed,
    heading: Heading,
    texture: Sprite,
    radius: Radius,
}

const ENEMY_HEIGHT: u32 = 20;
const ENEMY_WIDTH: u32 = 22;

impl EnemyBundle {
    pub fn new(texture: Sprite, health: Health) -> Self {
        EnemyBundle {
            cleanup: cleanup::ExitGame,
            marker: Enemy,
            health,
            speed: MovementSpeed(100.),
            heading: Heading::new(Vec2::new(0., 0.)),
            radius: Radius(Vec2::new(ENEMY_HEIGHT as f32, ENEMY_WIDTH as f32).length() / 2.),
            texture,
        }
    }
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
    // x: 100, y: 200
    let angle: f32 = rng.gen_range(0.0..(2. * std::f32::consts::PI));
    let r = 1000.0;
    let x = r * angle.sin();
    let y = r * angle.cos();
    Vec2::new(pos.x + x, pos.y + y)
}

pub fn spawn_enemies(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    query: Query<&Transform, With<Player>>,
    _time: Res<Time>,
    mut spawncooldown: ResMut<SpawnCooldown>,
    spawnrate: Res<SpawnRate>,
    mut rng: ResMut<GameRng>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
    in_game_time: Res<InGameTime>,
) {
    for _ in 0..spawncooldown.reset(**spawnrate) {
        let texture_handle: Handle<Image> = asset_server.load("characters/jotun.png");
        let layout =
            TextureAtlasLayout::from_grid(UVec2::new(ENEMY_WIDTH, ENEMY_HEIGHT), 4, 1, None, None);
        let texture_atlas_layout = texture_atlas_layouts.add(layout);
        let player = query.single().translation;
        let enemy_position = generate_random_starting_position(player.xy(), &mut rng);
        commands.spawn((
            EnemyBundle::new(
                Sprite {
                    image: texture_handle,
                    texture_atlas: Some(TextureAtlas {
                        layout: texture_atlas_layout,
                        index: 0,
                    }),
                    ..default()
                },
                get_enemy_health(&in_game_time),
            ),
            Transform::from_xyz(enemy_position.x, enemy_position.y, ENEMY_Z),
        ));
    }
}

pub fn get_enemy_health(in_game_time: &Res<InGameTime>) -> Health {
    let health = 10 + (in_game_time.time().as_secs_f32() / 30.0).floor() as u32;
    return Health(health);
}
