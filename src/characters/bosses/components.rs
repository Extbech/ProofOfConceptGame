use bevy::ecs::resource::Resource;

/// There is a vision here...
#[derive(Resource)]
pub enum BossSpawnHistory {
    Wizard(bool),
}
