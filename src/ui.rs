use bevy::{prelude::*, window::PrimaryWindow};

use crate::{
    cleanup::{self, ExitGame, ExitLevelUpScreen},
    cooldown::InGameTime,
    damage::Health,
    items::{ItemTooltips, ItemType},
    player::{CurrentLevel, CurrentXP, PickUpRadius, Player, RequiredXP},
    GameState, MovementSpeed, MyGameCamera,
};
#[derive(Component)]
pub struct HealthUiSprite;

#[derive(Component)]
pub struct XPBar;

#[derive(Component)]
pub struct LevelUpUi;

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
#[derive(Component, Deref)]
pub struct SelectedItemType(pub ItemType);

pub fn spawn_upgrade_selection_ui(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    ui_query: Query<Entity, With<LevelUpUi>>,
    item_tooltips: Res<ItemTooltips>,
) {
    for entity in &ui_query {
        commands.entity(entity).despawn_recursive();
    }
    commands
        .spawn((
            NodeBundle {
                style: Style {
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    flex_direction: FlexDirection::Row,

                    ..default()
                },
                background_color: Color::rgba(0.0, 0.0, 0.0, 0.7).into(),
                ..default()
            },
            LevelUpUi,
            ExitLevelUpScreen,
        ))
        .with_children(|child| {
            item_tooltips.iter().for_each(|(item_type, tooltip)| {
                child
                    .spawn((
                        ButtonBundle {
                            style: Style {
                                width: Val::Percent(20.0),
                                height: Val::Percent(40.0),
                                margin: UiRect::all(Val::Px(40.0)),
                                flex_direction: FlexDirection::Column,
                                ..default()
                            },
                            background_color: Color::DARK_GRAY.into(),
                            ..default()
                        },
                        SelectedItemType(*item_type),
                    ))
                    .with_children(|text_child| {
                        text_child.spawn(
                            TextBundle::from_section(
                                "Good Item",
                                TextStyle {
                                    font: asset_server.load("font/pixel-font.ttf"),
                                    font_size: 32.0,
                                    color: Color::WHITE,
                                },
                            )
                            .with_text_justify(JustifyText::Center),
                        );
                        text_child.spawn(
                            TextBundle::from_section(
                                *tooltip,
                                TextStyle {
                                    font: asset_server.load("font/pixel-font.ttf"),
                                    font_size: 18.0,
                                    color: Color::WHITE,
                                },
                            )
                            .with_style(Style {
                                margin: UiRect::top(Val::Px(10.0)),
                                ..default()
                            }),
                        );
                    });
            });
        });
}

pub fn handle_selection_cursor(
    interaction_query: Query<
        (&Interaction, &SelectedItemType),
        (Changed<Interaction>, With<Button>),
    >,
    mut player_query: Query<(&mut PickUpRadius, &mut MovementSpeed), With<Player>>,
    mut game_state: ResMut<NextState<GameState>>,
) {
    let (mut pick_up_radius, mut movement_speed) = player_query.single_mut();
    for (interaction, item_type) in &interaction_query {
        match interaction {
            Interaction::Pressed => match **item_type {
                ItemType::PassiveDamageIncrease => {
                    println!("passive damage!");
                    game_state.set(GameState::Running);
                }
                ItemType::PassiveMovementSpeedIncrease => {
                    **movement_speed *= 1.1;
                    println!("Movement speed increased to: {}", **movement_speed);
                    game_state.set(GameState::Running);
                }
                ItemType::PassivePickUpRadiusIncrease => {
                    **pick_up_radius *= 1.1;
                    println!("Pickup radius increased to: {}", **pick_up_radius);
                    game_state.set(GameState::Running);
                }
            },
            Interaction::Hovered => {}
            Interaction::None => {}
        }
    }
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
