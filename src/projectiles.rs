use crate::{cleanup, cooldown::LifeTime, damage::HitList, player::Range, Heading, MovementSpeed};
use bevy::prelude::*;

#[derive(Component, Deref, DerefMut, Clone, Copy)]
pub struct RemDistance(pub f32);

#[derive(Bundle)]
pub struct ProjectileBundle {
    cleanup: cleanup::ExitGame,
    dir: Heading,
    speed: MovementSpeed,
    lifetime: LifeTime,
    has_hit: HitList,
}

impl ProjectileBundle {
    pub fn new(dir: Heading, speed: MovementSpeed, range: Range) -> Self {
        ProjectileBundle {
            cleanup: cleanup::ExitGame,
            dir,
            speed,
            lifetime: LifeTime::from_speed_and_range(speed, range),
            has_hit: HitList(vec![]),
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
