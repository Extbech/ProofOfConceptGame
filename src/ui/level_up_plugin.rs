use bevy::{app::Plugin, color::palettes::css, prelude::*};
use rand::prelude::*;

use crate::{
    characters::player::{MaxHealth, Player, PlayerDamage, XpPickUpRadius},
    cleanup,
    mechanics::{damage::Health, projectiles::OrbitalRadius},
    skills::{
        skills::{enable_thors_lightning_skill, spawn_new_orb},
        skills_tooltips::{SkillTooltips, SkillType},
    },
    AppState, GameRng, GameState, MovementSpeed,
};
pub struct LevelUpPlugin;

impl Plugin for LevelUpPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::LevelUp), spawn_upgrade_selection_ui)
            .add_systems(
                Update,
                (handle_selection_cursor)
                    .run_if(in_state(AppState::InGame))
                    .run_if(in_state(GameState::LevelUp)),
            )
            .add_systems(
                OnExit(GameState::LevelUp),
                (cleanup::<cleanup::ExitLevelUpScreen>,),
            );
    }
}

#[derive(Component, Deref)]
pub struct SelectedItemType(pub SkillType);

#[derive(Component)]
pub struct LevelUpUi;

pub fn spawn_upgrade_selection_ui(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    ui_query: Query<Entity, With<LevelUpUi>>,
    item_tooltips: ResMut<SkillTooltips>,
    mut rng: ResMut<GameRng>,
) {
    for entity in &ui_query {
        commands.entity(entity).despawn_recursive();
    }
    let mut generated_indexes: Vec<usize> = Vec::new();
    for _ in 0..3 {
        let mut rand_index = rng.gen_range(0..item_tooltips.len() - generated_indexes.len());
        for gen_index in &generated_indexes {
            if *gen_index <= rand_index {
                rand_index += 1;
            } else {
                break;
            }
        }
        generated_indexes.push(rand_index);
        generated_indexes.sort();
    }
    generated_indexes.shuffle(&mut **rng);
    println!(
        "The random generated skill indexes are: {:?}",
        generated_indexes,
    );
    let randomly_skill_selection: Vec<(SkillType, &'static str, &'static str)> = generated_indexes
        .iter()
        .map(|index| item_tooltips[*index])
        .collect();
    commands
        .spawn((
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                flex_direction: FlexDirection::Row,
                ..default()
            },
            BackgroundColor(Color::Srgba(Srgba {
                red: 0.0,
                green: 0.0,
                blue: 0.0,
                alpha: 0.7,
            })),
            LevelUpUi,
            cleanup::ExitLevelUpScreen,
        ))
        .with_children(|child| {
            randomly_skill_selection
                .iter()
                .for_each(|(item_type, title, description)| {
                    child
                        .spawn((
                            Node {
                                width: Val::Percent(20.0),
                                height: Val::Percent(40.0),
                                align_items: AlignItems::Center,
                                margin: UiRect::all(Val::Px(40.0)),
                                flex_direction: FlexDirection::Column,
                                ..default()
                            },
                            Button,
                            BackgroundColor(css::DARK_GRAY.into()),
                            SelectedItemType(*item_type),
                        ))
                        .with_children(|text_child| {
                            text_child.spawn((
                                Text::new(*title),
                                TextFont {
                                    font: asset_server.load("font/pixel-font.ttf"),
                                    font_size: 28.0,
                                    ..Default::default()
                                },
                                TextColor(Color::WHITE),
                            ));
                            text_child.spawn((
                                Text::new(*description),
                                TextFont {
                                    font: asset_server.load("font/pixel-font.ttf"),
                                    font_size: 18.0,
                                    ..Default::default()
                                },
                                TextColor(Color::WHITE),
                            ));
                        });
                });
        });
}

pub fn handle_selection_cursor(
    mut commands: Commands,
    mut interaction_query: Query<
        (&Interaction, &SelectedItemType, &mut BackgroundColor),
        (Changed<Interaction>, With<Button>),
    >,
    mut player_query: Query<
        (
            &mut XpPickUpRadius,
            &mut MovementSpeed,
            &mut PlayerDamage,
            &mut Health,
            &mut MaxHealth,
            Entity,
        ),
        With<Player>,
    >,
    mut orb_query: Query<Entity, With<OrbitalRadius>>,
    mut game_state: ResMut<NextState<GameState>>,
) {
    let (
        mut pick_up_radius,
        mut movement_speed,
        mut player_damage,
        mut health,
        mut max_health,
        player_entity,
    ) = player_query.single_mut();
    for (interaction, item_type, mut background_color) in &mut interaction_query {
        match interaction {
            Interaction::Pressed => {
                match **item_type {
                    SkillType::PassiveDamageIncrease => {
                        **player_damage += 1;
                        println!("damage increase to: {}", **player_damage);
                    }
                    SkillType::PassiveMovementSpeedIncrease => {
                        **movement_speed *= 1.1;
                        println!("Movement speed increased to: {}", **movement_speed);
                    }
                    SkillType::PassivePickUpRadiusIncrease => {
                        **pick_up_radius *= 1.1;
                        println!("Pickup radius increased to: {}", **pick_up_radius);
                    }
                    SkillType::ActiveOrbitingOrb => {
                        println!("Orb jutsu");
                        spawn_new_orb(&mut commands, player_entity, &mut orb_query);
                    }
                    SkillType::PassiveHealthIncrease => {
                        **health += 1;
                        **max_health += 1;
                        println!("health increased to: {}", **health);
                    }
                    SkillType::ActiveThorLightning => {
                        enable_thors_lightning_skill(&mut commands, player_entity);
                    }
                }
                game_state.set(GameState::Running);
            }
            Interaction::Hovered => {
                *background_color = css::GRAY.into();
            }
            Interaction::None => *background_color = css::DARK_GRAY.into(),
        }
    }
}
