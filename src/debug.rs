use bevy::prelude::*;

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
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let color = materials.add(Color::Srgba(Srgba {
        red: 1.,
        green: 0.,
        blue: 0.,
        alpha: 0.3,
    }));
    for (rad, ent) in &q {
        if let Some(mut inent) = commands.get_entity(ent) {
            inent.insert(ShowsRadius).with_children(|parent| {
                parent.spawn((
                    Mesh2d(meshes.add(Circle::new(**rad)).into()),
                    MeshMaterial2d(color.clone()),
                    Transform::from_xyz(0., 0., 100.),
                ));
            });
        };
    }
}
