use crate::{Direction, Speed};
use bevy::prelude::*;

/// Projectiles are entities that move in a straight line at a fixed speed
#[derive(Component)]
pub struct Projectile;

#[derive(Bundle)]
pub struct ProjectileBundle {
    marker: Projectile,
    dir: Direction,
    sprite: SpriteBundle,
    speed: Speed
}

impl ProjectileBundle {
    pub fn new(sprite: SpriteBundle, dir: Direction, speed: Speed) -> Self {
        ProjectileBundle {
            marker: Projectile,
            dir,
            sprite,
            speed
        }
    }
}

pub fn projectile_movement(mut q: Query<(&Direction, &mut Transform, &Speed), With<Projectile>>) {
    for (dir, mut tran, &speed) in &mut q {
        let pos = &mut tran.translation;
        (pos.x, pos.y) =  (Vec2::new(pos.x, pos.y) + *speed * dir.v).into();
    }
}
