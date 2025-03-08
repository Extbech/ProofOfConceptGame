use bevy::{app::Plugin, color::palettes::css, prelude::*};

use crate::{
    cleanup,
    sound::events::{PlaySoundEffectEvent, SoundEffectKind, UiSound},
    tools::damage_tracking::DamageTracker,
    AppState,
};

pub struct UpgradePlugin;

impl Plugin for UpgradePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::Upgrade), (spawn_upgrade_ui,))
            .add_systems(
                Update,
                (handle_button_continue_click).run_if(in_state(AppState::Upgrade)),
            )
            .add_systems(
                OnExit(AppState::Upgrade),
                (cleanup::<cleanup::ExitUpgradeScreen>,),
            );
    }
}

#[derive(Component)]
pub struct UpgradeUi;

#[derive(Component)]
pub enum ButtonAction {
    MainMenu,
}

pub fn spawn_upgrade_ui(
    mut commands: Commands,
    ui_query: Query<Entity, With<UpgradeUi>>,
    asset_server: Res<AssetServer>,
    damage_tracker: Res<DamageTracker>,
) {
    for entity in &ui_query {
        commands.entity(entity).despawn_recursive();
    }

    commands
        .spawn((
            Node {
                width: Val::Percent(100.),
                height: Val::Percent(100.),
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                flex_direction: FlexDirection::Column,
                margin: UiRect {
                    left: Val::Percent(0.),
                    right: Val::Percent(0.),
                    top: Val::Percent(0.),
                    bottom: Val::Percent(10.),
                },
                ..default()
            },
            BackgroundColor(css::BLACK.into()),
            UpgradeUi,
            cleanup::ExitUpgradeScreen,
        ))
        .with_children(|child| {
            child
                .spawn(Node {
                    width: Val::Percent(80.),
                    height: Val::Percent(80.),
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
                        Text::new("Upgrade Your Character !"),
                        TextFont {
                            font: asset_server.load("font/pixel-font.ttf"),
                            font_size: 40.0,
                            ..Default::default()
                        },
                        TextColor(css::GREEN.into()),
                        TextLayout::new_with_justify(JustifyText::Center),
                    ));
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
                        TextLayout::new_with_justify(JustifyText::Center),
                    ));
                });
            child
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
                            BackgroundColor(css::MIDNIGHT_BLUE.into()),
                            ButtonAction::MainMenu,
                        ))
                        .with_children(|button_box_text| {
                            button_box_text.spawn((
                                Text::new("Main Menu"),
                                TextFont {
                                    font: asset_server.load("font/pixel-font.ttf"),
                                    font_size: 30.,
                                    ..default()
                                },
                                TextColor(css::WHITE.into()),
                                TextLayout::new_with_justify(JustifyText::Center),
                            ));
                        });
                });
        });
}

pub fn handle_button_continue_click(
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor, &ButtonAction),
        (Changed<Interaction>, With<Button>),
    >,
    mut app_state: ResMut<NextState<AppState>>,
    mut sound_event: EventWriter<PlaySoundEffectEvent>,
) {
    for (interaction, mut background_color, button_action) in &mut interaction_query {
        match interaction {
            Interaction::Pressed => {
                sound_event.send(PlaySoundEffectEvent(SoundEffectKind::UiSound(
                    UiSound::ClickButtonSound,
                )));
                match button_action {
                    ButtonAction::MainMenu => app_state.set(AppState::MainMenu),
                }
            }
            Interaction::Hovered => {
                sound_event.send(PlaySoundEffectEvent(SoundEffectKind::UiSound(
                    UiSound::HoverButtonSound,
                )));
                *background_color = css::ORANGE.into();
            }
            Interaction::None => *background_color = css::MIDNIGHT_BLUE.into(),
        }
    }
}
