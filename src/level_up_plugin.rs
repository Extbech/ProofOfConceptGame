use bevy::{app::Plugin, prelude::*};

use crate::{
    cleanup,
    ui::{handle_selection_cursor, spawn_upgrade_selection_ui},
    AppState, GameState,
};
pub struct LevelUpPlugin;

impl Plugin for LevelUpPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (spawn_upgrade_selection_ui, handle_selection_cursor)
                .run_if(in_state(AppState::InGame))
                .run_if(in_state(GameState::LevelUp)),
        )
        .add_systems(
            OnExit(GameState::LevelUp),
            (cleanup::<cleanup::ExitLevelUpScreen>,),
        );
    }
}
