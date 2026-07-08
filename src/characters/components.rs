use std::time::Duration;

use bevy::{
    ecs::{component::Component, resource::Resource},
    prelude::{Deref, DerefMut},
};

use crate::{
    mechanics::cooldown::{CooldownComponent, CooldownResource},
    Heading,
};

#[derive(Component)]
pub struct Player;

#[derive(Component, Deref, Clone, Copy)]
pub struct ProjectileSpeed(pub f32);

#[derive(Component, Deref, DerefMut, Clone, Copy)]
pub struct MaxAttackCooldown(pub Duration);

#[derive(Component, Deref, DerefMut, Clone)]
pub struct AttackCooldown(pub CooldownComponent);

#[derive(Component, Deref, DerefMut, Clone, Copy)]
pub struct Range(pub f32);

#[derive(Component, Deref, DerefMut, Clone, Copy)]
pub struct CurrentXP(pub f32);

#[derive(Component, Deref, DerefMut, Clone, Copy)]
pub struct RequiredXP(pub f32);

#[derive(Component, Deref, DerefMut, Clone, Copy)]
pub struct CurrentLevel(pub usize);

#[derive(Component, Deref, Clone, Copy)]
pub struct MaxLevel(pub usize);

#[derive(Component, Deref, DerefMut, Clone, Copy)]
pub struct XpPickUpRadius(pub f32);

#[derive(Component, Deref, DerefMut, Default)]
pub struct Vulnerability(pub CooldownComponent);

#[derive(Component, Deref, DerefMut, Clone, Copy)]
pub struct MaxSpeed(pub f32);

#[derive(Component, Deref, DerefMut)]
pub struct AttackDirection(pub Heading);

#[derive(Component, Deref, DerefMut, Clone, Copy)]
pub struct PlayerDamage(pub u32);

#[derive(Component, Deref, DerefMut)]
pub struct Health(pub u32);

#[derive(Component, Deref, DerefMut, Clone, Copy)]
pub struct MaxHealth(pub u32);

// <-- MOBS -->

#[derive(Resource, Deref, DerefMut)]
pub struct SpawnRate(pub Duration);

#[derive(Resource, Deref, DerefMut)]
pub struct SpawnCooldown(pub CooldownResource);

#[derive(Component)]
pub struct Enemy;

// <-- BOSS -->
use bevy::state::state::SubStates;

use crate::GameState;

#[derive(SubStates, Clone, Debug, PartialEq, Eq, Hash, Default)]
#[source(GameState = GameState::Running)]
pub enum Stage {
    #[default]
    Start,
    Wizard,
}
