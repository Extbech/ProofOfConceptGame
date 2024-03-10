use bevy::{input::ButtonInput, prelude::*, sprite::MaterialMesh2dBundle, window::PrimaryWindow};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .add_systems(Startup, setup)
        .add_systems(Update, keyboard_input)
        .add_systems(Update, sync_player_and_camera_pos)
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
    commands.spawn(MaterialMesh2dBundle {
        mesh: meshes.add(Mesh::from(Rectangle::default())).into(),
        transform: Transform::default().with_scale(Vec3::new(4000., 4000., 1.)),
        material: materials.add(ColorMaterial::from(Color::PURPLE)),
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
    keys: Res<ButtonInput<KeyCode>>,
    mut player: Query<&mut Transform, With<Player>>,
) {
    let player_position = &mut player.single_mut().translation;
    if keys.pressed(KeyCode::KeyW) && player_position.y < 1900. {
        player_position.y += 5.0;
    }
    if keys.pressed(KeyCode::KeyS) && player_position.y > -1900. {
        player_position.y -= 5.0;
    }
    if keys.pressed(KeyCode::KeyD) && player_position.x < 1900. {
        player_position.x += 5.0;
    }
    if keys.pressed(KeyCode::KeyA) && player_position.x > -1900. {
        player_position.x -= 5.0;
    }
}

fn app_window_config(mut window: Query<&mut Window, With<PrimaryWindow>>) {
    let mut curr_window = window.single_mut();
    curr_window.title = "Vampire Survivors Copy Cat".to_string();
}
