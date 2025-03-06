use bevy::ecs::component::Component;

/// Component for enemy entites.
#[derive(Component)]
pub struct Enemy;

/// Component for simple mobs.
#[derive(Component)]
pub struct Mob;

/// Component for elite mobs.
#[derive(Component)]
pub struct Elite;

/// Component for boss mobs.
#[derive(Component)]
pub struct Boss;
