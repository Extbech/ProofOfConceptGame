use crate::cooldown::Cooldown;
use crate::{Direction, MovementSpeed};
use bevy::prelude::*;
use bevy::sprite::MaterialMesh2dBundle;

#[derive(Component)]
pub struct Player;

#[derive(Component, Deref, Clone, Copy)]
pub struct ProjectileSpeed(f32);

#[derive(Component, Deref, DerefMut, Clone, Copy)]
pub struct MaxAttackCooldown(f32);

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
    max_attack_cooldown: MaxAttackCooldown,
}

impl PlayerBundle {
    pub fn new(sprite: SpriteSheetBundle) -> Self {
        PlayerBundle {
            marker: Player,
            dir: default(),
            sprite,
            speed: MovementSpeed(300.),
            attack_cooldown: AttackCooldown(default()),
            max_attack_cooldown: MaxAttackCooldown(0.5),
            projectile_stats: ProjectileStatBundle {
                damage: Damage(1.0),
                projectile_speed: ProjectileSpeed(450.),
                range: Range(500.),
            },
            current_xp: CurrentXP(0.0),
            required_xp: RequiredXP(100.0),
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
