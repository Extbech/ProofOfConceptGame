use bevy::{app::Plugin, prelude::*};

use crate::{
    cleanup, items::{ItemTooltips, ItemType}, player::{PickUpRadius, Player, PlayerDamage}, AppState, GameState, MovementSpeed
};
pub struct LevelUpPlugin;

impl Plugin for LevelUpPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (spawn_upgrade_selection_ui, handle_selection_cursor)
                .run_if(in_state(AppState::InGame))
                .run_if(in_state(GameState::LevelUp)),
        )
        .add_systems(
            OnExit(GameState::LevelUp),
            (cleanup::<cleanup::ExitLevelUpScreen>,),
        );
    }
}

#[derive(Component)]
pub struct LevelUpUi;

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
            cleanup::ExitLevelUpScreen,
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
    mut player_query: Query<
        (&mut PickUpRadius, &mut MovementSpeed, &mut PlayerDamage),
        With<Player>,
    >,
    mut game_state: ResMut<NextState<GameState>>,
) {
    let (mut pick_up_radius, mut movement_speed, mut player_damage) = player_query.single_mut();
    for (interaction, item_type) in &interaction_query {
        match interaction {
            Interaction::Pressed => match **item_type {
                ItemType::PassiveDamageIncrease => {
                    **player_damage += 1;
                    println!("damage increase to: {}", **player_damage);
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