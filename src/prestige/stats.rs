use std::{fs, time::Duration};

use bevy::ecs::{component::Component, system::Resource};
use test_game::SAVE_FILE;

#[derive(Component, Clone, Copy)]
pub enum UpgradeOptions {
    MaximumHealth,
    HealthRegen,
    DamageMultiplier,
}

pub trait PrestigeTier: Sized {
    const MAX_TIER: u32;

    fn price(&self) -> u32;

    fn description(&self) -> String;

    fn next(&self) -> Option<Self>;
}

#[derive(serde::Serialize, serde::Deserialize, Clone, Copy)]
pub struct DamageMultiplierTier(u32);

impl DamageMultiplierTier {
    pub fn get_multiplier(&self) -> f32 {
        1. + self.0 as f32 * 0.1
    }
}

impl PrestigeTier for DamageMultiplierTier {
    const MAX_TIER: u32 = 5;

    fn price(&self) -> u32 {
        10 * self.0
    }

    fn description(&self) -> String {
        match self.next() {
            Some(next) => {
                format!(
                    "Increase all damage by: {:.1}% (+{:.1}%),  Tier {}/{}",
                    (self.get_multiplier() - 1.) * 100.,
                    (next.get_multiplier() - self.get_multiplier()) * 100.,
                    self.0,
                    Self::MAX_TIER
                )
            }
            None => {
                format!(
                    "Increase all damage by: {:.1}%,  Tier {}/{}",
                    (self.get_multiplier() - 1.) * 100.,
                    self.0,
                    self.0
                )
            }
        }
    }

    fn next(&self) -> Option<Self> {
        if self.0 == Self::MAX_TIER {
            return None;
        }
        Some(Self(self.0 + 1))
    }
}

#[derive(serde::Serialize, serde::Deserialize, Clone, Copy)]
pub struct MaximumHealthTier(u32);

impl MaximumHealthTier {
    fn get_increase(&self) -> u32 {
        self.0
    }
}

impl PrestigeTier for MaximumHealthTier {
    const MAX_TIER: u32 = 5;

    fn price(&self) -> u32 {
        10 * self.0
    }

    fn description(&self) -> String {
        match self.next() {
            Some(next) => {
                format!(
                    "Increase maximum health by: {} (+{}),  Tier {}/{}",
                    self.get_increase(),
                    next.get_increase() - self.get_increase(),
                    self.0,
                    Self::MAX_TIER
                )
            }
            None => {
                format!(
                    "Increase maximum health by: {},  Tier {}/{}",
                    self.get_increase(),
                    self.0,
                    self.0
                )
            }
        }
    }
    fn next(&self) -> Option<Self> {
        if self.0 == Self::MAX_TIER {
            return None;
        }
        Some(Self(self.0 + 1))
    }
}

#[derive(serde::Serialize, serde::Deserialize, Clone, Copy)]
pub struct HealthRegenTier(u32);

impl HealthRegenTier {
    fn get_increase(&self) -> Option<Duration> {
        if self.0 > 0 {
            return Some(Duration::from_secs(60 - self.0 as u64 * 5));
        }
        None
    }
}

impl PrestigeTier for HealthRegenTier {
    const MAX_TIER: u32 = 5;

    fn price(&self) -> u32 {
        10 * self.0
    }

    fn description(&self) -> String {
        match self.next() {
            Some(next) => {
                format!(
                    "Regenerate health every: {} seconds ({} seconds),  Tier {}/{}",
                    self.get_increase()
                        .map(|d| d.as_secs().to_string())
                        .unwrap_or("Infinity".to_string()),
                    next.get_increase().unwrap().as_secs(),
                    self.0,
                    Self::MAX_TIER
                )
            }
            None => {
                format!(
                    "Regenerate health every: {} seconds,  Tier {}/{}",
                    self.get_increase()
                        .map(|d| d.as_secs().to_string())
                        .unwrap_or("Infinity".to_string()),
                    self.0,
                    self.0
                )
            }
        }
    }

    fn next(&self) -> Option<Self> {
        if self.0 == Self::MAX_TIER {
            return None;
        }
        Some(Self(self.0 + 1))
    }
}

#[derive(Resource, serde::Serialize, serde::Deserialize, Clone, Copy)]
pub struct Stats {
    pub coins: u32,
    pub damage_multiplier: DamageMultiplierTier,
    pub maximum_health: MaximumHealthTier,
    pub health_regen: HealthRegenTier,
}

impl Default for Stats {
    fn default() -> Self {
        Self {
            coins: 1000,
            damage_multiplier: DamageMultiplierTier(0),
            maximum_health: MaximumHealthTier(0),
            health_regen: HealthRegenTier(0),
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
        let Some(next) = self.maximum_health.next() else {
            return;
        };
        if self.coins >= next.price() {
            self.coins -= next.price();
            self.maximum_health = next;
        }
    }

    fn uprade_health_regen(&mut self) {
        let Some(next) = self.health_regen.next() else {
            return;
        };
        if self.coins >= next.price() {
            self.coins -= next.price();
            self.health_regen = next;
        }
    }

    fn uprade_damage_multiplier(&mut self) {
        let Some(next) = self.damage_multiplier.next() else {
            return;
        };
        if self.coins >= next.price() {
            self.coins -= next.price();
            self.damage_multiplier = next;
        }
    }

    pub fn upgrade(&mut self, upgrade_option: UpgradeOptions) {
        match upgrade_option {
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

    pub fn get_upgrade_info(self, upgrade_option: UpgradeOptions) -> String {
        match upgrade_option {
            UpgradeOptions::MaximumHealth => self.maximum_health.description(),
            UpgradeOptions::HealthRegen => self.health_regen.description(),
            UpgradeOptions::DamageMultiplier => self.damage_multiplier.description(),
        }
    }

    pub fn get_next_price(self, upgrade_option: UpgradeOptions) -> Option<u32> {
        match upgrade_option {
            UpgradeOptions::MaximumHealth => Some(self.maximum_health.next()?.price()),
            UpgradeOptions::HealthRegen => Some(self.health_regen.next()?.price()),
            UpgradeOptions::DamageMultiplier => Some(self.damage_multiplier.next()?.price()),
        }
    }

    pub fn is_upgradeable(self, upgrade_option: UpgradeOptions) -> Option<bool> {
        Some(self.get_next_price(upgrade_option)? <= self.coins)
    }
}
