use std::f32::consts::PI;

use bevy::prelude::*;

use crate::mechanics::damage::{self, DealDamageHitbox, TakeDamageHitbox};

#[derive(Component)]
struct ShowDamagingHitbox;

#[derive(Component)]
struct ShowWeaknessHitbox;

pub struct DebugPlugin;

impl Plugin for DebugPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PreUpdate, (show_damaging, show_weakness));
    }
}

fn show_weakness(
    mut commands: Commands,
    q: Query<(&TakeDamageHitbox, Entity), (With<Transform>, Without<ShowWeaknessHitbox>)>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let color = materials.add(Color::Srgba(Srgba {
        red: 1.,
        green: 0.,
        blue: 0.,
        alpha: 0.3,
    }));
    for (TakeDamageHitbox(damage::Circle{radius}), ent) in &q {
        if let Some(mut inent) = commands.get_entity(ent) {
            inent.insert(ShowWeaknessHitbox);
            inent.with_children(|parent| {
                parent.spawn((
                    Mesh2d(meshes.add(Circle::new(*radius))),
                    MeshMaterial2d(color.clone()),
                    Transform::from_xyz(0., 0., 100.),
                ));
            });
        }
    }
}

fn show_damaging(
    mut commands: Commands,
    q: Query<(&DealDamageHitbox, Entity), (With<Transform>, Without<ShowDamagingHitbox>)>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let color = materials.add(Color::Srgba(Srgba {
        red: 1.,
        green: 0.,
        blue: 0.,
        alpha: 0.3,
    }));
    for (hitbox, ent) in &q {
        if let Some(mut inent) = commands.get_entity(ent) {
            inent.insert(ShowDamagingHitbox);
            match hitbox {
                DealDamageHitbox::Circle(damage::Circle { radius }) => {
                    inent.with_children(|parent| {
                        parent.spawn((
                            Mesh2d(meshes.add(Circle::new(*radius))),
                            MeshMaterial2d(color.clone()),
                            Transform::from_xyz(0., 0., 100.),
                        ));
                    });
                }
                DealDamageHitbox::Cone(damage::Cone {
                    mid_angle,
                    angular_width,
                }) => {
                    let mut tf = Transform::from_xyz(0., 0., 100.);
                    tf.rotate_z(2.*PI-mid_angle.to_angle().rem_euclid(2.*PI));
                    inent.with_children(|parent| {
                        parent.spawn((
                            Mesh2d(meshes.add(CircularSector::new(mid_angle.length(), *angular_width))),
                            MeshMaterial2d(color.clone()),
                            tf,
                        ));
                    });
                }
                DealDamageHitbox::Global => {}
            }
        };
    }
}
