use crate::{cleanup, cooldown::LifeTime, player::Range, Heading, MovementSpeed};
use bevy::prelude::*;

#[derive(Component, Deref, DerefMut, Clone, Copy)]
pub struct RemDistance(pub f32);

#[derive(Bundle)]
pub struct ProjectileBundle {
    cleanup: cleanup::ExitGame,
    dir: Heading,
    speed: MovementSpeed,
    lifetime: LifeTime,
}

impl ProjectileBundle {
    pub fn new(dir: Heading, speed: MovementSpeed, range: Range) -> Self {
        ProjectileBundle {
            cleanup: cleanup::ExitGame,
            dir,
            speed,
            lifetime: LifeTime::from_speed_and_range(speed, range),
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

#[derive(Component, Default, Deref, DerefMut)]
pub struct AngularVelocity(pub f32);

#[derive(Component, Default, Deref, DerefMut)]
pub struct Angle(f32);

#[derive(Component, Default, Deref, DerefMut)]
pub struct OrbitalRadius(pub f32);

#[derive(Bundle, Default)]
pub struct OrbitingBundle {
    pub vel: AngularVelocity,
    pub angle: Angle,
    pub radius: OrbitalRadius,
    pub spatial: SpatialBundle,
}

pub fn orbital_movement(time: Res<Time>, mut query: Query<(&AngularVelocity, &mut Angle)>) {
    for (vel, mut ang) in &mut query {
        **ang += **vel * time.delta_seconds();
        **ang %= std::f32::consts::TAU;
    }
}

pub fn orbital_position(mut query: Query<(&mut Transform, &OrbitalRadius, &Angle)>) {
    for (mut trans, rad, ang) in &mut query {
        (trans.translation.x, trans.translation.y) = (**rad * Vec2::from_angle(**ang)).into();
    }
}
