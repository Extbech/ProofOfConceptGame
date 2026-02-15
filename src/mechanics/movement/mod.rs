use crate::{
    characters::player::{Player, Range},
    cleanup,
    mechanics::cooldown::LifeTime,
    GameState, Heading, MovementSpeed, MyGameCamera, SCALE,
};
use bevy::prelude::*;

pub struct ProjectilePlugin;

impl Plugin for ProjectilePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                handle_projectile_rotation,
                player_movement,
                sync_player_and_camera_pos,
                speed_to_movement.before(sync_player_and_camera_pos),
            )
                .run_if(in_state(GameState::Running)),
        );
        app.add_systems(
            PostUpdate,
            sync_player_and_camera_pos.run_if(in_state(GameState::Running)),
        );
    }
}

fn sync_player_and_camera_pos(
    player: Query<&Transform, With<Player>>,
    mut cam: Query<&mut Transform, (With<MyGameCamera>, Without<Player>)>,
) {
    let player = player.single().expect("No player found!");
    let mut cam = cam.single_mut().expect("Err camera!");
    cam.translation.x = player.translation.x;
    cam.translation.y = player.translation.y;
}

fn player_movement(
    keys: Res<ButtonInput<KeyCode>>,
    mut player: Query<(&mut Transform, &mut Heading, &mut Sprite), With<Player>>,
) {
    let (mut player_trans, mut player_dir, mut player_sprite) = player.single_mut().expect("Err");
    let player_position = &mut player_trans.translation;
    let keyboard_dir_x = if keys.pressed(KeyCode::KeyD) { 1. } else { 0. }
        - if keys.pressed(KeyCode::KeyA) { 1. } else { 0. };
    let keyboard_dir_y = if keys.pressed(KeyCode::KeyW) { 1. } else { 0. }
        - if keys.pressed(KeyCode::KeyS) { 1. } else { 0. };
    const BOUND: f32 = 1900.;

    if keyboard_dir_x != 0. {
        if let Some(atlas) = &mut player_sprite.texture_atlas {
            atlas.index = 3;
        }
        player_sprite.flip_x = keyboard_dir_x == -1.;
    } else if keyboard_dir_y == 1. {
        if let Some(atlas) = &mut player_sprite.texture_atlas {
            atlas.index = 2;
        }
    } else if keyboard_dir_y == -1. {
        if let Some(atlas) = &mut player_sprite.texture_atlas {
            atlas.index = 1;
        }
    }
    (player_position.x, player_position.y) = player_position
        .xy()
        .clamp(-Vec2::splat(BOUND), Vec2::splat(BOUND))
        .into();
    *player_dir = Heading::new(Vec2::new(keyboard_dir_x, keyboard_dir_y));
}

#[derive(Component, Deref, DerefMut, Clone, Copy)]
pub struct RemDistance(pub f32);

#[derive(Component, Deref, DerefMut, Clone, Copy)]
pub struct ShouldRotate(pub bool);

#[derive(Component)]
pub struct ProjectileMarker;

pub fn projectile(
    dir: Heading,
    speed: MovementSpeed,
    range: Range,
    should_rotate: ShouldRotate,
) -> impl Bundle {
    (
        cleanup::ExitGame,
        dir,
        speed,
        LifeTime::from_speed_and_range(speed, range),
        ProjectileMarker,
        should_rotate,
    )
}

fn handle_projectile_rotation(
    mut q: Query<(&Heading, &mut Transform, &ShouldRotate), With<ProjectileMarker>>,
) {
    for (dir, mut tran, should_rotate) in &mut q {
        if **should_rotate {
            let angle = dir.y.atan2(dir.x);
            tran.rotation = Quat::from_axis_angle(Vec3::new(0., 0., 1.), angle);
        }
    }
}

fn speed_to_movement(time: Res<Time>, mut q: Query<(&Heading, &mut Transform, &MovementSpeed)>) {
    for (dir, mut tran, &speed) in &mut q {
        let pos = &mut tran.translation;
        (pos.x, pos.y) =
            (Vec2::new(pos.x, pos.y) + *speed * SCALE * time.delta_secs() * dir.v).into();
    }
}

pub mod orbiting;
