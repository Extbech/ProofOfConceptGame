use crate::{
    characters::player::Range, cleanup, mechanics::cooldown::LifeTime, Heading, MovementSpeed,
    SCALE,
};
use bevy::prelude::*;

#[derive(Component, Deref, DerefMut, Clone, Copy)]
pub struct RemDistance(pub f32);

#[derive(Component, Deref, DerefMut, Clone, Copy)]
pub struct ShouldRotate(pub bool);

#[derive(Component)]
pub struct ProjectileMarker;

#[derive(Bundle)]
pub struct ProjectileBundle {
    cleanup: cleanup::ExitGame,
    dir: Heading,
    speed: MovementSpeed,
    lifetime: LifeTime,
    marker: ProjectileMarker,
    should_rotate: ShouldRotate,
}

impl ProjectileBundle {
    pub fn new(
        dir: Heading,
        speed: MovementSpeed,
        range: Range,
        should_rotate: ShouldRotate,
    ) -> Self {
        ProjectileBundle {
            cleanup: cleanup::ExitGame,
            dir,
            speed,
            lifetime: LifeTime::from_speed_and_range(speed, range),
            marker: ProjectileMarker,
            should_rotate,
        }
    }
}
pub fn handle_projectile_rotation(
    mut q: Query<(&Heading, &mut Transform, &ShouldRotate), With<ProjectileMarker>>,
) {
    for (dir, mut tran, should_rotate) in &mut q {
        if **should_rotate {
            let angle = dir.y.atan2(dir.x);
            tran.rotation = Quat::from_axis_angle(Vec3::new(0., 0., 1.), angle);
        }
    }
}
pub fn speed_to_movement(
    time: Res<Time>,
    mut q: Query<(&Heading, &mut Transform, &MovementSpeed)>,
) {
    for (dir, mut tran, &speed) in &mut q {
        let pos = &mut tran.translation;
        (pos.x, pos.y) =
            (Vec2::new(pos.x, pos.y) + *speed * SCALE * time.delta_secs() * dir.v).into();
    }
}

#[derive(Component, Default, Deref, DerefMut)]
pub struct AngularVelocity(pub f32);

#[derive(Component, Default, Deref, DerefMut)]
pub struct Angle(pub f32);

#[derive(Component, Default, Deref, DerefMut)]
pub struct OrbitalRadius(pub f32);

#[derive(Bundle, Default)]
pub struct OrbitingBundle {
    pub vel: AngularVelocity,
    pub angle: Angle,
    pub radius: OrbitalRadius,
    pub sprite: Sprite,
}

pub fn orbital_movement(time: Res<Time>, mut query: Query<(&AngularVelocity, &mut Angle)>) {
    for (vel, mut ang) in &mut query {
        **ang += **vel * time.delta_secs();
        **ang %= std::f32::consts::TAU;
    }
}

pub fn orbital_position(mut query: Query<(&mut Transform, &OrbitalRadius, &Angle)>) {
    for (mut trans, rad, ang) in &mut query {
        (trans.translation.x, trans.translation.y) = (**rad * Vec2::from_angle(**ang)).into();
    }
}
