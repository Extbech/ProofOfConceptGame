use bevy::app::{App, Plugin};

pub mod components;
pub mod wizard;

pub(super) struct BossesPlugin;

impl Plugin for BossesPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(wizard::WizardBossPlugin);
    }
}
