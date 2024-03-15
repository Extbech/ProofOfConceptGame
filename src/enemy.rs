use crate::cooldown::Cooldown;
use crate::player::Damage;
use crate::{projectiles::Projectile, Player};
use crate::{GameRng, MovementSpeed};
use bevy::prelude::*;
use rand::prelude::*;

#[derive(Resource, Deref, DerefMut)]
pub struct SpawnRate(f32);

#[derive(Resource, Deref, DerefMut)]
pub struct SpawnCooldown(pub Cooldown);

pub const DEFAULT_SPAWN_RATE: SpawnRate = SpawnRate(1.);

#[derive(Component, Deref, DerefMut)]
pub struct Health(f32);

#[derive(Component)]
pub struct Enemy;

#[derive(Bundle)]
pub struct EnemyBundle {
    marker: Enemy,
    health: Health,
    speed: MovementSpeed,
    sprite: SpriteBundle,
}

impl EnemyBundle {
    pub fn new(sprite: SpriteBundle) -> Self {
        EnemyBundle {
            marker: Enemy,
            health: Health(2.),
            speed: MovementSpeed(100.),
            sprite,
        }
    }
}

pub fn update_enemies(
    q_pl: Query<&Transform, With<Player>>,
    mut q_enmy: Query<(&mut Transform, &MovementSpeed), (With<Enemy>, Without<Player>)>,
    time: Res<Time>,
) {
    let player_position = q_pl.single().translation.xy();
    for (mut enmy_trans, &speed) in &mut q_enmy {
        let enemy_pos = enmy_trans.translation.xy();
        enmy_trans.translation = (enemy_pos
            - (enemy_pos - player_position).normalize_or_zero() * time.delta_seconds() * *speed)
            .extend(enmy_trans.translation.z);
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
) {
    for _ in 0..spawncooldown.reset(**spawnrate) {
        let enemy_sprite: Handle<Image> = asset_server.load("models/spiky_blob_enemy.png");
        let player = query.single().translation;
        let enemy_position = generate_random_starting_position(player.xy(), &mut rng);
        commands.spawn(EnemyBundle::new(SpriteBundle {
            transform: Transform::from_xyz(enemy_position.x, enemy_position.y, 1.),
            texture: enemy_sprite,
            sprite: Sprite {
                custom_size: Some(Vec2::new(70., 70.)),
                ..Default::default()
            },
            ..default()
        }));
    }
}

pub fn handle_enemy_collision(
    projectiles_query: Query<(&Transform, &Damage), With<Projectile>>,
    mut enemy_query: Query<(&Transform, &mut Health), With<Enemy>>,
) {
    for (projectile_transform, damage) in projectiles_query.iter() {
        for (enemy_transform, mut health) in enemy_query.iter_mut() {
            if is_collision(
                projectile_transform.translation,
                enemy_transform.translation,
                50.,
                10.,
            ) {
                **health -= **damage;
            }
        }
    }
}

pub fn is_collision(obj1: Vec3, obj2: Vec3, obj1_radius: f32, obj2_radius: f32) -> bool {
    let diff = (obj1.xy() - obj2.xy()).length();
    if diff < obj1_radius + obj2_radius {
        return true;
    }
    false
}
