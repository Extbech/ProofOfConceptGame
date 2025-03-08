use crate::{cleanup, AppState};
use crate::{cleanup::ExitPauseScreen, GameState};
use bevy::{app::Plugin, color::palettes::css, prelude::*};

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
                    grandchild
                        .spawn((
                            Node {
                                width: Val::Percent(60.0),
                                height: Val::Percent(20.0),
                                align_items: AlignItems::Center,
                                justify_content: JustifyContent::Center,
                                margin: UiRect::all(Val::Px(10.0)),
                                ..default()
                            },
                            Button,
                            BackgroundColor(css::MIDNIGHT_BLUE.into()),
                            OptionsButtonAction::Continue,
                        ))
                        .with_children(|text_child| {
                            text_child.spawn((
                                Text::new("Continue"),
                                TextFont {
                                    font: asset_server.load("font/pixel-font.ttf"),
                                    font_size: 38.0,
                                    ..Default::default()
                                },
                                TextColor(Color::WHITE),
                            ));
                        });
                    grandchild
                        .spawn((
                            Node {
                                width: Val::Percent(60.0),
                                height: Val::Percent(20.0),
                                align_items: AlignItems::Center,
                                justify_content: JustifyContent::Center,
                                margin: UiRect::all(Val::Px(10.0)),
                                ..default()
                            },
                            Button,
                            BackgroundColor(css::MIDNIGHT_BLUE.into()),
                            OptionsButtonAction::Settings,
                        ))
                        .with_children(|text_child| {
                            text_child.spawn((
                                Text::new("Settings"),
                                TextFont {
                                    font: asset_server.load("font/pixel-font.ttf"),
                                    font_size: 38.0,
                                    ..Default::default()
                                },
                                TextColor(Color::WHITE),
                            ));
                        });

                    grandchild
                        .spawn((
                            Node {
                                width: Val::Percent(60.0),
                                height: Val::Percent(20.0),
                                align_items: AlignItems::Center,
                                justify_content: JustifyContent::Center,
                                margin: UiRect::all(Val::Px(10.0)),
                                ..default()
                            },
                            Button,
                            BackgroundColor(css::MIDNIGHT_BLUE.into()),
                            OptionsButtonAction::Exit,
                        ))
                        .with_children(|text_child| {
                            text_child.spawn((
                                Text::new("Exit Game"),
                                TextFont {
                                    font: asset_server.load("font/pixel-font.ttf"),
                                    font_size: 38.0,
                                    ..Default::default()
                                },
                                TextColor(Color::WHITE),
                            ));
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
                *background_color = css::ORANGE.into();
            }
            Interaction::None => {
                *background_color = css::MIDNIGHT_BLUE.into();
            }
        }
    }
}
