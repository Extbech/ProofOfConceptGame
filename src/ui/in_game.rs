use bevy::prelude::*;

use crate::{
    characters::player::{CurrentLevel, CurrentXP, MaxHealth, Player, RequiredXP},
    cleanup::{self, ExitGame},
    mechanics::cooldown::InGameTime,
    mechanics::damage::Health,
    SCALE,
};
#[derive(Component)]
pub struct HealthUiSprite;

#[derive(Component)]
pub struct XPBar;

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
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                flex_direction: FlexDirection::ColumnReverse,
                ..default()
            },
            ExitGame,
            XPBar,
        ))
        .with_children(|child| {
            child
                .spawn((
                    Node {
                        width: Val::Percent(100.0),
                        height: Val::Percent(5.0),
                        align_self: AlignSelf::End,
                        ..default()
                    },
                    BackgroundColor(Color::Srgba(Srgba {
                        red: 0.0,
                        green: 0.0,
                        blue: 0.0,
                        alpha: 0.5,
                    })),
                ))
                .with_children(|grandchild| {
                    grandchild.spawn((
                        Node {
                            width: Val::Percent(xp_percent),
                            height: Val::Percent(100.0),
                            ..default()
                        },
                        BackgroundColor(Color::Srgba(Srgba {
                            red: 0.0,
                            green: 255.0,
                            blue: 255.0,
                            alpha: 1.0,
                        })),
                    ));
                });
            child
                .spawn(Node {
                    width: Val::Percent(100.0),
                    height: Val::Percent(10.0),
                    align_self: AlignSelf::End,
                    flex_direction: FlexDirection::Row,
                    ..default()
                })
                .with_children(|grandchild| {
                    grandchild.spawn(Node {
                        width: Val::Percent(85.0),
                        height: Val::Percent(100.0),
                        align_self: AlignSelf::End,
                        ..default()
                    });
                    grandchild.spawn((
                        Text::new(format!("lvl. {}", **level)),
                        TextFont {
                            font: asset_server.load("font/pixel-font.ttf"),
                            font_size: 60.0,
                            ..default()
                        },
                        TextColor(Color::WHITE),
                    ));
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
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(5.0),
                ..default()
            },
            HealthUiSprite,
            cleanup::ExitGame,
        ))
        .with_children(|child| {
            for i in 0..**player_max_health {
                if i < **player_health {
                    child.spawn((
                        Node {
                            width: Val::Px(21.0 / SCALE),
                            height: Val::Px(20.0 / SCALE),
                            margin: UiRect {
                                left: Val::Px(5.0),
                                top: Val::Px(5.0),
                                ..default()
                            },
                            ..default()
                        },
                        ImageNode::new(asset_server.load("ui/heart.png")),
                        HealthUiSprite,
                    ));
                } else {
                    child.spawn((
                        Node {
                            width: Val::Px(21.0 / SCALE),
                            height: Val::Px(20.0 / SCALE),
                            margin: UiRect {
                                left: Val::Px(5.0),
                                top: Val::Px(5.0),
                                ..default()
                            },
                            ..default()
                        },
                        ImageNode {
                            image: asset_server.load("ui/heart.png"),
                            color: Color::srgba(0.0, 0.0, 0.0, 0.7).into(),
                            ..Default::default()
                        },
                        HealthUiSprite,
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
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(8.0),
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                ..default()
            },
            cleanup::ExitGame,
            StopWatchDisplay,
        ))
        .with_children(|child| {
            child.spawn((
                Text::new(format!(
                    "{}:{:0>2}",
                    igt.time().as_secs() / 60,
                    igt.time().as_secs() % 60
                )),
                TextFont {
                    font: asset_server.load("font/pixel-font.ttf"),
                    //color: Color::WHITE,
                    font_size: 34.0,
                    ..default()
                },
                TextColor(Color::WHITE),
                TextLayout {
                    justify: JustifyText::Center,
                    ..Default::default()
                },
            ));
        });
}
