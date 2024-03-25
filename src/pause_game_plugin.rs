use bevy::{app::Plugin, prelude::*};

use crate::{cleanup, AppState};
use crate::{cleanup::ExitPauseScreen, GameState};
pub struct PauseGamePlugin;

impl Plugin for PauseGamePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Paused), render_pause_options_node)
            .add_systems(
                Update,
                (handle_options_interaction).run_if(in_state(GameState::Paused)),
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
    if keys.pressed(KeyCode::Escape) {
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
            NodeBundle {
                style: Style {
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    ..default()
                },
                background_color: Color::rgba(0.0, 0.0, 0.0, 0.8).into(),
                ..default()
            },
            ExitPauseScreen,
        ))
        .with_children(|child| {
            child
                .spawn(NodeBundle {
                    style: Style {
                        width: Val::Percent(35.0),
                        height: Val::Percent(70.0),
                        align_items: AlignItems::Center,
                        justify_content: JustifyContent::SpaceEvenly,
                        flex_direction: FlexDirection::Column,
                        ..default()
                    },
                    background_color: Color::DARK_GRAY.into(),
                    ..default()
                })
                .with_children(|grandchild| {
                    grandchild
                        .spawn((
                            ButtonBundle {
                                style: Style {
                                    width: Val::Percent(60.0),
                                    height: Val::Percent(20.0),
                                    align_items: AlignItems::Center,
                                    justify_content: JustifyContent::Center,
                                    margin: UiRect::all(Val::Px(10.0)),
                                    ..default()
                                },
                                background_color: Color::MIDNIGHT_BLUE.into(),
                                ..default()
                            },
                            OptionsButtonAction::Continue,
                        ))
                        .with_children(|text_child| {
                            text_child.spawn(
                                TextBundle::from_section(
                                    "Continue",
                                    TextStyle {
                                        font: asset_server.load("font/pixel-font.ttf"),
                                        color: Color::WHITE,
                                        font_size: 38.0,
                                    },
                                )
                                .with_text_justify(JustifyText::Center),
                            );
                        });
                    grandchild
                        .spawn((
                            ButtonBundle {
                                style: Style {
                                    width: Val::Percent(60.0),
                                    height: Val::Percent(20.0),
                                    align_items: AlignItems::Center,
                                    justify_content: JustifyContent::Center,
                                    margin: UiRect::all(Val::Px(10.0)),
                                    ..default()
                                },
                                background_color: Color::MIDNIGHT_BLUE.into(),
                                ..default()
                            },
                            OptionsButtonAction::Settings,
                        ))
                        .with_children(|text_child| {
                            text_child.spawn(
                                TextBundle::from_section(
                                    "Settings",
                                    TextStyle {
                                        font: asset_server.load("font/pixel-font.ttf"),
                                        color: Color::WHITE,
                                        font_size: 38.0,
                                    },
                                )
                                .with_text_justify(JustifyText::Center),
                            );
                        });

                    grandchild
                        .spawn((
                            ButtonBundle {
                                style: Style {
                                    width: Val::Percent(60.0),
                                    height: Val::Percent(20.0),
                                    align_items: AlignItems::Center,
                                    justify_content: JustifyContent::Center,
                                    margin: UiRect::all(Val::Px(10.0)),
                                    ..default()
                                },
                                background_color: Color::MIDNIGHT_BLUE.into(),
                                ..default()
                            },
                            OptionsButtonAction::Exit,
                        ))
                        .with_children(|text_child| {
                            text_child.spawn(
                                TextBundle::from_section(
                                    "Exit Game",
                                    TextStyle {
                                        font: asset_server.load("font/pixel-font.ttf"),
                                        color: Color::WHITE,
                                        font_size: 38.0,
                                    },
                                )
                                .with_text_justify(JustifyText::Center),
                            );
                        });
                });
        });
}

pub fn handle_options_interaction(
    mut interaction_query: Query<
        (&Interaction, &OptionsButtonAction, &mut BackgroundColor),
        (Changed<Interaction>, With<Button>),
    >,
    mut game_state: ResMut<NextState<GameState>>,
    mut app_state: ResMut<NextState<AppState>>,
) {
    for (interaction, action_type, mut background_color) in &mut interaction_query {
        match interaction {
            Interaction::Pressed => match action_type {
                OptionsButtonAction::Continue => game_state.set(GameState::Running),
                OptionsButtonAction::Settings => {}
                OptionsButtonAction::Exit => app_state.set(AppState::MainMenu),
            },
            Interaction::Hovered => {
                *background_color = Color::ORANGE.into();
            }
            Interaction::None => {
                *background_color = Color::MIDNIGHT_BLUE.into();
            }
        }
    }
}
