use bevy::prelude::*;
use std::{collections::HashMap, time::Duration};

use crate::{
    cooldown::LifeTime,
    enemy::is_collision,
    loot::Loot,
    player::{Damage, Player},
    projectiles::{HitList, Radius},
};

pub fn spawn_bomb(commands: &mut Commands, pos: Vec2) {
    commands.spawn((
        Damage(100),
        LifeTime(Duration::from_secs_f32(1.)),
        HitList(vec![]),
        Radius(1000.),
        Transform {
            translation: Vec3::new(pos.x, pos.y, 1.),
            ..default()
        },
    ));
}

pub fn pickup_loot(
    mut commands: Commands,
    query_player: Query<&Transform, With<Player>>,
    query_loot: Query<(&Transform, &Loot, Entity)>,
) {
    let player_trans = query_player.single();
    let player_pos = player_trans.translation.xy();
    for (loot_trans, _loot, ent) in &query_loot {
        let loot_position = loot_trans.translation.xy();
        if is_collision(player_pos, loot_position, 100., 0.) {
            spawn_bomb(&mut commands, loot_position);
            commands.entity(ent).despawn_recursive();
        }
    }
}
#[derive(Component, PartialEq, Eq, Hash, Copy, Clone)]
pub enum ItemType {
    PassiveDamageIncrease,
    PassiveMovementSpeedIncrease,
    PassivePickUpRadiusIncrease,
}
#[derive(Component)]
pub struct PassiveDamageIncrease(pub u8);

#[derive(Component)]
pub struct PassiveMovementSpeedIncrease(pub u8);

#[derive(Component)]
pub struct PassivePickUpRadiusIncrease(pub u8);

#[derive(Resource, Deref)]
pub struct ItemTooltips(pub [(ItemType, &'static str); 3]);

impl Default for ItemTooltips {
    fn default() -> Self {
        ItemTooltips([
            (
                ItemType::PassiveDamageIncrease,
                "Increase All Damage Done By 10%.",
            ),
            (
                ItemType::PassiveMovementSpeedIncrease,
                "Increase Movement Speed By 10%.",
            ),
            (
                ItemType::PassivePickUpRadiusIncrease,
                "Increase Pickup Radius By 10%.",
            ),
        ])
    }
}
