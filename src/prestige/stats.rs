use std::fs;

use bevy::ecs::{component::Component, system::Resource};
use test_game::SAVE_FILE;

#[derive(Component)]
pub enum UpgradeOptions {
    MaximumHealth,
    HealthRegen,
    DamageMultiplier,
}

pub struct UpgradeOptionsTooltip {
    kind: UpgradeOptions,
    description: &'static str,
}

#[derive(serde::Serialize, serde::Deserialize, Clone, Copy)]
pub struct DamageMultiplierStats {
    pub price: u32,
    pub amount: f32,
    pub tier: u32,
    pub max_tier: u32,
}

#[derive(serde::Serialize, serde::Deserialize, Clone, Copy)]
pub struct MaximumHealthStats {
    pub price: u32,
    pub amount: u32,
    pub tier: u32,
    pub max_tier: u32,
}

#[derive(serde::Serialize, serde::Deserialize, Clone, Copy)]
pub struct HealthRegenStats {
    pub price: u32,
    pub amount: f32,
    pub tier: u32,
    pub max_tier: u32,
}

#[derive(Resource, serde::Serialize, serde::Deserialize, Clone, Copy)]
pub struct Stats {
    pub coins: u32,
    pub damage_multiplier: DamageMultiplierStats,
    pub maximum_health: MaximumHealthStats,
    pub health_regen: HealthRegenStats,
}

impl Default for Stats {
    fn default() -> Self {
        Self {
            coins: 0,
            damage_multiplier: DamageMultiplierStats {
                price: 10,
                amount: 1.,
                tier: 1,
                max_tier: 5,
            },
            maximum_health: MaximumHealthStats {
                price: 10,
                amount: 2,
                tier: 1,
                max_tier: 5,
            },
            health_regen: HealthRegenStats {
                price: 10,
                amount: 120.,
                tier: 1,
                max_tier: 5,
            },
        }
    }
}

impl Stats {
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

    pub fn update_coins_amount(&mut self, amount: u32) {
        self.coins += amount;
    }

    fn uprade_max_health(&mut self) {
        if self.coins >= self.maximum_health.price
            && self.maximum_health.tier < self.maximum_health.max_tier
        {
            self.coins -= self.maximum_health.price;
            self.maximum_health.amount += 1;
            self.maximum_health.tier += 1;
            self.maximum_health.price *= 2;
        }
    }

    fn uprade_health_regen(&mut self) {
        if self.coins >= self.health_regen.price
            && self.health_regen.tier < self.health_regen.max_tier
        {
            self.coins -= self.health_regen.price;
            self.health_regen.amount += 60.;
            self.health_regen.tier += 1;
            self.maximum_health.tier += 1;
            self.maximum_health.price *= 2;
        }
    }

    fn uprade_damage_multiplier(&mut self) {
        if self.coins >= self.damage_multiplier.price
            && self.damage_multiplier.tier < self.damage_multiplier.max_tier
        {
            self.coins -= self.damage_multiplier.price;
            self.damage_multiplier.amount += 0.1;
            self.damage_multiplier.tier += 1;
            self.maximum_health.tier += 1;
            self.maximum_health.price *= 2;
        }
    }

    pub fn upgrade(&mut self, upgrade_options: UpgradeOptions) {
        match upgrade_options {
            UpgradeOptions::MaximumHealth => {
                self.uprade_max_health();
            }
            UpgradeOptions::HealthRegen => {
                self.uprade_health_regen();
            }
            UpgradeOptions::DamageMultiplier => {
                self.uprade_damage_multiplier();
            }
        }
    }

    pub fn get_upgrade_info(self) {}
}
