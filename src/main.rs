mod projectiles;

use projectiles::{projectile_movement, ProjectileBundle};
use bevy::{input::ButtonInput, prelude::*, sprite::MaterialMesh2dBundle, window::PrimaryWindow};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .add_systems(Startup, setup)
        .add_systems(Update, (keyboard_input, sync_player_and_camera_pos, projectile_movement))
        .run();
}

#[derive(Component)]
struct MyGameCamera;

#[derive(Component)]
struct Player;

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    window: Query<&mut Window, With<PrimaryWindow>>,
) {
    commands.spawn((
        {
            let mut cam = Camera2dBundle::default();
            cam.transform = cam.transform.with_scale(Vec3::new(3., 3., 1.));
            cam
        },
        MyGameCamera,
    ));
    commands.spawn((
        SpriteBundle {
            transform: Transform::from_xyz(0., 0., 1.),
            texture: asset_server.load("models/sprite1.png"),
            ..default()
        },
        Player,
    ));
    let tile1: Handle<Image> = asset_server.load("environment/backgrounddetailed1.png");
    let tile2: Handle<Image> = asset_server.load("environment/backgrounddetailed2.png");
    commands.spawn(MaterialMesh2dBundle {
        mesh: meshes.add(Mesh::from(Rectangle::default())).into(),
        transform: Transform::default().with_scale(Vec3::new(4000., 4000., 1.)),
        material: materials.add(ColorMaterial::from(tile1)),
        ..default()
    });
    app_window_config(window);
}

fn sync_player_and_camera_pos(
    player: Query<&Transform, With<Player>>,
    mut cam: Query<&mut Transform, (With<MyGameCamera>, Without<Player>)>,
) {
    let player = player.single();
    let mut cam = cam.single_mut();
    cam.translation.y = player.translation.y;
    cam.translation.x = player.translation.x;
}

fn keyboard_input(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    keys: Res<ButtonInput<KeyCode>>,
    mut player: Query<&mut Transform, With<Player>>,
) {
    const SPEED: f32 = 5.;
    let player_dir_x = if keys.pressed(KeyCode::KeyA) {0.} else {1.} - if keys.pressed(KeyCode::KeyD) {0.} else {1.};
    let player_dir_y = if keys.pressed(KeyCode::KeyS) {0.} else {1.} - if keys.pressed(KeyCode::KeyW) {0.} else {1.};
    let player_position = &mut player.single_mut().translation;
    const BOUND: f32 = 1900.;
    player_position.x = (player_position.x + SPEED * player_dir_x).clamp(-BOUND, BOUND);
    player_position.y = (player_position.y + SPEED * player_dir_y).clamp(-BOUND, BOUND);
    if keys.pressed(KeyCode::KeyJ) {
        commands.spawn(ProjectileBundle::new(SpriteBundle {
            transform: Transform::from_xyz(player_position.x, player_position.y, 1.),
            texture: asset_server.load("models/sprite1.png"),
            ..default()
        },
    Vec2::new(player_dir_x, player_dir_y)));
    }
}

fn app_window_config(mut window: Query<&mut Window, With<PrimaryWindow>>) {
    let mut curr_window = window.single_mut();
    curr_window.title = "Vampire Survivors Copy Cat".to_string();
}
