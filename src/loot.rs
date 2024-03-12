use crate::{
    enemy::{Enemy, Health},
    GameRng,
};
use bevy::prelude::*;
use rand::prelude::*;

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
/// **GAMBA MODULE:**
/// Fetch user data and check if they have donated $$$
/// increase drop rate by 500%
/// PROFIT ???
pub fn spawn_loot(commands: &mut Commands, asset_server: &Res<AssetServer>, pos: Vec3) {
    let loot_texture_handle: Handle<Image> = asset_server.load("loot/Icon29.png");
    commands.spawn(LootBundle::new(SpriteBundle {
        transform: Transform::from_xyz(pos.x, pos.y, pos.z),
        texture: loot_texture_handle,
        sprite: Sprite {
            custom_size: Some(Vec2::new(25., 25.)),
            ..Default::default()
        },
        ..default()
    }));
}

pub fn check_for_dead_enemies(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    query: Query<(&Transform, Entity, &Health), With<Enemy>>,
    mut rng: ResMut<GameRng>,
) {
    for (transform, entity, health) in query.iter() {
        if **health <= 0.0 {
            commands.entity(entity).despawn();
            // 1/5 -> 20%
            if rng.gen_range(0u16..3) == 0 as u16 {
                spawn_loot(&mut commands, &asset_server, transform.translation);
            }
        }
    }
}
