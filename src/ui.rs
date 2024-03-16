use bevy::prelude::*;

use crate::player::{CurrentLevel, CurrentXP, Player, RequiredXP};

pub fn spawn_ui(mut commands: Commands, asset_server: Res<AssetServer>) {
    todo!()
}

pub fn update_xp_bar(
    mut commands: Commands,
    query: Query<(&RequiredXP, &CurrentXP), With<Player>>,
) {
    let (req_xp, curr_xp) = query.single();
    let xp_percent = if **curr_xp == 0.0 {
        0.0
    } else {
        (**curr_xp / **req_xp) * 100.0
    };
    commands
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
        .with_children(|child| {
            child.spawn(NodeBundle {
                style: Style {
                    width: Val::Percent(xp_percent),
                    height: Val::Percent(100.0),
                    ..default()
                },
                background_color: Color::CYAN.into(),
                ..default()
            });
        });
}
