use std::time::Duration;

use bevy::color::palettes::css;
use bevy::{prelude::*, platform::collections::HashMap};
use test_game::PROJECTILES_Z;

use crate::characters::player::{AttackCooldown, MaxAttackCooldown};
use crate::prestige::stats::Stats;
use crate::sound::events::{PlaySoundEffectEvent, PlayerSound, SoundEffectKind};
use crate::tools::damage_tracking::{DamageTracker, DamageTrackerKind};
use crate::{
    characters::player::{Player, Range, Vulnerability},
    mechanics::cooldown::Cooldown,
    mechanics::movement::ShouldRotate,
    mobs::enemy::Enemy,
    Heading, MovementSpeed,
};
use crate::{GameRng, GameState};

use super::movement::projectile;

#[derive(Component, Deref, DerefMut, Clone, Copy)]
pub struct BaseDamage(pub u32);

#[derive(Component, Deref, DerefMut, Clone, Copy)]
pub struct Damage(pub u32);

#[derive(Clone, Copy)]
pub struct Circle {
    pub radius: f32,
}

#[derive(Clone, Copy)]
pub struct Cone {
    pub mid_angle: Vec2,
    pub angular_width: f32,
}

#[derive(Component, Clone, Copy)]
pub enum DealDamageHitbox {
    Circle(Circle),
    Cone(Cone),
    Global,
}

#[derive(Component, Clone, Copy)]
pub struct TakeDamageHitbox(pub Circle);

#[derive(Component, Deref, DerefMut)]
pub struct Health(pub u32);

pub struct DamagePlugin;

impl Plugin for DamagePlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<PlayerDamageEvent>().add_systems(
            Update,
            (
                handle_damager_with_hitlist,
                handle_damager_with_per_entity_cooldown,
                handle_damager_with_global_hit_cooldown,
                tick_entity_hit_cooldown,
                handle_damage_to_player,
                display_player_damage,
                damage_multiplier,
            )
                .run_if(in_state(GameState::Running)),
        );
    }
}

#[derive(Message)]
struct PlayerDamageEvent {
    pos: Vec2,
    damage: Damage,
}

fn display_player_damage(
    mut commands: Commands,
    mut dmg_event: MessageReader<PlayerDamageEvent>,
    asset_server: Res<AssetServer>,
    mut rng: ResMut<GameRng>,
) {
    for PlayerDamageEvent { pos, damage } in dmg_event.read() {
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
                    font_size: 10.0,
                    font: asset_server.load("font/pixel-font.ttf"),
                    ..default()
                },
                TextColor(css::WHITE.into()),
                Transform {
                    translation: (pos + rng.rand_vec(0., 5.)).extend(PROJECTILES_Z),
                    ..default()
                },
            ));
    }
}

fn overlapping(
    hitbox1: DealDamageHitbox,
    pos1: Vec2,
    hitbox2: TakeDamageHitbox,
    pos2: Vec2,
) -> bool {
    let radius2 = hitbox2.0.radius;
    match hitbox1 {
        DealDamageHitbox::Circle(Circle { radius }) => pos1.distance(pos2) <= radius + radius2,
        DealDamageHitbox::Global => true,
        DealDamageHitbox::Cone(Cone {
            mid_angle,
            angular_width,
        }) => {
            let v = pos2 - pos1;
            let v = v.rotate_towards(mid_angle, angular_width);
            if v.distance(Vec2::ZERO) <= radius2 || v.distance(mid_angle) <= radius2 {
                true
            } else {
                let proj = v.dot(mid_angle);
                let proj_ortho = v.dot(mid_angle.rotate(Vec2::new(0., 1.)));
                0. <= proj
                    && proj <= mid_angle.length_squared()
                    && proj_ortho * proj_ortho <= radius2 * radius2 * mid_angle.length_squared()
            }
        }
    }
}

/// Bundle for entity that can do contact damage
pub fn damaging(base_damage: BaseDamage, hitbox: DealDamageHitbox) -> impl Bundle {
    (base_damage, Damage(0), hitbox)
}

/// Damaging entities with a [HitList] can only hit another entity once in their lifetime
#[derive(Component, Deref, DerefMut, Default)]
pub struct HitList(pub Vec<Entity>);

fn handle_damager_with_hitlist(
    mut damage_tracker: ResMut<DamageTracker>,
    mut damager_query: Query<(
        &GlobalTransform,
        &Damage,
        Option<&DamageTrackerKind>,
        &mut HitList,
        &DealDamageHitbox,
    )>,
    mut enemy_query: Query<(&GlobalTransform, &mut Health, &TakeDamageHitbox, Entity), With<Enemy>>,
    mut damage_events: MessageWriter<PlayerDamageEvent>,
) {
    for (projectile_transform, &damage, damage_tracker_kind, mut hitlist, hitbox) in
        damager_query.iter_mut()
    {
        for (enemy_transform, mut health, enemy_hitbox, ent) in enemy_query.iter_mut() {
            if hitlist.contains(&ent) {
                continue;
            }
            if overlapping(
                *hitbox,
                projectile_transform.translation().xy(),
                *enemy_hitbox,
                enemy_transform.translation().xy(),
            ) {
                if let Some(new_health) = health.checked_sub(*damage) {
                    if let Some(damage_tracker_kind) = damage_tracker_kind {
                        damage_tracker.update(*damage_tracker_kind, health.0 - new_health);
                    }
                    damage_events.write(PlayerDamageEvent {
                        pos: enemy_transform.translation().xy(),
                        damage,
                    });
                    health.0 = new_health;
                    hitlist.push(ent);
                } else {
                    health.0 = 0
                }
            }
        }
    }
}

