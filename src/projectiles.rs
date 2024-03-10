use crate::Direction;
use bevy::{asset::io::memory::Dir, prelude::*};

/// Projectiles are entities that move in a straight line at a fixed speed
#[derive(Component)]
pub struct Projectile;

#[derive(Bundle)]
pub struct ProjectileBundle {
    marker: Projectile,
    dir: Direction,
    sprite: SpriteBundle,
}

impl ProjectileBundle {
    pub fn new(sprite: SpriteBundle, dir: Direction) -> Self {
        ProjectileBundle {
            marker: Projectile,
            dir,
            sprite,
        }
    }
}

pub fn projectile_movement(mut q: Query<(&Direction, &mut Transform), With<Projectile>>) {
    for (dir, mut tran) in &mut q {
        const SPEED: f32 = 5.0;
        let pos = &mut tran.translation;
        pos.x += SPEED * dir.v.x;
        pos.y += SPEED * dir.v.y;
    }
}
