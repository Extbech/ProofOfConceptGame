use bevy::{
    app::{App, Plugin, Update},
    state::state::OnExit,
};

use crate::AppState;

use super::events::{
    save_game_stats, save_prestige, SaveGameStatsEventToFile, SaveGameStatsEventToMemory,
};

pub struct SaveGamePlugin;

impl Plugin for SaveGamePlugin {
    fn build(&self, app: &mut App) {
        app.add_message::<SaveGameStatsEventToFile>()
            .add_message::<SaveGameStatsEventToMemory>()
            .add_systems(Update, (save_game_stats,))
            .add_systems(OnExit(AppState::InGame), (save_prestige,))
            .add_systems(OnExit(AppState::Upgrade), (save_prestige,));
    }
}
