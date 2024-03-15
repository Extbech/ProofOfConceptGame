use std::time::Duration;

use crate::cooldown::Cooldown;
use crate::{Direction, MovementSpeed};
use bevy::prelude::*;
use bevy::sprite::MaterialMesh2dBundle;

pub const XP_SCALING_FACTOR: f32 = 1.5;
#[derive(Component)]
pub struct Player;

#[derive(Component, Deref, Clone, Copy)]
pub struct ProjectileSpeed(f32);

#[derive(Component, Deref, DerefMut, Clone, Copy)]
pub struct MaxAttackCooldown(Duration);

#[derive(Component, Deref, DerefMut, Clone)]
pub struct AttackCooldown(Cooldown);

#[derive(Component, Deref, DerefMut, Clone, Copy)]
pub struct Damage(f32);

#[derive(Component, Deref, DerefMut, Clone, Copy)]
pub struct Range(f32);

#[derive(Component, Deref, DerefMut, Clone, Copy)]
pub struct CurrentXP(f32);

#[derive(Component, Deref, DerefMut, Clone, Copy)]
pub struct RequiredXP(f32);

#[derive(Component, Deref, DerefMut, Clone, Copy)]
pub struct CurrentLevel(usize);

#[derive(Component, Deref, Clone, Copy)]
pub struct MaxLevel(usize);

#[derive(Component, Deref, DerefMut, Clone, Copy)]
pub struct PickUpRadius(f32);

#[derive(Bundle)]
pub struct ProjectileStatBundle {
    damage: Damage,
    projectile_speed: ProjectileSpeed,
    range: Range,
}

#[derive(Bundle)]
pub struct PlayerBundle {
    marker: Player,
    dir: Direction,
    sprite: SpriteSheetBundle,
    speed: MovementSpeed,
    attack_cooldown: AttackCooldown,
    projectile_stats: ProjectileStatBundle,
    current_xp: CurrentXP,
    required_xp: RequiredXP,
    current_level: CurrentLevel,
    max_level: MaxLevel,
    max_attack_cooldown: MaxAttackCooldown,
    pick_up_radius: PickUpRadius,
}

impl PlayerBundle {
    pub fn new(sprite: SpriteSheetBundle) -> Self {
        PlayerBundle {
            marker: Player,
            dir: default(),
            sprite,
            speed: MovementSpeed(300.),
            attack_cooldown: AttackCooldown(default()),
            max_attack_cooldown: MaxAttackCooldown(Duration::from_secs_f32(0.5)),
            projectile_stats: ProjectileStatBundle {
                damage: Damage(1.0),
                projectile_speed: ProjectileSpeed(450.),
                range: Range(500.),
            },
            current_xp: CurrentXP(0.0),
            required_xp: RequiredXP(100.0),
            current_level: CurrentLevel(1),
            max_level: MaxLevel(10),
            pick_up_radius: PickUpRadius(100.0),
        }
    }
}

pub fn spawn_player_hero(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let texture_handle: Handle<Image> = asset_server.load("models/cowboy_hero.png");
    let layout = TextureAtlasLayout::from_grid(Vec2::new(45.0, 45.0), 8, 4, None, None);
    let texture_atlas_layout = texture_atlas_layouts.add(layout);
    commands
        .spawn(PlayerBundle::new(SpriteSheetBundle {
            texture: texture_handle,
            atlas: TextureAtlas {
                layout: texture_atlas_layout,
                index: 0,
            },
            transform: Transform::from_xyz(0.0, 0.0, 3.0),
            sprite: Sprite {
                custom_size: Some(Vec2::new(70.0, 70.0)),
                ..Default::default()
            },
            ..default()
        }))
        .with_children(|child| {
            child.spawn(MaterialMesh2dBundle {
                mesh: bevy::sprite::Mesh2dHandle(meshes.add(Rectangle::new(70.0, 20.0))),
                material: materials.add(Color::GREEN),
                transform: Transform::from_xyz(0.0, 50.0, 1.0),
                ..default()
            });
        });
}

pub fn handle_player_xp(
    mut commands: Commands,
    mut query: Query<
        (
            &Transform,
            &mut CurrentXP,
            &mut RequiredXP,
            &mut CurrentLevel,
            &MaxLevel,
        ),
        With<Player>,
    >,
) {
    let (transform, mut current_xp, mut required_xp, mut current_level, max_level) =
        query.single_mut();
    if **current_xp >= **required_xp {
        if **current_level < **max_level {
            **current_level += 1;
            **current_xp = **current_xp - **required_xp;
            **required_xp = **required_xp * XP_SCALING_FACTOR;
        }
    }
    println!("Level: {}", **current_level);
    println!("Current xp: {}", **current_xp);
    println!("Current xp: {}", **required_xp);
}
