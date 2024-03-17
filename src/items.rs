use std::time::Duration;

use bevy::prelude::*;

use crate::{cooldown::LifeTime, enemy::is_collision, loot::Loot, player::{Damage, Player}, projectiles::{HitList, Radius}};

pub fn spawn_bomb(commands: &mut Commands, pos: Vec2) {
    commands.spawn((Damage(100), LifeTime(Duration::from_secs_f32(1.)), HitList(vec![]), Radius(1000.), Transform {translation: Vec3::new(pos.x, pos.y, 1.), ..default()}));
}

pub fn pickup_loot(mut commands: Commands, query_player: Query<&Transform, With<Player>>, query_loot: Query<(&Transform, &Loot, Entity)>) {
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