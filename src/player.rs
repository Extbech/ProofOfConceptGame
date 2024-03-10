use bevy::prelude::*;
use crate::{Speed, Direction};

#[derive(Component)]
pub struct Player;

#[derive(Bundle)]
pub struct PlayerBundle {
    marker: Player,
    dir: Direction,
    sprite: SpriteBundle,
    speed: Speed
}

impl PlayerBundle {
    pub fn new(sprite: SpriteBundle) -> Self {
        PlayerBundle {
            marker: Player,
            dir: default(),
            sprite,
            speed: Speed(5.)
        }
    }
}