use std::time::Duration;

use crate::{cleanup, player::Range, Direction, Heading, MovementSpeed};
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
    dir: Heading,
    speed: MovementSpeed,
    lifetime: LifeTime,
    has_hit: HitList
}

impl ProjectileBundle {
    pub fn new(dir: Heading, speed: MovementSpeed, range: Range) -> Self {
        ProjectileBundle {
            cleanup: cleanup::ExitGame,
            marker: Projectile,
            dir,
            speed,
            lifetime: LifeTime::from_speed_and_range(speed, range),
            has_hit: HitList(vec![])
        }
    }
}

pub fn speed_to_movement(
    time: Res<Time>,
    mut q: Query<(&Heading, &mut Transform, &MovementSpeed)>,
) {
    for (dir, mut tran, &speed) in &mut q {
        let pos = &mut tran.translation;
        (pos.x, pos.y) = (Vec2::new(pos.x, pos.y) + *speed * time.delta_seconds() * dir.v).into();
    }
}

#[derive(Component)]
pub struct LifeTime(Duration);

impl LifeTime {
    pub fn from_speed_and_range(spd: MovementSpeed, rng: Range) -> Self {
        LifeTime(Duration::from_secs_f32(*rng / *spd)) 
    }

    pub fn try_decrease(&mut self, by: Duration) -> bool {
        match self.0.checked_sub(by) {
            Some(dur) => {self.0 = dur; true}
            None => false,
        }
    }
}

pub fn handle_lifetime(
    mut commands: Commands,
    time: Res<Time>,
    mut q: Query<(&mut LifeTime, Entity)>
) {
    for (mut lt, ent) in &mut q {
        if !lt.try_decrease(time.delta()) {
            commands.entity(ent).despawn_recursive();
        }
    }
}