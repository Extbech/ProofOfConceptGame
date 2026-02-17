use crate::{
    characters::player::{
        handle_player_death, handle_player_xp, player_attack_facing_from_mouse, player_shooting,
        spawn_player_hero,
    },
    cleanup,
    loot::loot_plugin::LootPlugin,
    map::map_plugin::MapPlugin,
    mechanics::{
        cooldown::{handle_ingametime, reset_ingametime, CooldownPlugin},
        damage::DamagePlugin,
        movement::{
            orbiting::{orbital_movement, update_orbital_position},
            ProjectilePlugin,
        },
    },
    mobs::{boss::reset_boss_spawn, MobPlugin},
    prestige::save_game_plugin::SaveGamePlugin,
    skills::SkillsPlugin,
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
use bevy_inspector_egui::{bevy_egui::EguiPlugin, quick::WorldInspectorPlugin};

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        const STATE: AppState = AppState::InGame;
        app.add_plugins(
            DefaultPlugins
                .set(ImagePlugin::default_nearest())
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        resolution: WindowResolution::new(1000, 800)
                            .with_scale_factor_override(1.0),
                        ..default()
                    }),
                    ..default()
                }),
        )
        .insert_state(GameState::NotStarted)
        .add_plugins((
            TilemapPlugin,
            EguiPlugin::default(),
            WorldInspectorPlugin::new(),
            RunningPlugin,
            LootPlugin,
            LevelUpPlugin,
            PauseGamePlugin,
            MapPlugin,
            LossPlugin,
            WinPlugin,
            SaveGamePlugin,
            MobPlugin,
        ))
        .add_systems(
            OnEnter(STATE),
            (reset_ingametime, start_game, spawn_player_hero),
        )
        .add_systems(
            OnExit(STATE),
            (cleanup::<cleanup::ExitGame>, reset_stats, reset_boss_spawn),
        )
        .add_systems(Update, (update_health_ui).run_if(in_state(STATE)));
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
            .add_plugins((DamagePlugin, DebugPlugin, ProjectilePlugin, SkillsPlugin))
            .add_systems(
                Update,
                ((
                    handle_ingametime,
                    player_attack_facing_from_mouse,
                    handle_player_death,
                    orbital_movement,
                    update_orbital_position,
                    update_xp_bar_and_level,
                    handle_player_xp.before(update_xp_bar_and_level),
                    update_cursor,
                    player_shooting,
                    render_stop_watch,
                    check_if_paused,
                ),)
                    .run_if(in_state(STATE)),
            );
    }
}
