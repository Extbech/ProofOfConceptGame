use std::fs;

use bevy::ecs::system::Resource;

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
        fs::write("save/save_file.json", json).expect("Failed to write to file");
    }

    pub fn get_save() -> Option<Stats> {
        if let Some(json_str) = fs::read_to_string("save/save_file.json").ok() {
            if let Some(json) = serde_json::from_str(&json_str).ok() {
                return json;
            }
        }
        None
    }
}
