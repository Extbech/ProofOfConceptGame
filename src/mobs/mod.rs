use bevy::prelude::*;
use boss::{check_for_victory, spawn_boss};
use enemy::{spawn_enemies, update_enemies};

use crate::GameState;

pub mod boss;
pub mod enemy;

pub struct MobPlugin;

impl Plugin for MobPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (
            spawn_enemies,
            spawn_boss,
            update_enemies,
            check_for_victory,
        ).run_if(in_state(GameState::Running)));
    }
}