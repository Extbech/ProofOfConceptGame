use bevy::prelude::*;

use crate::player::{CurrentLevel, CurrentXP, Player, RequiredXP};

pub fn spawn_ui(mut commands: Commands, asset_server: Res<AssetServer>) {
    todo!();
}

pub fn update_xp_bar_and_level(
    mut commands: Commands,
    query: Query<(&RequiredXP, &CurrentXP, &CurrentLevel), With<Player>>,
    asset_server: Res<AssetServer>,
) {
    let (req_xp, curr_xp, level) = query.single();
    let xp_percent = if **curr_xp == 0.0 {
        0.0
    } else {
        (**curr_xp / **req_xp) * 100.0
    };
    commands
        .spawn(NodeBundle {
            style: Style {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                flex_direction: FlexDirection::Column,
                ..default()
            },
            ..default()
        })
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
                        height: Val::Percent(15.0),
                        align_self: AlignSelf::End,
                        ..default()
                    },

                    ..default()
                })
                .with_children(|grandchild| {
                    grandchild.spawn(TextBundle::from_section(
                        (**level).to_string(),
                        TextStyle {
                            font: asset_server.load("font/pixel-font.ttf"),
                            font_size: 50.0,
                            color: Color::WHITE,
                        },
                    ));
                });
        });
}
