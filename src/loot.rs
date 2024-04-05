use crate::damage::Health;
use crate::{
    cleanup,
    damage::is_collision,
    enemy::Enemy,
    player::{CurrentXP, XpPickUpRadius, Player},
    GameRng, MovementSpeed,
};
use bevy::prelude::*;
use rand::prelude::*;
use test_game::LOOT_DROPS_Z;

#[derive(Component, Deref, Clone, Copy)]
pub struct XP(f32);

#[derive(Component, Deref, DerefMut, Clone, Copy)]
pub struct XPActive(pub bool);

#[derive(Component)]
pub struct XPAbsorbItem;

#[derive(Bundle)]
pub struct XPBundle {
    cleanup: cleanup::ExitGame,
    sprite: SpriteSheetBundle,
    animation_timer: AnimationTimer,
    animation_indices: AnimationIndices,
    xp: XP,
    speed: MovementSpeed,
    active: XPActive,
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
            active: XPActive(false),
        }
    }
}

#[derive(Component, Deref, DerefMut)]
pub struct LootId(u32);

#[derive(Bundle)]
pub struct LootBundle {
    cleanup: cleanup::ExitGame,
    id: LootId,
    sprite: SpriteBundle,
}
impl LootBundle {
    pub fn new(sprite: SpriteBundle, id: LootId) -> Self {
        LootBundle {
            cleanup: cleanup::ExitGame,
            id,
            sprite,
        }
    }
}
#[derive(Bundle)]
pub struct LootBundleSheet {
    cleanup: cleanup::ExitGame,
    id: LootId,
    sprite: SpriteSheetBundle,
}
impl LootBundleSheet {
    pub fn new(sprite: SpriteSheetBundle, id: LootId) -> Self {
        LootBundleSheet {
            cleanup: cleanup::ExitGame,
            id,
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

/// Spawns loot.
pub fn try_spawn_loot(
    rng: &mut ResMut<GameRng>,
    commands: &mut Commands,
    asset_server: &Res<AssetServer>,
    pos: Vec3,
) {
    let loot_id = rng.gen_range(0..10);
    let image_path = match loot_id {
        0 => "loot/potion.png",
        1 => "loot/hammeritem.png",
        2 => "loot/magnet.png",
        _ => return,
    };
    let loot_texture_handle: Handle<Image> = asset_server.load(image_path);
    commands.spawn(LootBundle::new(
        SpriteBundle {
            transform: Transform::from_xyz(pos.x - 20.0, pos.y + 10.0, LOOT_DROPS_Z),
            texture: loot_texture_handle,
            ..default()
        },
        LootId(loot_id),
    ));
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
            transform: Transform::from_xyz(pos.x, pos.y, LOOT_DROPS_Z),
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
        if **health == 0 {
            commands.entity(entity).despawn_recursive();
            // 1/5 -> 20%
            spawn_xp_orb(
                &mut commands,
                &asset_server,
                &mut texture_atlas_layouts,
                transform.translation,
            );
            try_spawn_loot(
                &mut rng,
                &mut commands,
                &asset_server,
                transform.translation,
            );
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
            commands.entity(entity).despawn_recursive();
            // TODO: play sound effect for xp pickup may add good game feel or it might be annoying (?)
        }
    }
}
/// Causes xp orb entities to start moving towards the player if they are within pick up radius.
/// **NB** Make sure xp_orb movement speed is greater than the players, if not theoretically the player can always outrun the orbs.
pub fn activate_xp_orb_movement(
    mut player_query: Query<(&Transform, &XpPickUpRadius), With<Player>>,
    mut xp_query: Query<(&Transform, &mut XPActive), (With<XP>, Without<Player>)>,
) {
    let (player_trasnform, pick_up_radius) = player_query.single_mut();
    for (xp_transform, mut active) in xp_query.iter_mut() {
        if is_collision(
            player_trasnform.translation.xy(),
            xp_transform.translation.xy(),
            **pick_up_radius,
            0.0,
        ) {
            **active = true;
        }
    }
}

pub fn handle_xp_orb_movement(
    mut player_query: Query<&Transform, With<Player>>,
    mut xp_query: Query<(&mut Transform, &MovementSpeed, &XPActive), (With<XP>, Without<Player>)>,
    time: Res<Time>,
) {
    let player_trasnform = player_query.single_mut();
    for (mut xp_transform, speed, active) in xp_query.iter_mut() {
        if **active {
            let xp_pos = xp_transform.translation.xy();
            let player_pos = player_trasnform.translation.xy();
            xp_transform.translation = (xp_pos
                - (xp_pos - player_pos).normalize_or_zero() * time.delta_seconds() * **speed)
                .extend(xp_transform.translation.z);
        }
    }
}

pub fn activate_all_xp_orbs(q: &mut Query<&mut XPActive, With<XP>>) {
    for mut active in q {
        **active = true;
    }
}
