use bevy::{app::Plugin, prelude::*};

use crate::{cleanup, ui::render_pause_options_node, GameState};

pub struct PauseGamePlugin;

impl Plugin for PauseGamePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (render_pause_options_node).run_if(in_state(GameState::Paused)),
        )
        .add_systems(
            OnExit(GameState::Paused),
            (cleanup::<cleanup::ExitPauseScreen>,),
        );
    }
}

pub fn check_if_paused(
    mut game_state: ResMut<NextState<GameState>>,
    keys: Res<ButtonInput<KeyCode>>,
) {
    if keys.pressed(KeyCode::Escape) {
        game_state.set(GameState::Paused);
    }
}
