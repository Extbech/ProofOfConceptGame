use bevy::ecs::{
    message::{Message, MessageReader, MessageWriter},
    system::ResMut
};

use super::stats::Stats;

#[derive(Message)]
pub struct SaveGameStatsEventToFile;

#[derive(Message)]
pub struct SaveGameStatsEventToMemory;

pub fn save_game_stats(
    stats: ResMut<Stats>,
    mut save_event: MessageReader<SaveGameStatsEventToFile>,
) {
    if save_event.read().count() > 0 {
        println!("Saving in game stats to file!");
        stats.save_stats();
    };
}

pub fn save_prestige(mut event: MessageWriter<SaveGameStatsEventToFile>) {
    println!("Sending Event yo!");
    event.write(SaveGameStatsEventToFile);
}
