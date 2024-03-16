use crate::{
    cleanup,
    enemy::{is_collision, Enemy, Health},
    player::{CurrentXP, PickUpRadius, Player},
    GameRng, MovementSpeed,
};
use bevy::prelude::*;
use rand::prelude::*;

#[derive(Component, Deref, Clone, Copy)]
pub struct XP(f32);

#[derive(Bundle)]
pub struct XPBundle {
    cleanup: cleanup::ExitGame,
    sprite: SpriteSheetBundle,
    animation_timer: AnimationTimer,
    animation_indices: AnimationIndices,
    xp: XP,
    speed: MovementSpeed,
}
impl XPBundle {
    pub fn new(
        sprite: SpriteSheetBundle,
        animation_timer: AnimationTimer,
        animation_indices: AnimationIndices,
        xp: f32,
        speed: MovementSpeed,
    ) -> Self {
        XPBundle {
            cleanup: cleanup::ExitGame,
            sprite,
            xp: XP(xp),
            animation_timer,
            animation_indices,
            speed,
        }
    }
}

#[derive(Component)]
pub struct Loot;

#[derive(Bundle)]
pub struct LootBundle {
    cleanup: cleanup::ExitGame,
    marker: Loot,
    sprite: SpriteBundle,
}
impl LootBundle {
    pub fn new(sprite: SpriteBundle) -> Self {
        LootBundle {
            cleanup: cleanup::ExitGame,
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

/// Spawns health globes
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
/// Responsible for spawning the xp orbs and setting up the animation sequence.
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
        MovementSpeed(500.0),
    ));
}
/// Checks for dead enemies and will spawn loot accordingly.
pub fn check_for_dead_enemies(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    query: Query<(&Transform, Entity, &Health), With<Enemy>>,
    mut rng: ResMut<GameRng>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    for (transform, entity, health) in query.iter() {
        if **health <= 0 {
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
/// Handles the animation sequence for the xp orbs.
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
/// Handles the event where the xp orb is close enough to the player to be consumed.
pub fn xp_orbs_collision(
    mut commands: Commands,
    mut player_query: Query<(&Transform, &mut CurrentXP), With<Player>>,
    mut xp_query: Query<(&Transform, &XP, Entity), With<XP>>,
) {
    let (player_transform, mut current_xp) = player_query.single_mut();
    for (xp_transform, xp, entity) in xp_query.iter_mut() {
        if is_collision(
            player_transform.translation.xy(),
            xp_transform.translation.xy(),
            10.0,
            0.0,
        ) {
            **current_xp += **xp;
            commands.entity(entity).despawn();
            // TODO: play sound effect for xp pickup may add good game feel or it might be annoying (?)
        }
    }
}
/// Causes xp orb entities to start moving towards the player if they are within pick up radius.
/// **NB** Make sure xp_orb movement speed is greater than the players, if not theoretically the player can always outrun the orbs.
pub fn pick_up_xp_orbs(
    mut player_query: Query<(&Transform, &PickUpRadius), With<Player>>,
    mut xp_query: Query<(&mut Transform, &MovementSpeed), (With<XP>, Without<Player>)>,
    time: Res<Time>,
) {
    let (player_trasnform, pick_up_radius) = player_query.single_mut();
    for (mut xp_transform, speed) in xp_query.iter_mut() {
        if is_collision(
            player_trasnform.translation.xy(),
            xp_transform.translation.xy(),
            **pick_up_radius,
            0.0,
        ) {
            let xp_pos = xp_transform.translation.xy();
            let player_pos = player_trasnform.translation.xy();
            xp_transform.translation = (xp_pos
                - (xp_pos - player_pos).normalize_or_zero() * time.delta_seconds() * **speed)
                .extend(xp_transform.translation.z);
        }
    }
}
