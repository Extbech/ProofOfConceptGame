use crate::cooldown::Cooldown;
use crate::damage::{Health, Radius};
use crate::Player;
use crate::{cleanup, GameRng, MovementSpeed};
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
    texture: SpriteSheetBundle,
    radius: Radius,
}

impl EnemyBundle {
    pub fn new(texture: SpriteSheetBundle) -> Self {
        EnemyBundle {
            cleanup: cleanup::ExitGame,
            marker: Enemy,
            health: Health(2),
            speed: MovementSpeed(100.),
            texture,
            radius: Radius(50.),
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
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    for _ in 0..spawncooldown.reset(**spawnrate) {
        let texture_handle: Handle<Image> = asset_server.load("jotun.png");
        let layout = TextureAtlasLayout::from_grid(Vec2::new(22.0, 20.0), 4, 1, None, None);
        let texture_atlas_layout = texture_atlas_layouts.add(layout);
        let player = query.single().translation;
        let enemy_position = generate_random_starting_position(player.xy(), &mut rng);
        commands.spawn(EnemyBundle::new(SpriteSheetBundle {
            transform: Transform::from_xyz(enemy_position.x, enemy_position.y, ENEMY_Z),
            texture: texture_handle,
            atlas: TextureAtlas {
                layout: texture_atlas_layout,
                index: 0,
            },
            ..default()
        }));
    }
}
