use bevy::prelude::*;
use crate::{MovementSpeed, Direction};

#[derive(Component)]
pub struct Player;

#[derive(Component, Deref, Clone, Copy)]
pub struct ProjectileSpeed(f32);

#[derive(Component, Deref, Clone, Copy)]
pub struct MaxAttackCooldown(f32);

#[derive(Component, Deref, DerefMut, Clone, Copy)]
pub struct AttackCooldown(f32);

#[derive(Component, Deref, DerefMut, Clone, Copy)]
pub struct Damage(f32);

#[derive(Bundle)]
pub struct PlayerBundle {
    marker: Player,
    dir: Direction,
    sprite: SpriteBundle,
    speed: MovementSpeed,
    damage: Damage,
    projectile_speed: ProjectileSpeed,
    attack_speed: MaxAttackCooldown,
    attack_cooldown: AttackCooldown
}

impl PlayerBundle {
    pub fn new(sprite: SpriteBundle) -> Self {
        PlayerBundle {
            marker: Player,
            dir: default(),
            sprite,
            speed: MovementSpeed(300.),
            damage: Damage(1.0),
            projectile_speed: ProjectileSpeed(450.),
            attack_speed: MaxAttackCooldown(0.5),
            attack_cooldown: AttackCooldown(0.)
        }
    }
}

/// system for decreasing the attack cooldown timer of the player
pub fn tick_cooldown(time: Res<Time>, mut q: Query<&mut AttackCooldown, With<Player>>) {
    let mut cd = q.single_mut();
    if 0. < **cd {
        **cd -= time.delta_seconds();
    }
}