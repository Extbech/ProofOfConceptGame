use bevy::prelude::*;
use crate::{MovementSpeed, Direction};

#[derive(Component)]
pub struct Player;

#[derive(Component, Deref, Clone, Copy)]
pub struct ProjectileSpeed(f32);

#[derive(Component, Deref, Clone, Copy)]
pub struct AttackSpeed(f32);

#[derive(Bundle)]
pub struct PlayerBundle {
    marker: Player,
    dir: Direction,
    sprite: SpriteBundle,
    speed: MovementSpeed,
    projectile_speed: ProjectileSpeed,
    attack_speed: AttackSpeed,
}

impl PlayerBundle {
    pub fn new(sprite: SpriteBundle) -> Self {
        PlayerBundle {
            marker: Player,
            dir: default(),
            sprite,
            speed: MovementSpeed(5.),
            projectile_speed: ProjectileSpeed(5.),
            attack_speed: AttackSpeed(5.)
        }
    }
}