use bevy::{
    ecs::{bundle::Bundle, system::ResMut},
    transform::components::Transform,
};
use test_game::LOOT_DROPS_Z;

use crate::{
    cleanup,
    prestige::stats::Stats,
    sprites::{Item, SpriteKind},
    MovementSpeed,
};

use super::{loot::LootId, xp::MagnetActive};

pub fn spawn_coin(id: u32, x: f32, y: f32) -> impl Bundle {
    (
        cleanup::ExitGame,
        LootId(id),
        Transform::from_xyz(x, y, LOOT_DROPS_Z),
        SpriteKind::Item(Item::Coin),
        MagnetActive(false),
        MovementSpeed(500.0),
    )
}

pub fn handle_coin_pickup(stats: &mut ResMut<Stats>) {
    stats.update_coins_amount(1);
}
