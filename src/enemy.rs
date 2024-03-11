use crate::{
    projectiles::{Projectile, ProjectileBundle},
    Player,
};
use bevy::prelude::*;
use rand::prelude::*;

#[derive(Component)]
pub struct Enemy;

#[derive(Bundle)]
pub struct EnemyBundle {
    pub marker: Enemy,
    pub sprite: SpriteBundle,
}

impl EnemyBundle {
    pub fn new(sprite: SpriteBundle) -> Self {
        EnemyBundle {
            marker: Enemy,
            sprite,
        }
    }
}

pub fn update_enemies(
    commands: Commands,
    asset_server: Res<AssetServer>,
    query: Query<&mut Transform, With<Player>>,
    time: Res<Time>,
) {
    todo!()
}

pub fn generate_random_starting_position(pos: Vec2) -> Vec2 {
    // x: 100, y: 200
    let angle: f32 = rand::thread_rng().gen_range(0.0..360.0);
    let r = 1000.0;
    let x = r * angle.sin();
    let y = r * angle.cos();
    Vec2::new(pos.x + x, pos.y + y)
}

pub fn spawn_enemies(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    query: Query<&mut Transform, With<Player>>,
    _time: Res<Time>,
) {
    let enemy_sprite: Handle<Image> = asset_server.load("models/enemy.png");
    let player = query.single().translation;
    let enemy_position = generate_random_starting_position(player.xy());
    commands.spawn(EnemyBundle::new(SpriteBundle {
        transform: Transform::from_xyz(enemy_position.x, enemy_position.y, 1.),
        texture: enemy_sprite,
        sprite: Sprite {
            custom_size: Some(Vec2::new(75., 100.)),
            ..Default::default()
        },
        ..default()
    }));
}

pub fn handle_enemy_collision(
    mut commands: Commands,
    projectiles_query: Query<&Transform, With<Projectile>>,
    mut enemy_query: Query<(&Transform, Entity), With<Enemy>>,
) {
    for projectile_transform in projectiles_query.iter() {
        for (enemy_transform, entity) in enemy_query.iter_mut() {
            if is_collision(
                projectile_transform.translation,
                enemy_transform.translation,
                50.,
                10.,
            ) {
                commands.entity(entity).despawn();
            }
        }
    }
}

fn is_collision(obj1: Vec3, obj2: Vec3, obj1_radius: f32, obj2_radius: f32) -> bool {
    let diff = (obj1.xy() - obj2.xy()).length();
    if diff < obj1_radius + obj2_radius {
        return true;
    }
    false
}
