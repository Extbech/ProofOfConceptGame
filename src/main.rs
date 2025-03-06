mod characters;
mod loot;
mod map;
mod mechanics;
mod mobs;
mod skills;
mod sprites;
mod start_game;
mod tools;
mod ui;

use bevy::{prelude::*, window::PrimaryWindow, winit::WinitWindows};
use characters::player::Player;
use mechanics::cooldown::InGameTime;
use mobs::enemy::{SpawnCooldown, SpawnRate};
use rand::{rngs::SmallRng, SeedableRng};
use skills::skills::ItemTooltips;
use sprites::sprites::add_sprite;
use start_game::GamePlugin;
use std::time::Duration;
use ui::start_menu::StartMenuPlugin;
use winit::window::Icon;

fn main() {
    App::new()
        .add_plugins(GamePlugin)
        .insert_state(AppState::MainMenu)
        .add_plugins(StartMenuPlugin {
            state: AppState::MainMenu,
        })
        .add_systems(Startup, (setup, set_window_icon))
        .add_systems(OnExit(AppState::InGame), set_state_not_started)
        .add_systems(Update, add_sprite)
        .run();
}

fn set_state_not_started(mut state: ResMut<NextState<GameState>>) {
    state.set(GameState::NotStarted);
}

#[derive(States, Debug, Clone, PartialEq, Eq, Hash)]
enum AppState {
    MainMenu,
    InGame,
}
#[derive(States, Debug, Clone, PartialEq, Eq, Hash)]
enum GameState {
    Paused,
    Running,
    LevelUp,
    NotStarted,
}
#[derive(Component)]
struct MyGameCamera;

/// normalized or zero vector pointing in some direction.
#[derive(Component, Clone, Copy, Deref, DerefMut)]
pub struct Heading {
    v: Vec2,
}

impl Heading {
    fn new(v: Vec2) -> Self {
        Heading {
            v: v.normalize_or_zero(),
        }
    }
}

impl Default for Heading {
    fn default() -> Self {
        Heading {
            v: Vec2::new(0., 1.),
        }
    }
}

#[derive(Resource, Deref, DerefMut)]
struct GameRng(SmallRng);

#[derive(Component, Deref, DerefMut, Clone, Copy)]
struct MovementSpeed(f32);

const SCALE: f32 = 1. / 3.;

fn setup(mut commands: Commands, window: Query<&mut Window, With<PrimaryWindow>>) {
    commands.spawn((
        Camera2d,
        Transform::from_scale(Vec3::new(1.0, 1.0, 1.0)),
        OrthographicProjection {
            far: 1000.,
            near: -1000.,
            scale: SCALE,
            ..OrthographicProjection::default_2d()
        },
        MyGameCamera,
    ));
    commands.insert_resource(SpawnRate(Duration::from_secs_f32(1.)));
    commands.insert_resource(GameRng(SmallRng::from_entropy()));
    commands.insert_resource(SpawnCooldown(default()));
    commands.insert_resource(CursorTranslation(Vec2::new(0., 0.)));
    commands.insert_resource(InGameTime::default());
    commands.insert_resource(ItemTooltips::default());
    app_window_config(window);
}

fn app_window_config(mut window: Query<&mut Window, With<PrimaryWindow>>) {
    let mut curr_window = window.single_mut();
    curr_window.title = ui::start_menu::GAME_TITLE.to_string();
}

/// ~ref bevy docs: https://bevy-cheatbook.github.io/window/icon.html
fn set_window_icon(windows: NonSend<WinitWindows>) {
    let (icon_rgba, icon_width, icon_height) = {
        let image = image::open("assets/window/game.png")
            .expect("Failed to open icon path")
            .into_rgba8();
        let (width, height) = image.dimensions();
        let rgba = image.into_raw();
        (rgba, width, height)
    };
    let icon = Icon::from_rgba(icon_rgba, icon_width, icon_height).unwrap();

    for window in windows.windows.values() {
        window.set_window_icon(Some(icon.clone()));
    }
}

#[derive(Resource, Default, Deref, DerefMut)]
struct CursorTranslation(Vec2);

fn update_cursor(
    mut mycoords: ResMut<CursorTranslation>,
    // query to get the window (so we can read the current cursor position)
    q_window: Query<&Window, With<PrimaryWindow>>,
    // query to get camera transform
    q_camera: Query<(&Camera, &GlobalTransform), With<MyGameCamera>>,
) {
    // get the camera info and transform
    // assuming there is exactly one main camera entity, so Query::single() is OK
    let (camera, camera_transform) = q_camera.single();

    // There is only one primary window, so we can similarly get it from the query:
    let window = q_window.single();

    // check if the cursor is inside the window and get its position
    // then, ask bevy to convert into world coordinates, and truncate to discard Z
    if let Some(world_position) = window
        .cursor_position()
        .and_then(|cursor| camera.viewport_to_world(camera_transform, cursor).ok())
        .map(|ray| ray.origin.truncate())
    {
        **mycoords = world_position;
    }
}

mod cleanup {
    use bevy::prelude::*;

    #[derive(Component)]
    pub struct ExitGame;

    #[derive(Component)]
    pub struct ExitLevelUpScreen;

    #[derive(Component)]
    pub struct ExitPauseScreen;
}

// Generic system that takes a component as a parameter, and will despawn all entities with that component
fn cleanup<T: Component>(to_despawn: Query<Entity, With<T>>, mut commands: Commands) {
    for entity in &to_despawn {
        commands.entity(entity).despawn_recursive();
    }
}
