use bevy::{app::Plugin, color::palettes::css, prelude::*};

use crate::{
    cleanup,
    sound::events::{PlaySoundEffectEvent, SoundEffectKind, UiSound},
    tools::damage_tracking::DamageTracker,
    AppState, GameState,
};

pub struct LossPlugin;

impl Plugin for LossPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Loss), (spawn_loss_ui,))
            .add_systems(
                Update,
                (handle_button_continue_click)
                    .run_if(in_state(AppState::InGame))
                    .run_if(in_state(GameState::Loss)),
            )
            .add_systems(
                OnExit(GameState::Loss),
                (cleanup::<cleanup::ExitLossScreen>,),
            );
    }
}

#[derive(Component)]
pub struct LossUi;

pub fn spawn_loss_ui(
    mut commands: Commands,
    ui_query: Query<Entity, With<LossUi>>,
    asset_server: Res<AssetServer>,
    damage_tracker: Res<DamageTracker>,
) {
    for entity in &ui_query {
        commands.entity(entity).despawn();
    }

    commands
        .spawn((
            Node {
                width: Val::Percent(100.),
                height: Val::Percent(100.),
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                flex_direction: FlexDirection::Column,
                ..default()
            },
            BackgroundColor(Color::Srgba(Srgba {
                red: 0.0,
                green: 0.0,
                blue: 0.0,
                alpha: 0.9,
            })),
            LossUi,
            cleanup::ExitLossScreen,
        ))
        .with_children(|child| {
            child
                .spawn((
                    Node {
                        width: Val::Percent(50.),
                        height: Val::Percent(50.),
                        flex_direction: FlexDirection::Column,
                        margin: UiRect {
                            left: Val::Px(0.),
                            right: Val::Px(0.),
                            top: Val::Px(25.),
                            bottom: Val::Px(0.),
                        },
                        ..default()
                    },
                    BackgroundColor(css::DARK_SLATE_GRAY.into()),
                ))
                .with_children(|grandchild| {
                    grandchild.spawn((
                        Text::new("You Died!"),
                        TextFont {
                            font: asset_server.load("font/pixel-font.ttf"),
                            font_size: 40.0,
                            ..Default::default()
                        },
                        TextColor(css::DARK_RED.into()),
                        TextLayout::new_with_justify(Justify::Center),
                    ));
                    grandchild
                        .spawn(Node {
                            width: Val::Percent(100.),
                            height: Val::Percent(100.),
                            flex_direction: FlexDirection::Column,
                            margin: UiRect {
                                left: Val::Percent(0.),
                                right: Val::Percent(0.),
                                top: Val::Percent(10.),
                                bottom: Val::Percent(0.),
                            },
                            ..default()
                        })
                        .with_children(|text_info_child| {
                            text_info_child.spawn((
                                Text::new(format!(
                                    "Total damage: {}",
                                    damage_tracker.get_total_damage()
                                )),
                                TextFont {
                                    font: asset_server.load("font/pixel-font.ttf"),
                                    font_size: 25.0,
                                    ..Default::default()
                                },
                                TextColor(css::GREEN.into()),
                                TextLayout::new_with_justify(Justify::Center),
                            ));
                            damage_tracker.get_sorted_by_damage().iter().for_each(|dt| {
                                text_info_child.spawn((
                                    Text::new(format!("{}: {}", dt.spell, dt.amount)),
                                    TextFont {
                                        font: asset_server.load("font/pixel-font.ttf"),
                                        font_size: 25.0,
                                        ..Default::default()
                                    },
                                    TextColor(css::GREEN.into()),
                                    TextLayout::new_with_justify(Justify::Center),
                                ));
                            });
                        });
                    grandchild
                        .spawn((Node {
                            width: Val::Percent(100.),
                            height: Val::Percent(100.),
                            flex_direction: FlexDirection::Column,
                            align_items: AlignItems::Center,
                            justify_content: JustifyContent::Center,
                            margin: UiRect {
                                left: Val::Percent(0.),
                                right: Val::Percent(0.),
                                top: Val::Percent(10.),
                                bottom: Val::Percent(0.),
                            },
                            ..default()
                        },))
                        .with_children(|button_box_child| {
                            button_box_child
                                .spawn((
                                    Node {
                                        width: Val::Percent(50.),
                                        height: Val::Percent(50.),
                                        align_items: AlignItems::Center,
                                        justify_content: JustifyContent::Center,
                                        ..default()
                                    },
                                    Button,
                                    BackgroundColor(css::BLUE.into()),
                                ))
                                .with_children(|button_box_text| {
                                    button_box_text.spawn((
                                        Text::new("Continue"),
                                        TextFont {
                                            font: asset_server.load("font/pixel-font.ttf"),
                                            font_size: 30.,
                                            ..default()
                                        },
                                        TextColor(css::WHITE.into()),
                                        TextLayout::new_with_justify(Justify::Center),
                                    ));
                                });
                        });
                });
        });
}

pub fn handle_button_continue_click(
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor),
        (Changed<Interaction>, With<Button>),
    >,
    mut app_state: ResMut<NextState<AppState>>,
    mut game_state: ResMut<NextState<GameState>>,
    mut sound_event: MessageWriter<PlaySoundEffectEvent>,
) {
    for (interaction, mut background_color) in &mut interaction_query {
        match interaction {
            Interaction::Pressed => {
                sound_event.write(PlaySoundEffectEvent(SoundEffectKind::Ui(
                    UiSound::ClickButtonSound,
                )));
                game_state.set(GameState::NotStarted);
                app_state.set(AppState::MainMenu);
            }
            Interaction::Hovered => {
                sound_event.write(PlaySoundEffectEvent(SoundEffectKind::Ui(
                    UiSound::HoverButtonSound,
                )));
                *background_color = css::ORANGE.into();
            }
            Interaction::None => *background_color = css::BLUE.into(),
        }
    }
}
