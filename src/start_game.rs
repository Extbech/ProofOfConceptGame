use bevy::prelude::*;
use bevy_ecs_tilemap::TilemapPlugin;
use bevy_inspector_egui::quick::WorldInspectorPlugin;

use crate::{
    cooldown,
    enemy::{handle_enemy_collision, spawn_enemies, update_enemies, SpawnCooldown},
    loot::{animate_sprite, check_for_dead_enemies, pick_up_xp_orbs, xp_orbs_collision},
    map,
    player::{
        handle_player_xp, player_movement, player_shooting_mouse_dir, spawn_player_hero,
        sync_player_and_camera_pos, AttackCooldown,
    },
    projectiles::projectile_movement,
    update_cursor, AppState,
};

pub struct GamePlugin<S: States> {
    pub state: S,
}
impl<S: States> Plugin for GamePlugin<S> {
    fn build(&self, app: &mut App) {
        app.add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
            .add_plugins(TilemapPlugin)
            .add_plugins(WorldInspectorPlugin::new())
            .add_systems(
                OnExit(AppState::MainMenu),
                (map::setup_map, spawn_player_hero).run_if(in_state(self.state.clone())),
            )
            .add_systems(
                Update,
                (
                    sync_player_and_camera_pos,
                    projectile_movement,
                    cooldown::tick_cooldown::<AttackCooldown>,
                    cooldown::tick_cooldown_res::<SpawnCooldown>,
                    spawn_enemies,
                    handle_enemy_collision,
                    update_enemies,
                    check_for_dead_enemies,
                    animate_sprite,
                    xp_orbs_collision,
                    pick_up_xp_orbs,
                    handle_player_xp,
                    update_cursor,
                    player_movement,
                    player_shooting_mouse_dir,
                )
                    .run_if(in_state(self.state.clone())),
            );
    }
}
