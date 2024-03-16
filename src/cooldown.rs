use std::{ops::DerefMut, time::Duration};

use bevy::prelude::*;


/// Struct for events that have some minimal waiting period between occurences.
/// The cooldown timer can be paused and scales to any length of cooldown, even short ones.
/// For short cooldowns the output of [Cooldown::reset] should be used to trigger multiple occurences.
/// It also supports a waiting state for when additional conditions to be met to trigger the event.
#[derive(Resource, Component, Clone, Default)]
pub struct Cooldown {
    timer: Duration,
    waiting: bool,
    paused: bool
}

impl Cooldown {
    /// Resets the cooldown with `period_length` waiting time per period and returns the number of periods elapsed.
    /// Also resets the waiting status of the timer.
    /// When waiting additional conditions only one cooldown period can pass.
    pub fn reset(&mut self, period_length: Duration) -> u32 {
        let mut count = 0;
        assert!(!period_length.is_zero());
        while period_length <= self.timer {
            *&mut self.timer -= period_length;
            count += 1;
        }
        if self.waiting && 0 < count {
            self.timer = default();
            self.waiting = false;
            1
        } else {
            count
        }
    }

    /// Same as [Cooldown::reset], except it sets the status to waiting.
    pub fn reset_and_wait(&mut self, period_length: Duration) -> u32 {
        let mut count = 0;
        assert!(!period_length.is_zero());
        while period_length <= self.timer {
            *&mut self.timer -= period_length;
            count += 1;
        }
        if self.waiting && 0 < count {
            1
        } else {
            self.wait();
            count
        }
    }

    /// Sets the cooldown to the waiting status, where the timer does not overflow.
    pub fn wait(&mut self) {
        self.waiting = true;
    }

    /// Reduce the cooldown unless paused.
    pub fn tick(&mut self, time: &Time) {
        if !self.paused {
            self.timer += time.delta();
        }
    }

    pub fn pause(&mut self) {
        self.paused = true;
    }

    pub fn unpause(&mut self) {
        self.paused = false;
    }
}

pub fn tick_cooldown<CD: DerefMut<Target = Cooldown> + Component>(time: Res<Time>, mut q: Query<&mut CD>) {
    for mut cd in &mut q {
        cd.tick(&time);
    }
}

pub fn tick_cooldown_res<CD: DerefMut<Target = Cooldown> + Resource>(time: Res<Time>, mut cd: ResMut<CD>) {
    cd.tick(&time);
}