use std::time::Duration;

use bevy::color::palettes::css;
use bevy::{prelude::*, utils::HashMap};
use test_game::PROJECTILES_Z;

use crate::{
    characters::player::{Player, Range, Vulnerability},
    mechanics::cooldown::Cooldown,
    mechanics::projectiles::ShouldRotate,
    mobs::enemy::Enemy,
    Heading, MovementSpeed,
};

use super::projectiles::projectile;

#[derive(Component, Deref, DerefMut, Clone, Copy)]
pub struct Damage(pub u32);

#[derive(Component, Deref, DerefMut, Clone, Copy)]
pub struct Radius(pub f32);

/// Bundle for entity that can do contact damage
pub fn damaging(damage: Damage, radius: Radius) -> impl Bundle {
    (damage, radius)
}

#[derive(Component, Deref, DerefMut)]
pub struct Health(pub u32);

/// Damaging entities with a [HitList] can only hit another entity once
#[derive(Component, Deref, DerefMut, Default)]
pub struct HitList(pub Vec<Entity>);

pub fn handle_enemy_damage_from_projectiles_with_hitlist(
    mut damager_query: Query<(&GlobalTransform, &Damage, &mut HitList, &Radius)>,
    mut enemy_query: Query<(&GlobalTransform, &mut Health, &Radius, Entity), With<Enemy>>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    for (projectile_transform, damage, mut hitlist, radius) in damager_query.iter_mut() {
        for (enemy_transform, mut health, enemy_rad, ent) in enemy_query.iter_mut() {
            if !hitlist.contains(&ent)
                && is_collision(
                    projectile_transform.translation().xy(),
                    enemy_transform.translation().xy(),
                    **radius,
                    **enemy_rad,
                )
            {
                **health = health.saturating_sub(**damage);
                spawn_damage_text(
                    &mut commands,
                    damage,
                    &asset_server,
                    enemy_transform.translation().xy(),
                );
                hitlist.push(ent)
            }
        }
    }
}

/// Damaging entities with a [EntityHitCooldown] can only hit another entity once in a while
#[derive(Component, Default, Deref, DerefMut)]
pub struct EntityHitCooldown(HashMap<Entity, Cooldown>);

pub fn handle_enemy_damage_from_projectiles_with_entity_hitcooldown(
    mut damager_query: Query<(&GlobalTransform, &Damage, &mut EntityHitCooldown, &Radius)>,
    mut enemy_query: Query<(&GlobalTransform, &mut Health, &Radius, Entity), With<Enemy>>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    const MAXHITCOOLDOWN: f32 = 1.;
    for (projectile_transform, damage, mut hitcd, radius) in damager_query.iter_mut() {
        for (enemy_transform, mut health, enemy_rad, ent) in enemy_query.iter_mut() {
            if is_collision(
                projectile_transform.translation().xy(),
                enemy_transform.translation().xy(),
                **radius,
                **enemy_rad,
            ) {
                let cd = hitcd.entry(ent).or_default();
                for _ in 0..cd.reset(Duration::from_secs_f32(MAXHITCOOLDOWN)) {
                    **health = health.saturating_sub(**damage);
                    spawn_damage_text(
                        &mut commands,
                        damage,
                        &asset_server,
                        enemy_transform.translation().xy(),
                    );
                }
            }
        }
    }
}

pub fn tick_entity_hit_cooldown(mut ent_hit: Query<&mut EntityHitCooldown>, time: Res<Time>) {
    for mut cd_hm in &mut ent_hit {
        for cd in cd_hm.values_mut() {
            cd.tick(&time)
        }
    }
}

pub fn spawn_damage_text(
    commands: &mut Commands,
    damage: &Damage,
    asset_server: &Res<AssetServer>,
    enemy_pos: Vec2,
) {
    commands
        .spawn(projectile(
            Heading::new(Vec2::new(0., 1.)),
            MovementSpeed(20.),
            Range(15.),
            ShouldRotate(false),
        ))
        .insert((
            Text2d::new(format!("{:.1}", **damage)),
            TextFont {
                font_size: 40.0,
                font: asset_server.load("font/pixel-font.ttf"),
                ..default()
            },
            TextColor(css::WHITE.into()),
            Transform {
                translation: Vec3::new(enemy_pos.x, enemy_pos.y + 30., PROJECTILES_Z),
                ..default()
            },
        ));
}

pub fn handle_enemy_damage_to_player(
    enemy_query: Query<(&GlobalTransform, &Radius), With<Enemy>>,
    mut player_query: Query<
        (
            &GlobalTransform,
            &mut Health,
            &mut Vulnerability,
            &Radius,
            &mut Sprite,
        ),
        With<Player>,
    >,
) {
    let (player_trans, mut player_health, mut vulnerability, player_radius, mut sprite) =
        player_query.single_mut();
    let player_pos = player_trans.translation().xy();
    let invuln_timer = Duration::from_secs_f32(2.);
    if vulnerability.is_ready(invuln_timer) {
        sprite.color = sprite.color.with_alpha(1.0);
        for (enemy_trans, enemy_rad) in &enemy_query {
            let enemy_pos = enemy_trans.translation().xy();
            if is_collision(player_pos, enemy_pos, **player_radius, **enemy_rad) {
                **player_health = player_health.saturating_sub(1);
                vulnerability.reset(invuln_timer);
                return;
            }
        }
    } else {
        sprite.color = sprite.color.with_alpha(0.6);
    }
}

pub fn is_collision(obj1: Vec2, obj2: Vec2, obj1_radius: f32, obj2_radius: f32) -> bool {
    let diff = (obj1 - obj2).length();
    if diff < obj1_radius + obj2_radius {
        return true;
    }
    false
}
