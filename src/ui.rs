use bevy::{ecs::query, prelude::*, window::PrimaryWindow};

use crate::{
    cleanup,
    player::{CurrentLevel, CurrentXP, Player, RequiredXP},
    Health, MyGameCamera,
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
            NodeBundle {
                style: Style {
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    flex_direction: FlexDirection::ColumnReverse,
                    ..default()
                },
                ..default()
            },
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

pub fn spawn_health_ui(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    q_window: Query<&Window, With<PrimaryWindow>>,
    q_camera: Query<&Transform, With<MyGameCamera>>,
    q_player: Query<&Health, With<Player>>,
) {
    let window = q_window.single();
    let camera_pos = q_camera.single().translation;
    let health = q_player.single();
    let texture_handle: Handle<Image> = asset_server.load("ui/heart-pixel.png");
    for i in 0..**health {
        commands.spawn((
            SpriteBundle {
                transform: Transform::from_xyz(
                    (camera_pos.x - (window.width() / 2.0)) + 50.0 + (i as f32 * 60.0),
                    (camera_pos.y + (window.height() / 2.0)) - 50.0,
                    2.0,
                ),
                texture: texture_handle.clone(),
                ..default()
            },
            HealthUiSprite,
            cleanup::ExitGame,
        ));
    }
}
pub fn update_health_ui(
    mut commands: Commands,
    q_window: Query<&Window, With<PrimaryWindow>>,
    q_camera: Query<&Transform, (With<MyGameCamera>, Without<HealthUiSprite>)>,
    mut q_health: Query<(&mut Transform, Entity), (With<HealthUiSprite>, Without<Player>)>,
    q_player_health: Query<&Health, (With<Player>, Without<HealthUiSprite>)>,
) {
    let window = q_window.single();
    let camera_pos = q_camera.single().translation;
    let player_health = q_player_health.single();
    for (i, (mut transform, entity)) in q_health.iter_mut().enumerate() {
        if i >= **player_health as usize {
            commands.entity(entity).despawn();
        } else {
            transform.translation.x =
                (camera_pos.x - (window.width() / 2.0)) + 50.0 + (i as f32 * 60.0);
            transform.translation.y = (camera_pos.y + (window.height() / 2.0)) - 50.0;
        }
    }
}
pub fn spawn_upgrade_selection_ui(mut commands: Commands, asset_server: Res<AssetServer>) {
    todo!()
}
