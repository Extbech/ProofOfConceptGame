use bevy::{
    app::{App, Plugin, Update},
    state::state::OnExit,
};

use crate::AppState;

use super::events::{
    save_game_stats, save_game_stats_in_memory, save_prestige, SaveGameStatsEventToFile,
    SaveGameStatsEventToMemory,
};

pub struct SaveGamePlguin;

impl Plugin for SaveGamePlguin {
    fn build(&self, app: &mut App) {
        app.add_event::<SaveGameStatsEventToFile>()
            .add_event::<SaveGameStatsEventToMemory>()
            .add_systems(Update, (save_game_stats, save_game_stats_in_memory))
            .add_systems(OnExit(AppState::InGame), (save_prestige,));
    }
}
