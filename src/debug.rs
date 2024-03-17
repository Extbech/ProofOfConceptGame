use bevy::{prelude::*, sprite::{MaterialMesh2dBundle, Mesh2dHandle}};

use crate::damage::Radius;

pub fn count<C: Component>(q: Query<(), With<C>>) {
    let mut count = 0;
    for _ in &q {
        count += 1;
    }
    println!("{}", count)
}

#[derive(Component)]
pub struct ShowsRadius;

pub fn show_radius(
    mut commands: Commands, 
    q: Query<(&Transform, &Radius, Entity), Without<ShowsRadius>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>
) {
    let redc = materials.add(Color::RED);
    let greenc = materials.add(Color::GREEN);
    for (trans, real_rad, ent) in &q {
        let color;
        let rad;
        if 0. < **real_rad {
            color = &redc;
            rad = real_rad;
        }
        else {
            color = &greenc;
            rad = &Radius(5.);
        }
        let circ = Mesh2dHandle(meshes.add(Circle::new(**rad)));
        commands.entity(ent).insert(ShowsRadius).with_children(|parent| {
            parent.spawn(MaterialMesh2dBundle {
                mesh: circ,
                material: color.clone(),
                transform: Transform::from_xyz(
                    0.,
                    0.,
                    -1.,
                ),
                ..default()
        });
        });
    }
}