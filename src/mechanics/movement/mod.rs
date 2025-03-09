use crate::{
    characters::player::{sync_player_and_camera_pos, Range}, cleanup, mechanics::cooldown::LifeTime, GameState, Heading, MovementSpeed, SCALE
};
use bevy::prelude::*;

pub struct ProjectilePlugin;

impl Plugin for ProjectilePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (
            handle_projectile_rotation,
            speed_to_movement.before(sync_player_and_camera_pos)
        ).run_if(in_state(GameState::Running)));
    }
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

fn speed_to_movement(
    time: Res<Time>,
    mut q: Query<(&Heading, &mut Transform, &MovementSpeed)>,
) {
    for (dir, mut tran, &speed) in &mut q {
        let pos = &mut tran.translation;
        (pos.x, pos.y) =
            (Vec2::new(pos.x, pos.y) + *speed * SCALE * time.delta_secs() * dir.v).into();
    }
}

pub mod orbiting;

