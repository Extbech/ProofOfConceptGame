use bevy::{
    app::{Plugin, Update},
    state::condition::in_state,
};

use crate::GameState;

use super::{
    loot::{check_for_dead_enemies, pickup_loot},
    xp::{activate_xp_orb_movement, animate_sprite, handle_xp_orb_movement, xp_orbs_collision},
};

pub struct LootPlugin;

impl Plugin for LootPlugin {
    fn build(&self, app: &mut bevy::app::App) {
        app.add_systems(
            Update,
            (
                check_for_dead_enemies,
                pickup_loot,
                animate_sprite,
                xp_orbs_collision,
                activate_xp_orb_movement,
                handle_xp_orb_movement,
            )
                .run_if(in_state(GameState::Running)),
        );
    }
}
