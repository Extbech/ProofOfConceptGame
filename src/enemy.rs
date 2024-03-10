use crate::Direction;
use crate::Player;
use bevy::prelude::*;
use rand::prelude::*;

#[derive(Component)]
pub struct Enemy;

#[derive(Bundle)]
pub struct EnemyBundle {
    pub marker: Enemy,
    pub dir: Direction,
    pub sprite: SpriteBundle,
}

impl EnemyBundle {
    fn new(dir: Direction, sprite: SpriteBundle) -> Self {
        EnemyBundle {
            marker: Enemy,
            dir,
            sprite,
        }
    }
}

pub fn update_enemies(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    query: Query<&mut Transform, With<Player>>,
) {
    let enemy_sprite: Handle<Image> = asset_server.load("models/enemy.png");
    let player = query.single().translation;
    let enemy_position = generate_random_starting_position();
    let dir: Direction = Direction::new(player.x, player.y);
    commands.spawn({
        EnemyBundle::new(
            dir,
            SpriteBundle {
                transform: Transform::from_xyz(enemy_position.0, enemy_position.1, 1.),
                texture: enemy_sprite,
                ..default()
            },
        );
    });
}

fn generate_random_starting_position() -> (f32, f32) {
    let angle: f32 = rand::thread_rng().gen_range(0.0..360.0);
    let r = 2100.0;
    let x = r * angle.sin();
    let y = r * angle.cos();
    println!("X: {} Y: {}", x, y);
    (x, y)
}
