mod cooldown;
mod enemy;
mod loot;
mod map;
mod player;
mod projectiles;
mod start_game;
mod start_menu;
use std::time::Duration;

use bevy::{prelude::*, window::PrimaryWindow};
use bevy_ecs_tilemap::TilemapPlugin;
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use enemy::{handle_enemy_collision, spawn_enemies, update_enemies, SpawnCooldown, SpawnRate};
use player::{
    handle_player_xp, player_movement, player_shooting_mouse_dir, spawn_player_hero,
    sync_player_and_camera_pos, AttackCooldown, Player,
};
use projectiles::projectile_movement;
use rand::{rngs::SmallRng, SeedableRng};

use loot::{animate_sprite, check_for_dead_enemies, pick_up_xp_orbs, xp_orbs_collision};
use start_game::GamePlugin;
use start_menu::StartMenuPlugin;

fn main() {
    App::new()
        .insert_state(AppState::MainMenu)
        .add_plugins(StartMenuPlugin {
            state: AppState::MainMenu,
        })
        .add_plugins(GamePlugin {
            state: AppState::InGame,
        })
        .add_systems(Startup, setup)
        .run();
}

#[derive(States, Debug, Clone, PartialEq, Eq, Hash)]
enum AppState {
    LoadingScreen,
    MainMenu,
    InGame,
}
#[derive(States, Default, Debug, Clone, PartialEq, Eq, Hash)]
enum PausedState {
    #[default]
    Paused,
    Running,
}
#[derive(Component)]
struct MyGameCamera;

/// normalized vector pointing in some direction. Is always nonzero
#[derive(Component, Clone, Copy, Deref, DerefMut)]
pub struct Direction {
    v: Vec2,
}

impl Direction {
    fn try_new(v: Vec2) -> Option<Self> {
        Some(Direction {
            v: v.try_normalize()?,
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

#[derive(Resource, Deref, DerefMut)]
struct GameRng(SmallRng);

#[derive(Component, Deref, DerefMut, Clone, Copy)]
struct MovementSpeed(f32);

fn setup(mut commands: Commands, window: Query<&mut Window, With<PrimaryWindow>>) {
    commands.spawn((
        Camera2dBundle {
            transform: Transform::from_scale(Vec3::new(1.0, 1.0, 1.0)),
            ..Default::default()
        },
        MyGameCamera,
    ));
    commands.insert_resource(SpawnRate(Duration::from_secs_f32(1.)));
    commands.insert_resource(GameRng(SmallRng::from_entropy()));
    commands.insert_resource(SpawnCooldown(default()));
    commands.insert_resource(CursorTranslation(Vec2::new(0., 0.)));
    app_window_config(window);
}

fn app_window_config(mut window: Query<&mut Window, With<PrimaryWindow>>) {
    let mut curr_window = window.single_mut();
    curr_window.title = start_menu::GAME_TITLE.to_string();
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
        .and_then(|cursor| camera.viewport_to_world(camera_transform, cursor))
        .map(|ray| ray.origin.truncate())
    {
        **mycoords = world_position;
    }
}
