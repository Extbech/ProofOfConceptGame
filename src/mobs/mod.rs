use bevy::prelude::*;
use boss::BossPlugin;
use enemy::{spawn_enemies, update_enemies};

use crate::{GameState, mobs::enemy::update_enemy_spawn_rate};

pub mod boss;
pub mod enemy;
pub struct MobPlugin;

impl Plugin for MobPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(BossPlugin).add_systems(
            Update,
            (spawn_enemies, update_enemies, update_enemy_spawn_rate).run_if(in_state(GameState::Running)),
        );
    }
}
