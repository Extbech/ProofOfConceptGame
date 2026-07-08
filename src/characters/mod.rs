use bevy::prelude::*;
use bosses::BossesPlugin;

use crate::{
    characters::{components::Stage, systems::player::spawn_player_hero},
    AppState, GameState,
};
pub mod bosses;
pub mod components;
pub mod systems;

pub struct MobPlugin;

impl Plugin for MobPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(BossesPlugin)
            .add_systems(
                Update,
                (
                    systems::mobs::spawn_enemies,
                    systems::mobs::update_enemies,
                    systems::mobs::update_enemy_spawn_rate,
                )
                    .run_if(in_state(GameState::Running)),
            )
            .add_systems(
                Update,
                (systems::mobs::handle_stages_wizard).run_if(in_state(Stage::Start)),
            );
    }
}

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::InGame), spawn_player_hero)
            .add_systems(
                Update,
                (
                    systems::player::player_attack_facing_from_mouse,
                    systems::player::handle_player_death,
                    systems::player::handle_player_xp,
                    systems::player::player_shooting,
                )
                    .run_if(in_state(GameState::Running)),
            );
    }
}
