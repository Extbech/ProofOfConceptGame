use std::{ops::DerefMut, time::Duration};

use bevy::{ecs::component::Mutable, prelude::*};

use crate::{
    characters::player::{AttackCooldown, Range, Vulnerability},
    mobs::enemy::SpawnCooldown,
    GameState, MovementSpeed,
};

/// Struct for events that have some minimal waiting period between occurences.
/// The cooldown timer can be paused and scales to any length of cooldown, even short ones.
/// For short cooldowns the output of [Cooldown::reset] should be used to trigger multiple occurences.
/// It also supports a waiting state for when additional conditions to be met to trigger the event.
#[derive(Resource, Component, Clone)]
pub struct Cooldown {
    timer: Duration,
    waiting: bool,
}

impl Default for Cooldown {
    /// defaults to being ready for almost any length of cooldown, but only processing one duration
    fn default() -> Self {
        Self {
            timer: Duration::from_secs_f32(10000000.),
            waiting: true,
        }
    }
}

impl Cooldown {
    /// Resets the cooldown without returning any information about what the original value was
    pub fn fill(&mut self) {
        self.timer = Duration::ZERO;
    }

    /// Resets the cooldown with `period_length` waiting time per period and returns the number of periods elapsed.
    /// Also resets the waiting status of the timer.
    /// When waiting additional conditions only one cooldown period can pass.
    pub fn reset(&mut self, period_length: Duration) -> u32 {
        let mut count = 0;
        assert!(!period_length.is_zero());
        while period_length <= self.timer {
            self.timer -= period_length;
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
            self.timer -= period_length;
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
        self.timer += time.delta();
    }

    pub fn is_ready(&self, period_length: Duration) -> bool {
        period_length <= self.timer
    }
}

pub fn tick_cooldown<CD: DerefMut<Target = Cooldown> + Component<Mutability = Mutable>>(
    time: Res<Time>,
    mut q: Query<&mut CD>,
) {
    for mut cd in &mut q {
        cd.tick(&time);
    }
}

pub fn tick_cooldown_res<CD: DerefMut<Target = Cooldown> + Resource>(
    time: Res<Time>,
    mut cd: ResMut<CD>,
) {
    cd.tick(&time);
}

#[derive(Resource, Default)]
pub struct InGameTime(pub Duration);

impl InGameTime {
    pub fn tick(&mut self, by: Duration) {
        self.0 += by;
    }

    pub fn time(&self) -> Duration {
        self.0
    }

    pub fn reset(&mut self) {
        *self = default();
    }
}

pub fn handle_ingametime(time: Res<Time>, mut igt: ResMut<InGameTime>) {
    igt.tick(time.delta())
}

pub fn reset_ingametime(mut igt: ResMut<InGameTime>) {
    igt.reset()
}

#[derive(Component, Deref)]
pub struct LifeTime(pub Duration);

impl LifeTime {
    pub fn from_secs_f32(s: f32) -> Self {
        LifeTime(Duration::from_secs_f32(s))
    }

    pub fn from_speed_and_range(spd: MovementSpeed, rng: Range) -> Self {
        LifeTime::from_secs_f32(*rng / *spd)
    }

    pub fn try_decrease(&mut self, by: Duration) -> bool {
        match self.0.checked_sub(by) {
            Some(dur) => {
                self.0 = dur;
                true
            }
            None => false,
        }
    }
}

pub fn handle_lifetime(
    mut commands: Commands,
    time: Res<Time>,
    mut q: Query<(&mut LifeTime, Entity)>,
) {
    for (mut lt, ent) in &mut q {
        if !lt.try_decrease(time.delta()) {
            commands.entity(ent).despawn();
        }
    }
}

pub struct CooldownPlugin;

impl Plugin for CooldownPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                handle_lifetime,
                tick_cooldown_res::<SpawnCooldown>,
                tick_cooldown::<AttackCooldown>,
                tick_cooldown::<Vulnerability>,
            )
                .run_if(in_state(GameState::Running)),
        );
    }
}