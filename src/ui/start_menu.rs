use crate::sound::events::{PlaySoundEffectEvent, SoundEffectKind, UiSound};
use crate::{cleanup, AppState};
use bevy::color::palettes::css;
use bevy::prelude::*;
pub const GAME_TITLE: &str = "To be Announced";

// Tag component used to tag entities added on the main menu screen
#[derive(Component, Clone, Copy)]
enum MenuButtonAction {
    Play,
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
            BackgroundColor(css::MIDNIGHT_BLUE.into()),
            MainMenuScreen,
        ))
        .with_children(|child| {
            child
                .spawn((
                    Node {
                        width: Val::Percent(70.0),
                        height: Val::Percent(50.0),
                        align_items: AlignItems::Center,
                        flex_direction: FlexDirection::Column,
                        margin: UiRect {
                            left: Val::Percent(0.0),
                            right: Val::Percent(0.0),
                            top: Val::Percent(10.0),
                            bottom: Val::Percent(0.0),
                        },
                        ..default()
                    },
                    BackgroundColor(css::MIDNIGHT_BLUE.into()),
                ))
                .with_children(|grandchild| {
                    grandchild.spawn((
                        Text::new(GAME_TITLE),
                        TextFont {
                            font: asset_server.load("font/pixel-font.ttf"),
                            font_size: 60.0,
                            ..Default::default()
                        },
                        TextColor(css::ORANGE.into()),
                    ));
                    grandchild
                        .spawn((
                            Node {
                                width: Val::Px(300.0),
                                height: Val::Px(100.0),
                                margin: UiRect {
                                    left: Val::Percent(0.0),
                                    right: Val::Percent(0.0),
                                    top: Val::Percent(15.0),
                                    bottom: Val::Percent(0.0),
                                },
                                justify_content: JustifyContent::Center,
                                align_items: AlignItems::Center,
                                ..default()
                            },
                            Button,
                            BackgroundColor(css::ORANGE.into()),
                            MenuButtonAction::Play,
                        ))
                        .with_children(|great_grandchild| {
                            great_grandchild.spawn((
                                Text::new("Start Game"),
                                TextFont {
                                    font_size: 30.0,
                                    font: asset_server.load("font/pixel-font.ttf"),
                                    ..Default::default()
                                },
                                TextColor(css::WHITE.into()),
                            ));
                        });
                });
        });
}

fn handle_button_click(
    interaction_query: Query<
        (&Interaction, &MenuButtonAction),
        (Changed<Interaction>, With<Button>),
    >,
    mut app_state: ResMut<NextState<AppState>>,
    mut sound_event: EventWriter<PlaySoundEffectEvent>,
) {
    for (interaction, menu_button_action) in &interaction_query {
        match *interaction {
            Interaction::Pressed => {
                sound_event.send(PlaySoundEffectEvent(SoundEffectKind::UiSound(
                    UiSound::ClickButtonSound,
                )));
                match menu_button_action {
                    MenuButtonAction::Play => app_state.set(AppState::InGame),
                }
            }
            Interaction::Hovered => {
                sound_event.send(PlaySoundEffectEvent(SoundEffectKind::UiSound(
                    UiSound::HoverButtonSound,
                )));
            }
            Interaction::None => (),
        }
    }
}
