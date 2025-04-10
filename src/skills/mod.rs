use bevy::prelude::*;
use skills::animate_lightning;

use crate::GameState;
mod bundles;
pub mod skills;
pub mod skills_tooltips;

pub struct SkillsPlugin;

impl Plugin for SkillsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (animate_lightning,).run_if(in_state(GameState::Running)),
        );
    }
}
