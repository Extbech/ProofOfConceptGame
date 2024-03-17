use bevy::prelude::*;
use bevy_ecs_tilemap::TilemapPlugin;
use bevy_inspector_egui::quick::WorldInspectorPlugin;

use crate::{
    cleanup, cooldown::CooldownPlugin, enemy::{
        handle_enemy_damage_from_projectiles, handle_enemy_damage_to_player, spawn_enemies,
        update_enemies,
    }, items::pickup_loot, loot::{animate_sprite, check_for_dead_enemies, pick_up_xp_orbs, xp_orbs_collision}, map, player::{
        handle_player_death, handle_player_xp, player_attack_facing_from_mouse, player_movement,
        player_shooting, spawn_player_hero, sync_player_and_camera_pos,
    }, projectiles::speed_to_movement, ui::{spawn_health_ui, update_health_ui, update_xp_bar_and_level}, update_cursor, AppState, GameState
};

pub struct GamePlugin<S: States> {
    pub state: S,
}
impl<S: States> Plugin for GamePlugin<S> {
    fn build(&self, app: &mut App) {
        app.insert_state(GameState::Running)
            .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
            .add_plugins(TilemapPlugin)
            .add_plugins(WorldInspectorPlugin::new())
            .add_plugins(CooldownPlugin)
            .add_systems(
                OnEnter(AppState::InGame),
                (
                    map::setup_map,
                    spawn_player_hero,
                    spawn_health_ui.after(spawn_player_hero),
                ),
            )
            .add_systems(OnExit(AppState::InGame), cleanup::<cleanup::ExitGame>)
            .add_systems(
                Update,
                (
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
                        animate_sprite,
                    ),
                    xp_orbs_collision,
                    pick_up_xp_orbs,
                    handle_player_xp,
                    update_cursor,
                    player_movement,
                    player_shooting,
                    update_health_ui,
                    update_xp_bar_and_level,
                    update_health_ui.after(sync_player_and_camera_pos),
                )
                    .run_if(in_state(self.state.clone())),
            );
    }
}
