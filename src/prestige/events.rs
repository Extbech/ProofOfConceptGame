use bevy::ecs::{
    event::{Event, EventReader, EventWriter},
    system::{Query, ResMut},
};

use crate::characters::player::CurrentLevel;

use super::stats::Stats;

#[derive(Event)]
pub struct SaveGameStatsEventToFile;

#[derive(Event)]
pub struct SaveGameStatsEventToMemory;

pub fn save_game_stats_in_memory(
    mut stats: ResMut<Stats>,
    query_player: Query<&CurrentLevel>,
    mut save_event: EventReader<SaveGameStatsEventToMemory>,
) {
    if save_event.read().count() > 0 {
        if let Some(level) = query_player.iter().next() {
            stats.increase_damage(**level as f32 / 100.);
            println!("Saving Game Stats to memory: {}", stats.damage_increase);
        }
    };
}

pub fn save_game_stats(
    stats: ResMut<Stats>,
    mut save_event: EventReader<SaveGameStatsEventToFile>,
) {
    if save_event.read().count() > 0 {
        println!("Saving in game stats to file!");
        stats.save_stats();
    };
}

pub fn save_prestige(mut event: EventWriter<SaveGameStatsEventToFile>) {
    println!("Sending Event yo!");
    event.send(SaveGameStatsEventToFile);
}
