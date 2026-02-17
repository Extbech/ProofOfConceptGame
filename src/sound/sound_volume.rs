use std::fs;

use bevy::prelude::*;
use test_game::SETTINGS_SAVE_FILE;

#[derive(Resource, serde::Serialize, serde::Deserialize, Clone, Copy)]
pub struct SoundVolume {
    pub music: f32,
    pub sfx: f32,
}

impl Default for SoundVolume {
    fn default() -> Self {
        Self {
            music: 0.5,
            sfx: 0.5,
        }
    }
}

impl SoundVolume {
    pub fn save_stats(&self) {
        let json = serde_json::to_string(&self).expect("Failed to serialize stats");
        let _ = fs::create_dir("save");
        fs::write(SETTINGS_SAVE_FILE, json).expect("Failed to write to file");
    }

    pub fn get_save() -> Option<SoundVolume> {
        if let Ok(json_str) = fs::read_to_string(SETTINGS_SAVE_FILE) {
            if let Ok(json) = serde_json::from_str(&json_str) {
                return Some(json);
            }
        }
        None
    }

    pub fn update_music_volume(&mut self, volume: f32) {
        if volume >= 0.0 && volume <= 100.0 {
            self.music = volume;
        }
    }

    pub fn update_sfx_volume(&mut self, volume: f32) {
        if volume >= 0.0 && volume <= 100.0 {
            self.sfx = volume;
        }
    }
}
