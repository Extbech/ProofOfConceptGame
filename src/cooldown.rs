use std::ops::DerefMut;

use bevy::prelude::*;

#[derive(Resource, Component, Clone, Default)]
pub struct Cooldown {
    timer: f32,
    waiting: bool
}

impl Cooldown {
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

    pub fn wait(&mut self) {
        self.waiting = true;
    }
}

pub fn tick_cooldown<CD: DerefMut<Target = Cooldown> + Component>(time: Res<Time>, mut q: Query<&mut CD>) {
    for mut cd in &mut q {
        if 0. < cd.timer || !cd.waiting {
            cd.timer -= time.delta_seconds();
        } else if cd.waiting && cd.timer < 0. {
            cd.timer = 0.;
        }
    }
}

pub fn tick_cooldown_res<CD: DerefMut<Target = Cooldown> + Resource>(time: Res<Time>, mut cd: ResMut<CD>) {
    if 0. < cd.timer || !cd.waiting {
        cd.timer -= time.delta_seconds();
    } else if cd.waiting && cd.timer < 0. {
        cd.timer = 0.;
    }
}