use bevy::prelude::*;
use bevy_ecs_tilemap::TilemapPlugin;
use bevy_inspector_egui::quick::WorldInspectorPlugin;

use crate::{
    cleanup,
    cooldown::CooldownPlugin,
    enemy::{
        handle_enemy_damage_from_projectiles, handle_enemy_damage_to_player, spawn_enemies,
        update_enemies,
    },
    items::pickup_loot,
    level_up_plugin::LevelUpPlugin,
    loot::{animate_sprite, check_for_dead_enemies, pick_up_xp_orbs, xp_orbs_collision},
    map,
    player::{
        handle_player_death, handle_player_xp, player_attack_facing_from_mouse, player_movement,
        player_shooting, spawn_player_hero, sync_player_and_camera_pos,
    },
    projectiles::speed_to_movement,
    ui::{spawn_health_ui, update_health_ui, update_xp_bar_and_level},
    update_cursor, AppState, GameState,
};

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        const STATE: AppState = AppState::InGame;
        app.insert_state(GameState::Paused)
            .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
            .add_plugins(TilemapPlugin)
            .add_plugins(WorldInspectorPlugin::new())
            .add_plugins(RunningPlugin)
            .add_plugins(LevelUpPlugin)
            .add_systems(
                OnEnter(STATE),
                (
                    unpause,
                    map::setup_map,
                    spawn_player_hero,
                    spawn_health_ui.after(spawn_player_hero),
                ),
            )
            .add_systems(OnExit(STATE), (cleanup::<cleanup::ExitGame>,))
            .add_systems(
                Update,
                (
                    animate_sprite,
                    update_health_ui.after(sync_player_and_camera_pos),
                )
                    .run_if(in_state(STATE)),
            );
    }
}

pub fn unpause(mut state: ResMut<NextState<GameState>>) {
    state.0 = Some(GameState::Running);
}

pub struct RunningPlugin;

impl Plugin for RunningPlugin {
    fn build(&self, app: &mut App) {
        const STATE: GameState = GameState::Running;
        app.add_plugins(CooldownPlugin).add_systems(
            Update,
            (
                pickup_loot,
                player_attack_facing_from_mouse,
                handle_player_death,
                handle_enemy_damage_to_player,
                sync_player_and_camera_pos,
                speed_to_movement,
                spawn_enemies,
                handle_enemy_damage_from_projectiles,
                update_enemies,
                check_for_dead_enemies,
                xp_orbs_collision,
                pick_up_xp_orbs,
                update_xp_bar_and_level,
                handle_player_xp.before(update_xp_bar_and_level),
                update_cursor,
                player_movement,
                player_shooting,
            )
                .run_if(in_state(STATE)),
        );
    }
}
