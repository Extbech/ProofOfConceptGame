use bevy::prelude::*;
use std::time::Duration;

use crate::{
    cooldown::LifeTime,
    damage::{is_collision, Damage, DamagingBundle, HitList, Radius},
    loot::LootId,
    player::Player,
    projectiles::{AngularVelocity, OrbitalRadius, OrbitingBundle},
};

pub fn spawn_bomb(commands: &mut Commands, pos: Vec2) {
    commands.spawn((
        DamagingBundle {
            damage: Damage(100),
            radius: Radius(1000.),
        },
        LifeTime(Duration::from_secs_f32(1.)),
        HitList(vec![]),
        SpatialBundle {
            transform: Transform {
                translation: Vec3::new(pos.x, pos.y, 1.),
                ..default()
            },
            ..default()
        },
    ));
}

pub fn pickup_loot(
    mut commands: Commands,
    query_player: Query<(&Transform, Entity), With<Player>>,
    query_loot: Query<(&Transform, &LootId, Entity)>,
) {
    let (player_trans, player_entity) = query_player.single();
    let player_pos = player_trans.translation.xy();
    for (loot_trans, loot, ent) in &query_loot {
        let loot_position = loot_trans.translation.xy();
        if is_collision(player_pos, loot_position, 100., 0.) {
            match **loot {
                0 => {
                    spawn_bomb(&mut commands, loot_position);
                }
                1 => {
                    commands.entity(player_entity).with_children(|parent| {
                        parent.spawn((
                            OrbitingBundle {
                                vel: AngularVelocity(3.),
                                radius: OrbitalRadius(200.),
                                ..default()
                            },
                            DamagingBundle {
                                damage: Damage(2),
                                radius: Radius(50.),
                            },
                            HitList(vec![]), // TODO: Remove this and add a timing based system for orbiting damagers instead
                        ));
                    });
                }
                _ => unreachable!("invalid loot id"),
            }
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
