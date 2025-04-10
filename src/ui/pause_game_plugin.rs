use crate::sound::events::{PlaySoundEffectEvent, SoundEffectKind, UiSound};
use crate::{cleanup, AppState};
use crate::{cleanup::ExitPauseScreen, GameState};
use bevy::{app::Plugin, color::palettes::css, prelude::*};

use super::components::button::{custom_button, ButtonSize};

pub struct PauseGamePlugin;

impl Plugin for PauseGamePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Paused), render_pause_options_node)
            .add_systems(
                Update,
                ((handle_options_interaction, handle_escape_press))
                    .run_if(in_state(GameState::Paused)),
            )
            .add_systems(
                OnExit(GameState::Paused),
                (cleanup::<cleanup::ExitPauseScreen>,),
            );
    }
}

pub fn check_if_paused(
    mut game_state: ResMut<NextState<GameState>>,
    keys: Res<ButtonInput<KeyCode>>,
) {
    if keys.just_pressed(KeyCode::Escape) {
        game_state.set(GameState::Paused);
    }
}

#[derive(Component)]
pub enum OptionsButtonAction {
    Continue,
    Settings,
    Exit,
}
pub fn render_pause_options_node(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    query: Query<Entity, With<ExitPauseScreen>>,
) {
    for entity in &query {
        commands.entity(entity).despawn_recursive();
    }
    commands
        .spawn((
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
            BackgroundColor(Color::Srgba(Srgba {
                red: 0.0,
                green: 0.0,
                blue: 0.0,
                alpha: 0.8,
            })),
            ExitPauseScreen,
        ))
        .with_children(|child| {
            child
                .spawn((
                    Node {
                        width: Val::Percent(35.0),
                        height: Val::Percent(70.0),
                        align_items: AlignItems::Center,
                        justify_content: JustifyContent::SpaceEvenly,
                        flex_direction: FlexDirection::Column,
                        ..default()
                    },
                    BackgroundColor(css::DARK_GRAY.into()),
                ))
                .with_children(|grandchild| {
                    custom_button(
                        grandchild,
                        &asset_server,
                        OptionsButtonAction::Continue,
                        css::MIDNIGHT_BLUE,
                        Color::WHITE,
                        "Continue",
                        ButtonSize::Large,
                    );
                    custom_button(
                        grandchild,
                        &asset_server,
                        OptionsButtonAction::Settings,
                        css::MIDNIGHT_BLUE,
                        Color::WHITE,
                        "Settings",
                        ButtonSize::Large,
                    );
                    custom_button(
                        grandchild,
                        &asset_server,
                        OptionsButtonAction::Exit,
                        css::MIDNIGHT_BLUE,
                        Color::WHITE,
                        "Exit Game",
                        ButtonSize::Large,
                    );
                });
        });
}

fn handle_options_interaction(
    mut interaction_query: Query<
        (&Interaction, &OptionsButtonAction, &mut BackgroundColor),
        (Changed<Interaction>, With<Button>),
    >,
    mut game_state: ResMut<NextState<GameState>>,
    mut app_state: ResMut<NextState<AppState>>,
    mut sound_event: EventWriter<PlaySoundEffectEvent>,
) {
    for (interaction, action_type, mut background_color) in &mut interaction_query {
        match interaction {
            Interaction::Pressed => {
                sound_event.send(PlaySoundEffectEvent(SoundEffectKind::UiSound(
                    UiSound::ClickButtonSound,
                )));
                match action_type {
                    OptionsButtonAction::Continue => game_state.set(GameState::Running),
                    OptionsButtonAction::Settings => app_state.set(AppState::Settings),
                    OptionsButtonAction::Exit => app_state.set(AppState::MainMenu),
                }
            }
            Interaction::Hovered => {
                sound_event.send(PlaySoundEffectEvent(SoundEffectKind::UiSound(
                    UiSound::HoverButtonSound,
                )));
                *background_color = css::ORANGE.into();
            }
            Interaction::None => {
                *background_color = css::MIDNIGHT_BLUE.into();
            }
        }
    }
}

fn handle_escape_press(
    mut key: ResMut<ButtonInput<KeyCode>>,
    mut game_state: ResMut<NextState<GameState>>,
) {
    if key.clear_just_pressed(KeyCode::Escape) {
        game_state.set(GameState::Running)
    }
}