/// Damaging entities with a [EntityHitCooldown] can only hit another entity once in a while
#[derive(Component, Default, Deref, DerefMut)]
pub struct EntityHitCooldown(HashMap<Entity, Cooldown>);

fn tick_entity_hit_cooldown(mut ent_hit: Query<&mut EntityHitCooldown>, time: Res<Time>) {
    for mut cd_hm in &mut ent_hit {
        for cd in cd_hm.values_mut() {
            cd.tick(&time)
        }
    }
}

fn handle_damager_with_per_entity_cooldown(
    mut damage_tracker: ResMut<DamageTracker>,
    mut damager_query: Query<(
        &GlobalTransform,
        &Damage,
        Option<&DamageTrackerKind>,
        &mut EntityHitCooldown,
        &DealDamageHitbox,
    )>,
    mut enemy_query: Query<(&GlobalTransform, &mut Health, &TakeDamageHitbox, Entity), With<Enemy>>,
    mut damage_events: MessageWriter<PlayerDamageEvent>,
) {
    for (projectile_transform, &damage, damage_tracker_kind, mut cd, hitbox) in
        damager_query.iter_mut()
    {
        for (enemy_transform, mut health, enemy_hitbox, ent) in enemy_query.iter_mut() {
            if overlapping(
                *hitbox,
                projectile_transform.translation().xy(),
                *enemy_hitbox,
                enemy_transform.translation().xy(),
            ) {
                const MAXHITCOOLDOWN: f32 = 1.;
                let hit_count = cd
                    .entry(ent)
                    .or_default()
                    .reset(Duration::from_secs_f32(MAXHITCOOLDOWN));
                for _ in 0..hit_count {
                    if let Some(new_health) = health.checked_sub(*damage) {
                        if let Some(damage_tracker_kind) = damage_tracker_kind {
                            damage_tracker.update(*damage_tracker_kind, health.0 - new_health);
                        }
                        damage_events.write(PlayerDamageEvent {
                            pos: enemy_transform.translation().xy(),
                            damage,
                        });
                        health.0 = new_health;
                    } else {
                        health.0 = 0;
                        break;
                    }
                }
            }
        }
    }
}

/// Used for effects that only hit *some* of the enemies in their radius
/// Cannot hit more than once per tick
fn handle_damager_with_global_hit_cooldown(
    mut damage_tracker: ResMut<DamageTracker>,
    mut damager_query: Query<(
        &GlobalTransform,
        &Damage,
        Option<&DamageTrackerKind>,
        &mut AttackCooldown,
        &MaxAttackCooldown,
        &DealDamageHitbox,
    )>,
    mut enemy_query: Query<(&GlobalTransform, &mut Health, &TakeDamageHitbox), With<Enemy>>,
    mut damage_events: MessageWriter<PlayerDamageEvent>,
) {
    for (projectile_transform, &damage, damage_tracker_kind, mut attack_cd, max_cd, hitbox) in
        damager_query.iter_mut()
    {
        if !attack_cd.is_ready(max_cd.0) {
            continue;
        }
        for (enemy_transform, mut health, enemy_hitbox) in enemy_query.iter_mut() {
            if overlapping(
                *hitbox,
                projectile_transform.translation().xy(),
                *enemy_hitbox,
                enemy_transform.translation().xy(),
            ) {
                if health.0 == 0 {
                    continue;
                }
                attack_cd.reset(max_cd.0);
                **health = health.saturating_sub(*damage);
                damage_events.write(PlayerDamageEvent {
                    pos: enemy_transform.translation().xy(),
                    damage,
                });
                if let Some(damage_tracker_kind) = damage_tracker_kind {
                    damage_tracker.update(*damage_tracker_kind, *damage);
                }
                break;
            }
        }
    }
}

/// Enemies can hit a player every tick, but only if the player has not been recently hit
fn handle_damage_to_player(
    enemy_query: Query<(&GlobalTransform, &DealDamageHitbox), With<Enemy>>,
    mut player_query: Query<
        (
            &GlobalTransform,
            &mut Health,
            &mut Vulnerability,
            &TakeDamageHitbox,
            &mut Sprite,
        ),
        With<Player>,
    >,
    mut sound_event: MessageWriter<PlaySoundEffectEvent>,
) {
    let (player_trans, mut player_health, mut vulnerability, player_hitbox, mut sprite) =
        player_query.single_mut();
    let player_pos = player_trans.translation().xy();
    let invuln_timer = Duration::from_secs_f32(2.);
    if vulnerability.is_ready(invuln_timer) {
        sprite.color = sprite.color.with_alpha(1.0);
        for (enemy_trans, enemy_hitbox) in &enemy_query {
            let enemy_pos = enemy_trans.translation().xy();
            if overlapping(*enemy_hitbox, enemy_pos, *player_hitbox, player_pos) {
                **player_health = player_health.saturating_sub(1);
                vulnerability.reset(invuln_timer);
                sound_event.write(PlaySoundEffectEvent(SoundEffectKind::PlayerSound(
                    PlayerSound::PlayerTakeDamage,
                )));
                return;
            }
        }
    } else {
        sprite.color = sprite.color.with_alpha(0.6);
    }
}

fn damage_multiplier(
    mut damage_query: Query<(&BaseDamage, &mut Damage), Without<Enemy>>,
    stats: Res<Stats>,
) {
    for (base_damage, mut damage) in &mut damage_query {
        **damage = (**base_damage as f32 * stats.damage_multiplier.get_multiplier()) as u32;
    }
}
