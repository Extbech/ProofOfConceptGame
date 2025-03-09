use crate::{
    characters::player::{
        handle_player_death, handle_player_xp, player_attack_facing_from_mouse,
        player_shooting, spawn_player_hero,
    },
    cleanup,
    loot::loot::{
        activate_xp_orb_movement, animate_sprite, check_for_dead_enemies, handle_xp_orb_movement,
        pickup_loot, xp_orbs_collision,
    },
    map::map_plugin::MapPlugin,
    mechanics::{
        cooldown::{handle_ingametime, reset_ingametime, CooldownPlugin},
        damage::DamagePlugin,
        movement::{
            orbiting::{orbital_movement, update_orbital_position}, ProjectilePlugin
        },
    },
    mobs::{
        boss::{check_for_victory, spawn_boss},
        enemy::{spawn_enemies, update_enemies},
    },
    prestige::save_game_plugin::SaveGamePlugin,
    skills::skills::animate_lightning,
    tools::{damage_tracking::reset_stats, debug::DebugPlugin},
    ui::{
        in_game::{render_stop_watch, update_health_ui, update_xp_bar_and_level},
        level_up_plugin::LevelUpPlugin,
        loss_plugin::LossPlugin,
        pause_game_plugin::{check_if_paused, PauseGamePlugin},
        win_plguin::WinPlugin,
    },
    update_cursor, AppState, GameState,
};
use bevy::{prelude::*, window::WindowResolution};
use bevy_ecs_tilemap::TilemapPlugin;
use bevy_inspector_egui::quick::WorldInspectorPlugin;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        const STATE: AppState = AppState::InGame;
        app.add_plugins(
            DefaultPlugins
                .set(ImagePlugin::default_nearest())
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        resolution: WindowResolution::new(1000., 800.)
                            .with_scale_factor_override(1.0),
                        ..default()
                    }),
                    ..default()
                }),
        )
        .insert_state(GameState::NotStarted)
        .add_plugins((
            TilemapPlugin,
            WorldInspectorPlugin::new(),
            RunningPlugin,
            LevelUpPlugin,
            PauseGamePlugin,
            MapPlugin,
            LossPlugin,
            WinPlugin,
            SaveGamePlugin
        ))
        .add_systems(
            OnEnter(STATE),
            (reset_ingametime, start_game, spawn_player_hero),
        )
        .add_systems(OnExit(STATE), (cleanup::<cleanup::ExitGame>, reset_stats))
        .add_systems(
            Update,
            (
                animate_sprite,
                update_health_ui,
            )
                .run_if(in_state(STATE)),
        );
    }
}

pub fn start_game(mut state: ResMut<NextState<GameState>>) {
    state.set(GameState::Running);
}

pub struct RunningPlugin;

impl Plugin for RunningPlugin {
    fn build(&self, app: &mut App) {
        const STATE: GameState = GameState::Running;
        app.add_plugins(CooldownPlugin)
            .add_plugins((DamagePlugin, DebugPlugin, ProjectilePlugin))
            .add_systems(
                Update,
                (
                    (
                        handle_ingametime,
                        pickup_loot,
                        player_attack_facing_from_mouse,
                        handle_player_death,
                        spawn_enemies,
                        update_enemies,
                        orbital_movement,
                        update_orbital_position,
                    ),
                    (animate_lightning, spawn_boss),
                    (
                        check_for_dead_enemies,
                        xp_orbs_collision,
                        activate_xp_orb_movement,
                        update_xp_bar_and_level,
                        handle_player_xp.before(update_xp_bar_and_level),
                        update_cursor,
                        player_shooting,
                        render_stop_watch,
                        check_if_paused,
                        handle_xp_orb_movement,
                        check_for_victory,
                    ),
                )
                    .run_if(in_state(STATE)),
            );
    }
}
