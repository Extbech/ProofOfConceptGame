use bevy::{color::palettes::css, prelude::*};

use crate::sound::sound_volume::SoundVolume;
use crate::{cleanup, AppState};

use super::components::button::{custom_button, ButtonSize};
pub struct SettingsPlugin;

impl Plugin for SettingsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::Settings), setup_settings_ui)
            .add_systems(
                Update,
                (handle_save_settings).run_if(in_state(AppState::Settings)),
            )
            .add_systems(
                OnExit(AppState::Settings),
                (cleanup::<cleanup::ExitSettingsScreen>,),
            );
    }
}

#[derive(Component)]
struct SaveSettingsButton;

fn setup_settings_ui(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands
        .spawn((
            Node {
                width: Val::Percent(100.),
                height: Val::Percent(100.),
                align_items: AlignItems::Center,
                flex_direction: FlexDirection::Column,
                ..default()
            },
            BackgroundColor(css::BLACK.into()),
            cleanup::ExitSettingsScreen,
        ))
        .with_children(|child| {
            custom_button(
                child,
                &asset_server,
                SaveSettingsButton,
                css::MIDNIGHT_BLUE,
                css::WHITE,
                "Save",
                ButtonSize::Large,
            );
        });
}

fn handle_save_settings(
    mut interaction_query: Query<
        &Interaction,
        (Changed<Interaction>, With<Button>, With<SaveSettingsButton>),
    >,
    mut app_state: ResMut<NextState<AppState>>,
) {
    for interaction in &mut interaction_query {
        match *interaction {
            Interaction::Pressed => {
                app_state.set(AppState::MainMenu);
            }
            _ => (),
        }
    }
}
