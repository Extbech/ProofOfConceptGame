use bevy::{app::Plugin, color::palettes::css, prelude::*};

use crate::{
    cleanup,
    prestige::stats::{Stats, UpgradeOptions},
    sound::events::{PlaySoundEffectEvent, SoundEffectKind, UiSound},
    AppState,
};

use super::components::button::{custom_button, ButtonSize};

pub struct UpgradePlugin;

impl Plugin for UpgradePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::Upgrade), spawn_upgrade_ui)
            .add_systems(
                Update,
                ((handle_button_continue_click, handle_button_upgrade),)
                    .run_if(in_state(AppState::Upgrade)),
            )
            .add_systems(
                OnExit(AppState::Upgrade),
                (cleanup::<cleanup::ExitUpgradeScreen>,),
            );
    }
}

#[derive(Component)]
pub enum ButtonAction {
    MainMenu,
}

fn spawn_upgrade_ui(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut stats: ResMut<Stats>,
) {
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
            BackgroundColor(css::BLACK.into()),
            cleanup::ExitUpgradeScreen,
        ))
        .with_children(|child| {
            child
                .spawn((
                    Node {
                        flex_direction: FlexDirection::Column,
                        margin: UiRect {
                            left: Val::Percent(0.),
                            right: Val::Percent(0.),
                            top: Val::Percent(5.),
                            bottom: Val::Percent(5.),
                        },
                        ..default()
                    },
                    BackgroundColor(css::BLUE.into()),
                ))
                .with_children(|text_info_child| {
                    text_info_child.spawn((
                        Text::new("Upgrade Your Character !"),
                        TextFont {
                            font: asset_server.load("font/pixel-font.ttf"),
                            font_size: 32.0,
                            ..Default::default()
                        },
                        TextColor(css::ORANGE.into()),
                        TextLayout::new_with_justify(JustifyText::Center),
                    ));
                    text_info_child.spawn((
                        Text::new(format!("Coins: {}", stats.coins)),
                        TextFont {
                            font: asset_server.load("font/pixel-font.ttf"),
                            font_size: 16.0,
                            ..Default::default()
                        },
                        TextColor(css::WHITE.into()),
                        TextLayout::new_with_justify(JustifyText::Center),
                    ));
                });
            child
                .spawn((
                    Node {
                        width: Val::Percent(80.),
                        height: Val::Percent(80.),
                        flex_direction: FlexDirection::Column,
                        justify_content: JustifyContent::SpaceBetween,
                        overflow: Overflow::scroll_y(),
                        ..default()
                    },
                    BackgroundColor(css::RED.into()),
                ))
                .with_children(|upgrade_child| {
                    upgrade_options_bundle(
                        upgrade_child,
                        &asset_server,
                        &mut stats,
                        UpgradeOptions::DamageMultiplier,
                    );
                    upgrade_options_bundle(
                        upgrade_child,
                        &asset_server,
                        &mut stats,
                        UpgradeOptions::HealthRegen,
                    );
                    upgrade_options_bundle(
                        upgrade_child,
                        &asset_server,
                        &mut stats,
                        UpgradeOptions::MaximumHealth,
                    );
                    upgrade_options_bundle(
                        upgrade_child,
                        &asset_server,
                        &mut stats,
                        UpgradeOptions::DamageMultiplier,
                    );
                    upgrade_options_bundle(
                        upgrade_child,
                        &asset_server,
                        &mut stats,
                        UpgradeOptions::HealthRegen,
                    );
                    upgrade_options_bundle(
                        upgrade_child,
                        &asset_server,
                        &mut stats,
                        UpgradeOptions::MaximumHealth,
                    );
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
                                width: Val::Px(350.),
                                height: Val::Px(100.),
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

fn handle_button_continue_click(
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

fn upgrade_options_bundle(
    builder: &mut ChildBuilder,
    asset_server: &Res<'_, AssetServer>,
    stats: &mut Stats,
    upgrade_options: UpgradeOptions,
) {
    let price_text = match stats.get_next_price(upgrade_options) {
        Some(price) => &format!("{price}"),
        None => "MAX",
    };
    let button_color = match stats.is_upgradeable(upgrade_options) {
        Some(true) => css::GREEN,
        Some(false) => css::RED,
        None => css::GRAY,
    };
    builder
        .spawn((
            Node {
                width: Val::Percent(100.),
                height: Val::Px(100.),
                flex_direction: FlexDirection::Row,
                justify_content: JustifyContent::SpaceBetween,
                margin: UiRect {
                    left: Val::Px(0.),
                    right: Val::Px(0.),
                    top: Val::Px(0.),
                    bottom: Val::Px(3.),
                },
                ..default()
            },
            BackgroundColor(css::ORANGE.into()),
        ))
        .with_children(|child| {
            child.spawn((
                Text::new(stats.get_upgrade_info(upgrade_options)),
                TextFont {
                    font: asset_server.load("font/pixel-font.ttf"),
                    font_size: 16.,
                    ..Default::default()
                },
                TextColor(css::WHITE.into()),
                TextLayout::new_with_justify(JustifyText::Center),
            ));
            custom_button(
                child,
                asset_server,
                upgrade_options,
                button_color,
                Color::WHITE,
                price_text,
                ButtonSize::Medium,
            );
        });
}

fn handle_button_upgrade(
    interaction_query: Query<(&Interaction, &UpgradeOptions), (Changed<Interaction>, With<Button>)>,
    mut stats: ResMut<Stats>,
    mut sound_event: EventWriter<PlaySoundEffectEvent>,
) {
    for (interaction, upgradeoption) in interaction_query.iter() {
        match interaction {
            Interaction::Pressed => {
                sound_event.send(PlaySoundEffectEvent(SoundEffectKind::UiSound(
                    UiSound::ClickButtonSound,
                )));
                stats.upgrade(*upgradeoption);
            }
            Interaction::Hovered => (),
            Interaction::None => (),
        }
    }
}
