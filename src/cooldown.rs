use std::ops::DerefMut;

use bevy::prelude::*;

#[derive(Resource, Component, Clone, Default)]
pub struct Cooldown {
    timer: f32,
    waiting: bool
}

impl Cooldown {
    /// Resets the cooldown with `tot` waiting time per period and returns the number of periods elapsed
    pub fn reset(&mut self, tot: f32) -> u32 {
        let mut count = 0;
        assert!(0. < tot);
        while self.timer <= 0. {
            *&mut self.timer += tot;
            count += 1;
        }
        self.waiting = false;
        count
    }

    /// Sets the cooldown to the waiting status, where it clamps the timer to 0 when ticked
    pub fn wait(&mut self) {
        self.waiting = true;
    }

    /// Reduce the cooldown. Clamp it if waiting and negative
    pub fn tick(&mut self, time: &Time) {
        if 0. < self.timer || !self.waiting {
            self.timer -= time.delta_seconds();
        }
        if self.waiting && self.timer < 0. {
            self.timer = 0.;
        }
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