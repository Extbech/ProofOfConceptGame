use bevy::ecs::component::Component;
use bevy::prelude::*;

#[derive(Component, Clone, Copy)]
pub enum ItemSpriteKind {
    XPOrb,
    XPMagnet,
    Potion,
    ThorsHammer,
}

impl ItemSpriteKind {
    fn image_path(&self) -> &'static str {
        match self {
            ItemSpriteKind::XPOrb => "loot/rotating_orbs.png",
            ItemSpriteKind::XPMagnet => "loot/magnet.png",
            ItemSpriteKind::Potion => "loot/potion.png",
            ItemSpriteKind::ThorsHammer => "loot/hammeritem.png",
        }
    }

    fn atlas_layout(&self) -> Option<(TextureAtlasLayout, usize)> {
        match self {
            ItemSpriteKind::XPOrb => Some((
                TextureAtlasLayout::from_grid(UVec2::new(32, 32), 7, 1, None, None),
                0,
            )),
            _ => None,
        }
    }
}

#[derive(Component, Clone, Copy)]
pub enum UISpriteKind {
    Hearts,
}

impl UISpriteKind {
    fn image_path(&self) -> &'static str {
        match self {
            UISpriteKind::Hearts => "ui/heart.png",
        }
    }

    fn atlas_layout(&self) -> Option<(TextureAtlasLayout, usize)> {
        match self {
            UISpriteKind::Hearts => None,
        }
    }
}

pub const ENEMY_HEIGHT: u32 = 20;
pub const ENEMY_WIDTH: u32 = 22;

pub const PLAYER_HEIGHT: u32 = 20;
pub const PLAYER_WIDTH: u32 = 16;

#[derive(Component, Clone, Copy)]
pub enum CharacterSpriteKind {
    Jotun,
    Warrior,
}

impl CharacterSpriteKind {
    fn image_path(&self) -> &'static str {
        match self {
            CharacterSpriteKind::Jotun => "characters/jotun.png",
            CharacterSpriteKind::Warrior => "characters/viking.png",
        }
    }

    fn atlas_layout(&self) -> Option<(TextureAtlasLayout, usize)> {
        match self {
            CharacterSpriteKind::Jotun => Some((
                TextureAtlasLayout::from_grid(
                    UVec2::new(ENEMY_WIDTH, ENEMY_HEIGHT),
                    4,
                    1,
                    None,
                    None,
                ),
                0,
            )),
            CharacterSpriteKind::Warrior => Some((
                TextureAtlasLayout::from_grid(
                    UVec2::new(PLAYER_WIDTH, PLAYER_HEIGHT),
                    4,
                    1,
                    None,
                    None,
                ),
                0,
            )),
        }
    }
}

#[derive(Component, Clone, Copy)]
pub enum SkillSpriteKind {
    PrimaryAttack,
    OrbJutsu,
    LightningAttack,
}

impl SkillSpriteKind {
    fn image_path(&self) -> &'static str {
        match self {
            SkillSpriteKind::PrimaryAttack => "skills/axe.png",
            SkillSpriteKind::OrbJutsu => "skills/orb_purple.png",
            SkillSpriteKind::LightningAttack => "skills/lightning-strike.png",
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
    ItemSpriteKind(ItemSpriteKind),
    UISpriteKind(UISpriteKind),
    CharacterSpriteKind(CharacterSpriteKind),
    SkillSpriteKind(SkillSpriteKind),
}

impl SpriteKind {
    fn image_path(&self) -> &'static str {
        match self {
            SpriteKind::ItemSpriteKind(x) => x.image_path(),
            SpriteKind::UISpriteKind(x) => x.image_path(),
            SpriteKind::CharacterSpriteKind(x) => x.image_path(),
            SpriteKind::SkillSpriteKind(x) => x.image_path(),
        }
    }

    fn atlas_layout(&self) -> Option<(TextureAtlasLayout, usize)> {
        match self {
            SpriteKind::ItemSpriteKind(x) => x.atlas_layout(),
            SpriteKind::UISpriteKind(x) => x.atlas_layout(),
            SpriteKind::CharacterSpriteKind(x) => x.atlas_layout(),
            SpriteKind::SkillSpriteKind(x) => x.atlas_layout(),
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
        commands.entity(ent).insert(sprite);
    }
}
