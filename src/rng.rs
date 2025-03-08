use bevy::prelude::*;
use rand::prelude::*;
use rand::rngs::SmallRng;

pub struct RngPlugin;

impl Plugin for RngPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(GameRng(SmallRng::from_entropy()));
    }
}

#[derive(Resource, Deref, DerefMut)]
pub struct GameRng(SmallRng);

impl GameRng {
    pub fn rand_vec(&mut self, inner_bound: f32, outer_bound: f32) -> Vec2 {
        let angle: f32 = self.gen_range(0.0..(2. * std::f32::consts::PI));
        let r: f32 = self.gen_range(inner_bound..=outer_bound);
        r * Vec2::new(angle.sin(), angle.cos())
    }
}
