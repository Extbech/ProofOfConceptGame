use std::time::Duration;
use crate::{Direction, Heading, Health};
use crate::cooldown::Cooldown;
use crate::player::{Damage, Range, Vulnerability};
use crate::projectiles::{HitList, ProjectileBundle};
use crate::{projectiles::Projectile, Player};
use crate::{cleanup, GameRng, MovementSpeed};
use bevy::prelude::*;
use rand::prelude::*;

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
    sprite: SpriteBundle,
}

impl EnemyBundle {
    pub fn new(sprite: SpriteBundle) -> Self {
        EnemyBundle {
            cleanup: cleanup::ExitGame,
            marker: Enemy,
            health: Health(2),
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
                ..default()
            },
            ..default()
        }));
    }
}

pub fn handle_enemy_damage_from_projectiles(
    mut projectiles_query: Query<(&Transform, &Damage, &mut HitList), With<Projectile>>,
    mut enemy_query: Query<(&Transform, &mut Health, Entity), With<Enemy>>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    for (projectile_transform, damage, mut hitlist) in projectiles_query.iter_mut() {
        for (enemy_transform, mut health, ent) in enemy_query.iter_mut() {
            if !hitlist.contains(&ent) {
                if is_collision(
                    projectile_transform.translation.xy(),
                    enemy_transform.translation.xy(),
                    50.,
                    10.,
                ) {
                    **health = health.saturating_sub(**damage);
                    commands.spawn(ProjectileBundle::new(
                        Heading::new(Vec2::new(0., 1.)), MovementSpeed(20.), Range(15.))).insert(
                            Text2dBundle {
                                text: Text::from_section(format!("{:.1}", **damage), TextStyle {
                                    font_size: 40.0,
                                    color: Color::WHITE,
                                    font: asset_server.load("font/pixel-font.ttf"),
                                }),
                                transform: Transform {
                                    translation: Vec3::new(enemy_transform.translation.x, enemy_transform.translation.y + 30., 10.),
                                    ..default()
                                },
                                ..default()
                            }
                        );
                    hitlist.push(ent)
                }
            }
        }
    }
}

pub fn handle_enemy_damage_to_player(
    enemy_query: Query<&Transform, With<Enemy>>,
    mut player_query: Query<(&Transform, &mut Health, &mut Vulnerability), With<Player>>
) {
    let (player_trans, mut player_health, mut vulnerability) = player_query.single_mut();
    let player_pos = player_trans.translation.xy();
    let invuln_timer = Duration::from_secs_f32(1.);
    if vulnerability.is_ready(invuln_timer) {
        for enemy_trans in &enemy_query {
            let enemy_pos = enemy_trans.translation.xy();
            if is_collision(player_pos, enemy_pos, 0., 100.) {
                **player_health = player_health.saturating_sub(1);
                vulnerability.reset(invuln_timer);
                return
            }
        }
    }
}

pub fn is_collision(obj1: Vec2, obj2: Vec2, obj1_radius: f32, obj2_radius: f32) -> bool {
    let diff = (obj1 - obj2).length();
    if diff < obj1_radius + obj2_radius {
        return true;
    }
    false
}
