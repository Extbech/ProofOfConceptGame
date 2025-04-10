use bevy::ecs::component::Component;
use bevy::prelude::*;

#[derive(Component, Clone, Copy)]
pub enum Item {
    XPOrb,
    Magnet,
    Potion,
    ThorsHammer,
    Coin,
}

impl Item {
    fn image_path(&self) -> &'static str {
        match self {
            Item::XPOrb => "loot/rotating_orbs.png",
            Item::Magnet => "loot/magnet.png",
            Item::Potion => "loot/potion.png",
            Item::ThorsHammer => "loot/hammeritem.png",
            Item::Coin => "loot/coin.png",
        }
    }

    fn atlas_layout(&self) -> Option<(TextureAtlasLayout, usize)> {
        match self {
            Item::XPOrb => Some((
                TextureAtlasLayout::from_grid(UVec2::new(32, 32), 7, 1, None, None),
                0,
            )),
            _ => None,
        }
    }
}

pub const ENEMY_HEIGHT: u32 = 20;
pub const ENEMY_WIDTH: u32 = 22;

pub const PLAYER_HEIGHT: u32 = 20;
pub const PLAYER_WIDTH: u32 = 16;

pub const WIZARD_HEIGHT: u32 = 32;
pub const WIZARD_WIDTH: u32 = 32;

#[derive(Component, Clone, Copy)]
pub enum Character {
    Jotun,
    Warrior,
    Wizard,
}

impl Character {
    fn image_path(&self) -> &'static str {
        match self {
            Character::Jotun => "characters/jotun.png",
            Character::Warrior => "characters/viking.png",
            Character::Wizard => "characters/wizard.png",
        }
    }

    fn atlas_layout(&self) -> Option<(TextureAtlasLayout, usize)> {
        match self {
            Character::Jotun => Some((
                TextureAtlasLayout::from_grid(
                    UVec2::new(ENEMY_WIDTH, ENEMY_HEIGHT),
                    4,
                    1,
                    None,
                    None,
                ),
                0,
            )),
            Character::Warrior => Some((
                TextureAtlasLayout::from_grid(
                    UVec2::new(PLAYER_WIDTH, PLAYER_HEIGHT),
                    4,
                    1,
                    None,
                    None,
                ),
                0,
            )),
            _ => None,
        }
    }
}

#[derive(Component, Clone, Copy)]
pub enum Skill {
    PrimaryAttack,
    OrbJutsu,
    LightningAttack,
    FireBall,
}

impl Skill {
    fn image_path(&self) -> &'static str {
        match self {
            Skill::PrimaryAttack => "skills/axe.png",
            Skill::OrbJutsu => "skills/orb_purple.png",
            Skill::LightningAttack => "skills/lightning-strike.png",
            Skill::FireBall => "skills/fireball.png",
        }
    }

    fn atlas_layout(&self) -> Option<(TextureAtlasLayout, usize)> {
        match self {
            _ => None,
        }
    }
}

#[derive(Component, Clone, Copy)]
pub enum SpriteKind {
    Item(Item),
    Character(Character),
    Skill(Skill),
}

impl SpriteKind {
    fn image_path(&self) -> &'static str {
        match self {
            SpriteKind::Item(x) => x.image_path(),
            SpriteKind::Character(x) => x.image_path(),
            SpriteKind::Skill(x) => x.image_path(),
        }
    }

    fn atlas_layout(&self) -> Option<(TextureAtlasLayout, usize)> {
        match self {
            SpriteKind::Item(x) => x.atlas_layout(),
            SpriteKind::Character(x) => x.atlas_layout(),
            SpriteKind::Skill(x) => x.atlas_layout(),
        }
    }
}

pub fn add_sprite(
    asset_server: Res<AssetServer>,
    mut query: Query<(&SpriteKind, Entity), Without<Sprite>>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
    mut commands: Commands,
) {
    for (&kind, ent) in &mut query {
        let image_path = kind.image_path();
        let texture_atlas = kind.atlas_layout().map(|(layout, index)| TextureAtlas {
            layout: texture_atlas_layouts.add(layout),
            index,
        });
        let sprite = Sprite {
            image: asset_server.load(image_path),
            texture_atlas,
            ..default()
        };
        commands.entity(ent).try_insert(sprite);
    }
}
