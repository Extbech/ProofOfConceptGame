use std::{cmp::Reverse, vec};

use bevy::ecs::{
    component::Component,
    system::{ResMut, Resource},
};

#[derive(Component, Clone, Copy)]
pub enum DamageTrackerKind {
    PrimaryAttack,
    OrbJutsu,
    Lightning,
}
#[derive(Clone, Copy)]
pub struct DamageTrack {
    pub spell: &'static str,
    pub amount: u32,
}

#[derive(Resource, Clone, Copy)]
pub struct DamageTracker {
    primary_attack: DamageTrack,
    orb_justu: DamageTrack,
    lightning_strike: DamageTrack,
}
impl Default for DamageTracker {
    fn default() -> Self {
        DamageTracker {
            primary_attack: DamageTrack {
                spell: "Primary Attack",
                amount: 0,
            },
            orb_justu: DamageTrack {
                spell: "Orb Jutsu",
                amount: 0,
            },
            lightning_strike: DamageTrack {
                spell: "Lightning",
                amount: 0,
            },
        }
    }
}

impl DamageTracker {
    pub fn update(&mut self, kind: DamageTrackerKind, damage: u32) {
        match kind {
            DamageTrackerKind::PrimaryAttack => self.primary_attack.amount += damage,
            DamageTrackerKind::OrbJutsu => self.orb_justu.amount += damage,
            DamageTrackerKind::Lightning => self.lightning_strike.amount += damage,
        }
    }

    pub fn get_total_damage(self) -> u32 {
        self.primary_attack.amount + self.orb_justu.amount + self.lightning_strike.amount
    }

    pub fn get_sorted_by_damage(self) -> Vec<DamageTrack> {
        let mut damage_tracking_sorted: Vec<DamageTrack> =
            vec![self.primary_attack, self.orb_justu, self.lightning_strike]
                .into_iter()
                .filter(|x| x.amount > 0)
                .collect::<Vec<DamageTrack>>();

        damage_tracking_sorted.sort_by_key(|k| Reverse(k.amount));

        return damage_tracking_sorted;
    }

    pub fn reset(&mut self) {
        self.primary_attack.amount = 0;
        self.orb_justu.amount = 0;
        self.lightning_strike.amount = 0;
    }
}

pub fn reset_stats(mut damage_tracker: ResMut<DamageTracker>) {
    damage_tracker.reset();
}
