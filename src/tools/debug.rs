use std::f32::consts::PI;

use bevy::prelude::*;

use crate::mechanics::damage::{self, DealDamageHitbox};

#[derive(Component)]
pub struct ShowHitbox;

struct DebugSystem {}

pub fn show_radius(
    mut commands: Commands,
    q: Query<(&DealDamageHitbox, Entity), (With<Transform>, Without<ShowHitbox>)>,
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
            inent.insert(ShowHitbox);
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
                    tf.rotate_z(mid_angle.to_angle() % 2. * PI);
                    inent.with_children(|parent| {
                        parent.spawn((
                            Mesh2d(
                                meshes.add(CircularSector::new(
                                    mid_angle.length(),
                                    2. * angular_width,
                                )),
                            ),
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
