mod map;
mod player;
mod projectiles;
use bevy::{
    diagnostic::DiagnosticsStore, diagnostic::FrameTimeDiagnosticsPlugin, input::ButtonInput,
    prelude::*, window::PrimaryWindow,
};
use projectiles::{projectile_movement, ProjectileBundle};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .add_plugins(FrameTimeDiagnosticsPlugin::default())
        .add_systems(Startup, (setup, map::setup_map))
        .add_systems(
            Update,
            (
                keyboard_input,
                sync_player_and_camera_pos,
                projectile_movement,
            ),
        )
        .run();
}

#[derive(Component)]
struct MyGameCamera;

/// normalized vector pointing in some direction. Is always nonzero
#[derive(Component, Clone, Copy)]
pub struct Direction {
    v: Vec2,
}

impl Direction {
    fn try_new(x: f32, y: f32) -> Option<Self> {
        Some(Direction {
            v: Vec2::new(x, y).try_normalize()?,
        })
    }
}

impl Default for Direction {
    fn default() -> Self {
        Direction {
            v: Vec2::new(0., 1.),
        }
    }
}

#[derive(Component)]
struct Speed(f32);

#[derive(Component)]
struct Player;

#[derive(Bundle)]
pub struct PlayerBundle {
    marker: Player,
    dir: Direction,
    sprite: SpriteBundle,
    speed: Speed
}

impl PlayerBundle {
    fn new(sprite: SpriteBundle) -> Self {
        PlayerBundle {
            marker: Player,
            dir: default(),
            sprite,
            speed: Speed(5.)
        }
    }
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
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
    commands.spawn(PlayerBundle::new(SpriteBundle {
        transform: Transform::from_xyz(0., 0., 1.),
        texture: asset_server.load("models/hero.png"),
        ..default()
    }));
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
    mut player: Query<(&mut Transform, &mut Direction, &Speed), With<Player>>,
    diagnostics: Res<DiagnosticsStore>,
) {
    let (mut player_trans, mut player_dir, player_speed) = player.single_mut();
    let player_position = &mut player_trans.translation;
    let keyboard_dir_x = if keys.pressed(KeyCode::KeyA) {0.} else {1.} - if keys.pressed(KeyCode::KeyD) {0.} else {1.};
    let keyboard_dir_y = if keys.pressed(KeyCode::KeyS) {0.} else {1.} - if keys.pressed(KeyCode::KeyW) {0.} else {1.};
    
    let keyboard_dir = Vec2::new(keyboard_dir_x, keyboard_dir_y).normalize_or_zero();
    const BOUND: f32 = 1900.;
    player_position.x = (player_position.x + player_speed.0 * keyboard_dir.x).clamp(-BOUND, BOUND);
    player_position.y = (player_position.y + player_speed.0 * keyboard_dir.y).clamp(-BOUND, BOUND);
    *player_dir = Direction::try_new(keyboard_dir.x, keyboard_dir.y).unwrap_or(*player_dir);
    if keys.pressed(KeyCode::KeyJ) {
        commands.spawn(ProjectileBundle::new(
            SpriteBundle {
                transform: Transform::from_xyz(player_position.x, player_position.y, 1.),
                texture: asset_server.load("models/hero.png"),
                ..default()
            },
            *player_dir,
            Speed(5.)
        ));
    }
    // try to get a "smoothed" FPS value from Bevy
    if let Some(value) = diagnostics
        .get(&FrameTimeDiagnosticsPlugin::FPS)
        .and_then(|fps| fps.smoothed())
    {
        println!("fps: {}", value);
    }
}

fn app_window_config(mut window: Query<&mut Window, With<PrimaryWindow>>) {
    let mut curr_window = window.single_mut();
    curr_window.title = "Vampire Survivors Copy Cat".to_string();
}
