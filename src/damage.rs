use std::time::Duration;

use bevy::prelude::*;

use crate::{enemy::Enemy, player::{Player, Range, Vulnerability}, projectiles::ProjectileBundle, Heading, MovementSpeed};


#[derive(Component, Deref, DerefMut, Clone, Copy)]
pub struct Damage(pub u32);

#[derive(Component, Deref, DerefMut, Clone, Copy)]
pub struct Radius(pub f32);

#[derive(Bundle)]
pub struct DamagingBundle {
    pub damage: Damage,
    pub radius: Radius,
}

#[derive(Component, Deref, DerefMut)]
pub struct Health(pub u32);

#[derive(Component, Deref, DerefMut)]
pub struct HitList(pub Vec<Entity>);

pub fn handle_enemy_damage_from_projectiles(
    mut damager_query: Query<(&Transform, &Damage, &mut HitList, &Radius)>,
    mut enemy_query: Query<(&Transform, &mut Health, &Radius, Entity), With<Enemy>>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    for (projectile_transform, damage, mut hitlist, radius) in damager_query.iter_mut() {
        for (enemy_transform, mut health, enemy_rad, ent) in enemy_query.iter_mut() {
            if !hitlist.contains(&ent) && is_collision(
                    projectile_transform.translation.xy(),
                    enemy_transform.translation.xy(),
                    **radius,
                    **enemy_rad,
            ) {
                **health = health.saturating_sub(**damage);
                spawn_damage_text(&mut commands, damage, &asset_server, enemy_transform.translation.xy());
                hitlist.push(ent)
            }
        }
    }
}

fn spawn_damage_text(commands: &mut Commands, damage: &Damage, asset_server: &Res<AssetServer>, enemy_pos: Vec2) {
    commands
        .spawn(ProjectileBundle::new(
            Heading::new(Vec2::new(0., 1.)),
            MovementSpeed(20.),
            Range(15.),
        ))
        .insert(Text2dBundle {
            text: Text::from_section(
                format!("{:.1}", **damage),
                TextStyle {
                    font_size: 40.0,
                    color: Color::WHITE,
                    font: asset_server.load("font/pixel-font.ttf"),
                },
            ),
            transform: Transform {
                translation: Vec3::new(
                    enemy_pos.x,
                    enemy_pos.y + 30.,
                    10.,
                ),
                ..default()
            },
            ..default()
        });
}

pub fn handle_enemy_damage_to_player(
    enemy_query: Query<&Transform, With<Enemy>>,
    mut player_query: Query<(&Transform, &mut Health, &mut Vulnerability), With<Player>>,
) {
    let (player_trans, mut player_health, mut vulnerability) = player_query.single_mut();
    let player_pos = player_trans.translation.xy();
    let invuln_timer = Duration::from_secs_f32(5.);
    if vulnerability.is_ready(invuln_timer) {
        for enemy_trans in &enemy_query {
            let enemy_pos = enemy_trans.translation.xy();
            if is_collision(player_pos, enemy_pos, 0., 100.) {
                **player_health = player_health.saturating_sub(1);
                vulnerability.reset(invuln_timer);
                return;
            }
        }
    }
}

pub fn is_collision(obj1: Vec2, obj2: Vec2, obj1_radius: f32, obj2_radius: f32) -> bool {
    let diff = (obj1 - obj2).length();
    if diff < obj1_radius + obj2_radius {
        return true;
    }
    false
}