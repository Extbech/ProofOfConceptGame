use crate::{
    enemy::{is_collision, Enemy, Health},
    player::{CurrentXP, PickUpRadius, Player},
    GameRng,
};
use bevy::prelude::*;
use rand::prelude::*;

#[derive(Component, Deref, Clone, Copy)]
pub struct XP(f32);

#[derive(Bundle)]
pub struct XPBundle {
    sprite: SpriteSheetBundle,
    animation_timer: AnimationTimer,
    animation_indices: AnimationIndices,
    xp: XP,
}
impl XPBundle {
    pub fn new(
        sprite: SpriteSheetBundle,
        animation_timer: AnimationTimer,
        animation_indices: AnimationIndices,
        xp: f32,
    ) -> Self {
        XPBundle {
            sprite,
            xp: XP(xp),
            animation_timer,
            animation_indices,
        }
    }
}

#[derive(Component)]
pub struct Loot;

#[derive(Bundle)]
pub struct LootBundle {
    marker: Loot,
    sprite: SpriteBundle,
}
impl LootBundle {
    pub fn new(sprite: SpriteBundle) -> Self {
        LootBundle {
            marker: Loot,
            sprite,
        }
    }
}
#[derive(Component)]
pub struct AnimationIndices {
    first: usize,
    second: usize,
    third: usize,
    fourth: usize,
}
#[derive(Component, Deref, DerefMut)]
pub struct AnimationTimer(Timer);
/// **GAMBA MODULE:**
/// Fetch user data and check if they have donated $$$
/// increase drop rate by 500%
/// PROFIT ???
pub fn spawn_health_globe(commands: &mut Commands, asset_server: &Res<AssetServer>, pos: Vec3) {
    let loot_texture_handle: Handle<Image> = asset_server.load("loot/Icon36.png");
    commands.spawn(LootBundle::new(SpriteBundle {
        transform: Transform::from_xyz(pos.x - 20.0, pos.y + 10.0, pos.z),
        texture: loot_texture_handle,
        sprite: Sprite {
            custom_size: Some(Vec2::new(40., 40.)),
            ..Default::default()
        },
        ..default()
    }));
}
pub fn spawn_xp_orb(
    commands: &mut Commands,
    asset_server: &Res<AssetServer>,
    texture_atlas_layouts: &mut ResMut<Assets<TextureAtlasLayout>>,
    pos: Vec3,
) {
    let loot_texture_handle: Handle<Image> = asset_server.load("loot/rotating_orbs.png");
    let layout = TextureAtlasLayout::from_grid(Vec2::new(32.0, 32.0), 7, 1, None, None);
    let texture_atlas_layout = texture_atlas_layouts.add(layout);
    let animation_indices = AnimationIndices {
        first: 0,
        second: 1,
        third: 2,
        fourth: 3,
    };
    commands.spawn(XPBundle::new(
        SpriteSheetBundle {
            texture: loot_texture_handle,
            atlas: TextureAtlas {
                layout: texture_atlas_layout,
                index: animation_indices.first,
            },
            transform: Transform::from_xyz(pos.x, pos.y, pos.z),
            sprite: Sprite {
                custom_size: Some(Vec2::new(30., 30.)),
                ..Default::default()
            },
            ..default()
        },
        AnimationTimer(Timer::from_seconds(0.1, TimerMode::Repeating)),
        animation_indices,
        10.0,
    ));
}
pub fn check_for_dead_enemies(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    query: Query<(&Transform, Entity, &Health), With<Enemy>>,
    mut rng: ResMut<GameRng>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    for (transform, entity, health) in query.iter() {
        if **health <= 0.0 {
            commands.entity(entity).despawn();
            // 1/5 -> 20%
            spawn_xp_orb(
                &mut commands,
                &asset_server,
                &mut texture_atlas_layouts,
                transform.translation,
            );
            if rng.gen_range(0u16..10) == 0 as u16 {
                spawn_health_globe(&mut commands, &asset_server, transform.translation);
            }
        }
    }
}
pub fn animate_sprite(
    time: Res<Time>,
    mut query: Query<(&AnimationIndices, &mut AnimationTimer, &mut TextureAtlas)>,
) {
    for (indices, mut timer, mut atlas) in &mut query {
        timer.tick(time.delta());
        if timer.just_finished() {
            match atlas.index {
                0 => atlas.index = indices.second,
                1 => atlas.index = indices.third,
                2 => atlas.index = indices.fourth,
                3 => atlas.index = indices.first,
                _ => unreachable!("Inavlid animation index for this entity."),
            }
        }
    }
}

pub fn pick_up_xp_orbs(
    mut commands: Commands,
    mut player_query: Query<(&Transform, &mut CurrentXP, &PickUpRadius), With<Player>>,
    mut xp_query: Query<(&Transform, &XP, Entity), With<XP>>,
) {
    for (player_transform, mut current_xp, pick_up_radius) in player_query.iter_mut() {
        for (xp_transform, xp, entity) in xp_query.iter_mut() {
            if is_collision(
                player_transform.translation,
                xp_transform.translation,
                **pick_up_radius,
                0.0,
            ) {
                **current_xp += **xp;
                commands.entity(entity).despawn();
                println!("XP: {}", **current_xp);
            }
        }
    }
}
