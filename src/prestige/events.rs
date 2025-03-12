use bevy::ecs::{
    event::{Event, EventReader, EventWriter},
    system::ResMut,
};

use super::stats::Stats;

#[derive(Event)]
pub struct SaveGameStatsEventToFile;

#[derive(Event)]
pub struct SaveGameStatsEventToMemory;

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
