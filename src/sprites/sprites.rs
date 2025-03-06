use bevy::ecs::component::Component;

#[derive(Component)]
pub enum ItemSpriteKind {
    XPOrb,
    XPMagnet,
    Potion,
    Ligthning,
}

#[derive(Component)]
pub enum UISpriteKind {
    Hearts,
}

#[derive(Component)]
pub enum CharacterSpriteKind {
    Jotun,
    WarriorMob,
}

#[derive(Component)]
pub enum SkillSpriteKind {
    PrimaryAttack,
    OrbJutsu,
    LigthningAttack,
}

#[derive(Component)]
pub enum SpriteKind {
    ItemSpriteKind(ItemSpriteKind),
    UISpriteKind(UISpriteKind),
    CharacterSpriteKind(CharacterSpriteKind),
    SkillSpriteKind(SkillSpriteKind),
}
