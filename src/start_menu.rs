use bevy::prelude::*;

use crate::{cleanup, AppState, MyGameCamera};
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
    let button_style = Style {
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
    };
    commands
        .spawn((
            NodeBundle {
                style: Style {
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    align_items: AlignItems::Center,
                    flex_direction: FlexDirection::Column,
                    ..default()
                },
                background_color: Color::MIDNIGHT_BLUE.into(),
                ..default()
            },
            MainMenuScreen,
        ))
        .with_children(|child| {
            child
                .spawn(NodeBundle {
                    style: Style {
                        width: Val::Percent(50.0),
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
                    background_color: Color::MIDNIGHT_BLUE.into(),
                    ..default()
                })
                .with_children(|grandchild| {
                    grandchild.spawn(TextBundle::from_section(
                        GAME_TITLE,
                        TextStyle {
                            font_size: 80.0,
                            color: Color::ORANGE,
                            ..default()
                        },
                    ));
                    grandchild
                        .spawn((
                            ButtonBundle {
                                style: button_style.clone(),
                                background_color: Color::ORANGE.into(),
                                ..default()
                            },
                            MenuButtonAction::Play,
                        ))
                        .with_children(|great_grandchild| {
                            great_grandchild.spawn(TextBundle::from_section(
                                "Start Game",
                                TextStyle {
                                    font_size: 50.0,
                                    color: Color::WHITE,
                                    ..default()
                                },
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
) {
    for (interaction, menu_button_action) in &interaction_query {
        if *interaction == Interaction::Pressed {
            match menu_button_action {
                MenuButtonAction::Play => app_state.set(AppState::InGame),
            }
        }
    }
}
