use bevy::prelude::*;
#[derive(Component, Default, Deref, DerefMut)]
pub struct AngularVelocity(pub f32);

#[derive(Component, Default, Deref, DerefMut)]
pub struct Angle(pub f32);

#[derive(Component, Default, Deref, DerefMut)]
pub struct OrbitalRadius(pub f32);

pub fn orbital_movement(time: Res<Time>, mut query: Query<(&AngularVelocity, &mut Angle)>) {
    for (vel, mut ang) in &mut query {
        **ang += **vel * time.delta_secs();
        **ang %= std::f32::consts::TAU;
    }
}

pub fn update_orbital_position(mut query: Query<(&mut Transform, &OrbitalRadius, &Angle)>) {
    for (mut trans, rad, ang) in &mut query {
        (trans.translation.x, trans.translation.y) = (**rad * Vec2::from_angle(**ang)).into();
    }
}
