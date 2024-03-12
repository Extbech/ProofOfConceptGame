use bevy::prelude::*;

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
    let loot_texture_handle: Handle<Image> = asset_server.load("models/bullet.png");
    commands.spawn(LootBundle::new(SpriteBundle {
        transform: Transform::from_xyz(pos.x, pos.y, pos.z),
        texture: loot_texture_handle,
        sprite: Sprite {
            custom_size: Some(Vec2::new(100., 100.)),
            ..Default::default()
        },
        ..default()
    }));
}
