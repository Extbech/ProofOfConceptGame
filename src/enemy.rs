use crate::Direction;
use crate::Player;
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
    fn new(sprite: SpriteBundle) -> Self {
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
    spawn_enemies(commands, asset_server, query, time);
}

fn generate_random_starting_position(pos: Vec2) -> Vec2 {
    // x: 100, y: 200
    let angle: f32 = rand::thread_rng().gen_range(0.0..360.0);
    let r = 100.0;
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
    let enemy_sprite: Handle<Image> = asset_server.load("models/hero.png");
    let player = query.single().translation;
    let enemy_position = generate_random_starting_position(player.xy());
    commands.spawn(
        EnemyBundle::new(SpriteBundle {
            transform: Transform::from_xyz(enemy_position.x, enemy_position.y, 1.)
                .with_scale(Vec3::new(1., 1., 1.)),
            texture: enemy_sprite,
            ..default()
        })
    );
}
