use bevy::prelude::*;

use crate::{
    cleanup::{self, ExitGame},
    cooldown::InGameTime,
    damage::Health,
    player::{CurrentLevel, CurrentXP, MaxHealth, Player, RequiredXP},
    SCALE,
};
#[derive(Component)]
pub struct HealthUiSprite;

#[derive(Component)]
pub struct XPBar;

#[derive(Component)]
pub struct Item;

pub fn update_xp_bar_and_level(
    mut commands: Commands,
    query: Query<(&RequiredXP, &CurrentXP, &CurrentLevel), With<Player>>,
    asset_server: Res<AssetServer>,
    query_bar: Query<Entity, With<XPBar>>,
) {
    for entity in &query_bar {
        commands.entity(entity).despawn_recursive();
    }
    let (req_xp, curr_xp, level) = query.single();
    let xp_percent = if **curr_xp == 0.0 {
        0.0
    } else {
        (**curr_xp / **req_xp) * 100.0
    };
    commands
        .spawn((
            NodeBundle {
                style: Style {
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    flex_direction: FlexDirection::ColumnReverse,
                    ..default()
                },
                ..default()
            },
            ExitGame,
            XPBar,
        ))
        .with_children(|child| {
            child
                .spawn(NodeBundle {
                    style: Style {
                        width: Val::Percent(100.0),
                        height: Val::Percent(5.0),
                        align_self: AlignSelf::End,
                        ..default()
                    },
                    background_color: Color::GRAY.into(),
                    ..default()
                })
                .with_children(|grandchild| {
                    grandchild.spawn(NodeBundle {
                        style: Style {
                            width: Val::Percent(xp_percent),
                            height: Val::Percent(100.0),
                            ..default()
                        },
                        background_color: Color::CYAN.into(),
                        ..default()
                    });
                });
            child
                .spawn(NodeBundle {
                    style: Style {
                        width: Val::Percent(100.0),
                        height: Val::Percent(10.0),
                        align_self: AlignSelf::End,
                        flex_direction: FlexDirection::Row,
                        ..default()
                    },

                    ..default()
                })
                .with_children(|grandchild| {
                    grandchild.spawn(NodeBundle {
                        style: Style {
                            width: Val::Percent(85.0),
                            height: Val::Percent(100.0),
                            align_self: AlignSelf::End,
                            ..default()
                        },
                        ..default()
                    });
                    grandchild.spawn(
                        TextBundle::from_section(
                            format!("lvl. {}", **level),
                            TextStyle {
                                font: asset_server.load("font/pixel-font.ttf"),
                                font_size: 60.0,
                                color: Color::WHITE,
                            },
                        )
                        .with_no_wrap(),
                    );
                });
        });
}

pub fn update_health_ui(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut q_health: Query<(&mut Transform, Entity), (With<HealthUiSprite>, Without<Player>)>,
    q_player_health: Query<(&Health, &MaxHealth), (With<Player>, Without<HealthUiSprite>)>,
) {
    let (player_health, player_max_health) = q_player_health.single();
    for (mut _transform, entity) in q_health.iter_mut() {
        commands.entity(entity).despawn();
    }
    commands
        .spawn((
            NodeBundle {
                style: Style {
                    width: Val::Percent(100.0),
                    height: Val::Percent(5.0),
                    ..default()
                },
                ..default()
            },
            HealthUiSprite,
            cleanup::ExitGame,
        ))
        .with_children(|child| {
            for i in 0..**player_max_health {
                if i < **player_health {
                    child.spawn((
                        NodeBundle {
                            style: Style {
                                width: Val::Px(32.0 / SCALE),
                                height: Val::Px(32.0 / SCALE),
                                margin: UiRect {
                                    left: Val::Px(5.0),
                                    top: Val::Px(5.0),
                                    ..default()
                                },
                                ..default()
                            },
                            background_color: Color::WHITE.into(),
                            ..default()
                        },
                        UiImage::new(asset_server.load("ui/heart-pixel.png")),
                    ));
                } else {
                    child.spawn((
                        NodeBundle {
                            style: Style {
                                width: Val::Px(32.0 / SCALE),
                                height: Val::Px(32.0 / SCALE),
                                margin: UiRect {
                                    left: Val::Px(5.0),
                                    top: Val::Px(5.0),
                                    ..default()
                                },
                                ..default()
                            },
                            background_color: Color::rgba(0.0, 0.0, 0.0, 0.7).into(),
                            ..default()
                        },
                        UiImage::new(asset_server.load("ui/heart-pixel.png")),
                    ));
                }
            }
        });
}

#[derive(Component)]
pub struct StopWatchDisplay;

pub fn render_stop_watch(
    igt: Res<InGameTime>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    query: Query<Entity, With<StopWatchDisplay>>,
) {
    for entity in &query {
        commands.entity(entity).despawn_recursive();
    }
    commands
        .spawn((
            NodeBundle {
                style: Style {
                    width: Val::Percent(100.0),
                    height: Val::Percent(8.0),
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    ..default()
                },
                ..default()
            },
            cleanup::ExitGame,
            StopWatchDisplay,
        ))
        .with_children(|child| {
            child.spawn(
                TextBundle::from_section(
                    format!(
                        "{}:{:0>2}",
                        igt.time().as_secs() / 60,
                        igt.time().as_secs() % 60
                    ),
                    TextStyle {
                        font: asset_server.load("font/pixel-font.ttf"),
                        color: Color::WHITE,
                        font_size: 34.0,
                    },
                )
                .with_text_justify(JustifyText::Center),
            );
        });
}
