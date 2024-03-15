use std::ops::DerefMut;

use bevy::prelude::*;

#[derive(Resource, Component, Clone, Default)]
pub struct Cooldown {
    timer: f32,
}

impl Cooldown {
    pub fn reset(&mut self, tot: f32) -> u32 {
        let mut count = 0;
        assert!(0. < tot);
        while self.timer <= 0. {
            *&mut self.timer += tot;
            count += 1;
        }
        count
    }
}

pub fn tick_cooldown<CD: DerefMut<Target = Cooldown> + Component>(time: Res<Time>, mut q: Query<&mut CD>) {
    for mut cd in &mut q {
        if 0. < cd.timer {
            cd.timer -= time.delta_seconds();
        }
    }
}

pub fn tick_timer<T: DerefMut<Target = Timer> + Component>(time: Res<Time>, mut q: Query<&mut T>) {
    for mut cd in &mut q {
        cd.tick(time.delta());
    }
}

// TODO: Fix this for when we have multiple logic steps per frame step
pub fn tick_timer_res<T: DerefMut<Target = Timer> + Resource>(time: Res<Time>, mut cd: ResMut<T>) {
    cd.tick(time.delta());
}