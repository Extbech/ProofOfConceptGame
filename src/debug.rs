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
    q: Query<(&Radius, Entity), (With<Transform>, Without<ShowsRadius>)>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>
) {
    let color = materials.add(Color::Rgba { red: 1., green: 0., blue: 0., alpha: 0.3 });
    for (rad, ent) in &q {
        let circ = Mesh2dHandle(meshes.add(Circle::new(**rad)));
        commands.entity(ent).insert(ShowsRadius).with_children(|parent| {
            parent.spawn(MaterialMesh2dBundle {
                mesh: circ,
                material: color.clone(),
                transform: Transform::from_xyz(
                    0.,
                    0.,
                    100.,
                ),
                ..default()
        });
        });
    }
}
