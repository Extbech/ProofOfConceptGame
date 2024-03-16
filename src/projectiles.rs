use crate::{cleanup, player::Damage, Direction, MovementSpeed};
use bevy::prelude::*;

#[derive(Component, Deref, DerefMut, Clone, Copy)]
pub struct RemDistance(pub f32);

/// Projectiles are entities that move in a straight line at a fixed speed
#[derive(Component)]
pub struct Projectile;

#[derive(Component, Deref, DerefMut)]
pub struct HitList(Vec<Entity>);

#[derive(Bundle)]
pub struct ProjectileBundle {
    cleanup: cleanup::ExitGame,
    marker: Projectile,
    dir: Direction,
    sprite: SpriteBundle,
    speed: MovementSpeed,
    rem_distance: RemDistance,
    has_hit: HitList
}

impl ProjectileBundle {
    pub fn new(sprite: SpriteBundle, dir: Direction, speed: MovementSpeed, rem_distance: RemDistance) -> Self {
        ProjectileBundle {
            cleanup: cleanup::ExitGame,
            marker: Projectile,
            dir,
            sprite,
            speed,
            rem_distance,
            has_hit: HitList(vec![])
        }
    }
}

pub fn projectile_movement(
    mut commands: Commands,
    time: Res<Time>,
    mut q: Query<(&Direction, &mut Transform, &MovementSpeed, &mut RemDistance, Entity), With<Projectile>>,
) {
    for (dir, mut tran, &speed, mut rem_dist, ent) in &mut q {
        if 0. <= **rem_dist {    
            let pos = &mut tran.translation;
            **rem_dist -= *speed * time.delta_seconds();
            (pos.x, pos.y) = (Vec2::new(pos.x, pos.y) + *speed * time.delta_seconds() * dir.v).into();
        } else {
            commands.entity(ent).despawn();
        }
    }
}
