use bevy::{prelude::*, sprite::MaterialMesh2dBundle};

pub fn setup_map(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let tile1: Handle<Image> = asset_server.load("environment/backgrounddetailed1.png");
    let _tile2: Handle<Image> = asset_server.load("environment/backgrounddetailed2.png");
    commands.spawn(MaterialMesh2dBundle {
        mesh: meshes.add(Mesh::from(Rectangle::default())).into(),
        transform: Transform::default().with_scale(Vec3::new(4000., 4000., 1.)),
        material: materials.add(ColorMaterial::from(tile1)),
        ..default()
    });
}
