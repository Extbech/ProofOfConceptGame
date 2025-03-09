use std::fs;

use bevy::ecs::system::Resource;
use test_game::SAVE_FILE;

#[derive(Resource, serde::Serialize, serde::Deserialize, Clone, Copy)]
pub struct Stats {
    pub damage_increase: f32,
}

impl Default for Stats {
    fn default() -> Self {
        Self {
            damage_increase: 1.,
        }
    }
}

impl Stats {
    pub fn increase_damage(&mut self, amount: f32) {
        self.damage_increase += amount;
    }

    pub fn save_stats(&self) {
        let json = serde_json::to_string(&self).expect("Failed to serialize stats");
        let _ = fs::create_dir("save");
        fs::write(SAVE_FILE, json).expect("Failed to write to file");
    }

    pub fn get_save() -> Option<Stats> {
        if let Some(json_str) = fs::read_to_string(SAVE_FILE).ok() {
            if let Some(json) = serde_json::from_str(&json_str).ok() {
                return json;
            }
        }
        None
    }
}
