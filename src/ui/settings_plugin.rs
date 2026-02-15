use crate::{cleanup, AppState};
use bevy::{color::palettes::css, prelude::*};

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
enum ButtonAction {
    SaveSettings,
    ExitSettings,
}

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
            child
                .spawn((
                    Node {
                        width: Val::Percent(70.0),
                        height: Val::Percent(90.0),
                        align_items: AlignItems::Center,
                        justify_content: JustifyContent::SpaceEvenly,
                        flex_direction: FlexDirection::Column,
                        margin: UiRect {
                            left: Val::Percent(0.0),
                            right: Val::Percent(0.0),
                            top: Val::Percent(5.0),
                            bottom: Val::Percent(0.0),
                        },
                        ..default()
                    },
                    BackgroundColor(css::BLACK.into()),
                ))
                .with_children(|grandchild| {
                    grandchild.spawn((
                        Text::new("Settings"),
                        TextFont {
                            font: asset_server.load("font/pixel-font.ttf"),
                            font_size: 50.0,
                            ..default()
                        },
                        TextColor(css::ORANGE.into()),
                        TextLayout::new_with_justify(JustifyText::Center),
                    ));
                    custom_button(
                        grandchild,
                        &asset_server,
                        ButtonAction::SaveSettings,
                        css::MIDNIGHT_BLUE,
                        css::WHITE,
                        "Save",
                        ButtonSize::Large,
                    );
                    custom_button(
                        grandchild,
                        &asset_server,
                        ButtonAction::ExitSettings,
                        css::MIDNIGHT_BLUE,
                        css::WHITE,
                        "Exit",
                        ButtonSize::Large,
                    );
                });
        });
}

fn handle_save_settings(
    mut interaction_query: Query<
        (&Interaction, &ButtonAction),
        (Changed<Interaction>, With<Button>),
    >,
    mut app_state: ResMut<NextState<AppState>>,
) {
    for (interaction, button_action) in &mut interaction_query {
        match *interaction {
            Interaction::Pressed => match button_action {
                ButtonAction::SaveSettings => app_state.set(AppState::MainMenu),
                ButtonAction::ExitSettings => app_state.set(AppState::MainMenu),
            },
            _ => (),
        }
    }
}
