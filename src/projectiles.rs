use crate::{Direction, MovementSpeed};
use bevy::prelude::*;

/// Projectiles are entities that move in a straight line at a fixed speed
#[derive(Component)]
pub struct Projectile;

#[derive(Bundle)]
pub struct ProjectileBundle {
    marker: Projectile,
    dir: Direction,
    sprite: SpriteBundle,
    speed: MovementSpeed,
}

impl ProjectileBundle {
    pub fn new(sprite: SpriteBundle, dir: Direction, speed: MovementSpeed) -> Self {
        ProjectileBundle {
            marker: Projectile,
            dir,
            sprite,
            speed,
        }
    }
}

pub fn projectile_movement(
    time: Res<Time>,
    mut q: Query<(&Direction, &mut Transform, &MovementSpeed), With<Projectile>>,
) {
    for (dir, mut tran, &speed) in &mut q {
        let pos = &mut tran.translation;
        (pos.x, pos.y) = (Vec2::new(pos.x, pos.y) + *speed * time.delta_seconds() * dir.v).into();
    }
}
