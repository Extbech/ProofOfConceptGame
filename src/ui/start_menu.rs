use crate::sound::events::{PlaySoundEffectEvent, SoundEffectKind, UiSound};
use crate::{cleanup, AppState};
use bevy::color::palettes::css;
use bevy::prelude::*;
use test_game::GAME_TITLE;

use super::components::button::{custom_button, ButtonSize};

// Tag component used to tag entities added on the main menu screen
#[derive(Component, Clone, Copy)]
enum MenuButtonAction {
    Play,
    Upgrade,
    ExitGame,
    Settings,
}
#[derive(Component)]
struct MainMenuScreen;
pub struct StartMenuPlugin<S: States> {
    pub state: S,
}
impl<S: States> Plugin for StartMenuPlugin<S> {
    fn build(&self, app: &mut App) {
        app.add_systems(
            OnEnter(AppState::MainMenu),
            render_start_menu.run_if(in_state(self.state.clone())),
        )
        .add_systems(
            Update,
            handle_button_click.run_if(in_state(self.state.clone())),
        )
        .add_systems(OnExit(AppState::MainMenu), cleanup::<MainMenuScreen>);
    }
}

pub fn render_start_menu(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands
        .spawn((
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                align_items: AlignItems::Center,
                flex_direction: FlexDirection::Column,
                ..default()
            },
            BackgroundColor(css::BLACK.into()),
            MainMenuScreen,
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
                        Text::new(GAME_TITLE),
                        TextFont {
                            font: asset_server.load("font/pixel-font.ttf"),
                            font_size: 50.0,
                            ..Default::default()
                        },
                        TextColor(css::ORANGE.into()),
                    ));
                    custom_button(
                        grandchild,
                        &asset_server,
                        MenuButtonAction::Play,
                        css::MIDNIGHT_BLUE,
                        css::WHITE,
                        "Start Game",
                        ButtonSize::Large,
                    );
                    custom_button(
                        grandchild,
                        &asset_server,
                        MenuButtonAction::Upgrade,
                        css::MIDNIGHT_BLUE,
                        css::WHITE,
                        "Upgrades",
                        ButtonSize::Large,
                    );
                    custom_button(
                        grandchild,
                        &asset_server,
                        MenuButtonAction::Settings,
                        css::MIDNIGHT_BLUE,
                        css::WHITE,
                        "Settings",
                        ButtonSize::Large,
                    );
                    custom_button(
                        grandchild,
                        &asset_server,
                        MenuButtonAction::ExitGame,
                        css::MIDNIGHT_BLUE,
                        css::WHITE,
                        "Exit Game",
                        ButtonSize::Large,
                    );
                });
        });
}

fn handle_button_click(
    mut interaction_query: Query<
        (&Interaction, &MenuButtonAction, &mut BackgroundColor),
        (Changed<Interaction>, With<Button>),
    >,
    mut app_state: ResMut<NextState<AppState>>,
    mut sound_event: EventWriter<PlaySoundEffectEvent>,
    mut exit: EventWriter<AppExit>,
) {
    for (interaction, menu_button_action, mut background_color) in &mut interaction_query {
        match *interaction {
            Interaction::Pressed => {
                sound_event.send(PlaySoundEffectEvent(SoundEffectKind::UiSound(
                    UiSound::ClickButtonSound,
                )));
                match menu_button_action {
                    MenuButtonAction::Play => app_state.set(AppState::InGame),
                    MenuButtonAction::Upgrade => app_state.set(AppState::Upgrade),
                    MenuButtonAction::Settings => app_state.set(AppState::Settings),
                    MenuButtonAction::ExitGame => {
                        exit.send(AppExit::Success);
                    }
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
