use bevy::{app::Plugin, color::palettes::css, prelude::*};

use crate::{cleanup, tools::damage_tracking::DamageTracker, GameState};
pub struct LossPlugin;

impl Plugin for LossPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Loss), spawn_upgrade_loss_ui)
            // .add_systems(
            //     Update,
            //     (handle_selection_cursor)
            //         .run_if(in_state(AppState::InGame))
            //         .run_if(in_state(GameState::LevelUp)),
            // )
            .add_systems(
                OnExit(GameState::Loss),
                (cleanup::<cleanup::ExitLossScreen>,),
            );
    }
}

#[derive(Component)]
pub struct LossUi;

pub fn spawn_upgrade_loss_ui(
    mut commands: Commands,
    ui_query: Query<Entity, With<LossUi>>,
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
                ..default()
            },
            BackgroundColor(Color::Srgba(Srgba {
                red: 0.0,
                green: 0.0,
                blue: 0.0,
                alpha: 0.7,
            })),
            LossUi,
            cleanup::ExitLevelUpScreen,
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
                        TextLayout::new_with_justify(JustifyText::Center),
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
                                TextLayout::new_with_justify(JustifyText::Center),
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
                                    TextLayout::new_with_justify(JustifyText::Center),
                                ));
                            });
                        });
                });
        });
}

// pub fn handle_selection_cursor(
//     mut commands: Commands,
//     mut interaction_query: Query<
//         (&Interaction, &SelectedItemType, &mut BackgroundColor),
//         (Changed<Interaction>, With<Button>),
//     >,
//     mut player_query: Query<
//         (
//             &mut XpPickUpRadius,
//             &mut MovementSpeed,
//             &mut PlayerDamage,
//             &mut Health,
//             &mut MaxHealth,
//             Entity,
//         ),
//         With<Player>,
//     >,
//     mut orb_query: Query<Entity, With<OrbitalRadius>>,
//     mut game_state: ResMut<NextState<GameState>>,
// ) {
//     let (
//         mut pick_up_radius,
//         mut movement_speed,
//         mut player_damage,
//         mut health,
//         mut max_health,
//         player_entity,
//     ) = player_query.single_mut();
//     for (interaction, item_type, mut background_color) in &mut interaction_query {
//         match interaction {
//             Interaction::Pressed => {
//                 match **item_type {
//                     SkillType::PassiveDamageIncrease => {
//                         **player_damage += 1;
//                         println!("damage increase to: {}", **player_damage);
//                     }
//                     SkillType::PassiveMovementSpeedIncrease => {
//                         **movement_speed *= 1.1;
//                         println!("Movement speed increased to: {}", **movement_speed);
//                     }
//                     SkillType::PassivePickUpRadiusIncrease => {
//                         **pick_up_radius *= 1.1;
//                         println!("Pickup radius increased to: {}", **pick_up_radius);
//                     }
//                     SkillType::ActiveOrbitingOrb => {
//                         println!("Orb jutsu");
//                         spawn_new_orb(&mut commands, player_entity, &mut orb_query);
//                     }
//                     SkillType::PassiveHealthIncrease => {
//                         **health += 1;
//                         **max_health += 1;
//                         println!("health increased to: {}", **health);
//                     }
//                     SkillType::ActiveThorLightning => {
//                         enable_thors_lightning_skill(&mut commands, player_entity);
//                     }
//                 }
//                 game_state.set(GameState::Running);
//             }
//             Interaction::Hovered => {
//                 *background_color = css::GRAY.into();
//             }
//             Interaction::None => *background_color = css::DARK_GRAY.into(),
//         }
//     }
// }
