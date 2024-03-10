use bevy::{input::ButtonInput, prelude::*};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(Update, keyboard_input)
        .add_systems(Update, sync_player_and_camera_pos)
        .run();
}

#[derive(Component)]
struct MyGameCamera;

#[derive(Component)]
struct Player;

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((
        Camera2dBundle {
            transform: Transform::from_xyz(0., 0., 1000.).with_scale(Vec3::new(2., 2., 1.)),
            ..default()
        },
        MyGameCamera,
    ));
    commands.spawn((
        SpriteBundle {
            texture: asset_server.load("models/sprite1.png"),
            ..default()
        },
        Player,
    ));
    commands.spawn(SpriteBundle {
        texture: asset_server.load("models/sprite1.png"),
        ..default()
    });
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
    keys: Res<ButtonInput<KeyCode>>,
    mut player: Query<&mut Transform, With<Player>>,
) {
    if keys.pressed(KeyCode::KeyW) {
        player.single_mut().translation.y -= 5.0;
    }
    if keys.pressed(KeyCode::KeyS) {
        player.single_mut().translation.y += 5.0;
    }
    if keys.pressed(KeyCode::KeyD) {
        player.single_mut().translation.x -= 5.0;
    }
    if keys.pressed(KeyCode::KeyA) {
        player.single_mut().translation.x += 5.0;
    }
}
