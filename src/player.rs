use crate::{Direction, MovementSpeed};
use bevy::prelude::*;
use crate::cooldown::Cooldown;

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
    max_attack_cooldown: MaxAttackCooldown,
    projectile_stats: ProjectileStatBundle
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
        }
    }
}

pub fn spawn_player_hero(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    let texture_handle: Handle<Image> = asset_server.load("models/cowboy_hero.png");
    let layout = TextureAtlasLayout::from_grid(Vec2::new(45.0, 45.0), 8, 4, None, None);
    let texture_atlas_layout = texture_atlas_layouts.add(layout);
    commands.spawn(PlayerBundle::new(SpriteSheetBundle {
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
    }));
}
