use bevy::{
    ecs::{component::Component, resource::Resource},
    prelude::Deref,
};

/// Enum representing the various skills upgrades you can choose from.
#[derive(Component, PartialEq, Eq, Hash, Copy, Clone)]
pub enum SkillType {
    PassiveDamageIncrease,
    PassiveMovementSpeedIncrease,
    PassivePickUpRadiusIncrease,
    PassiveHealthIncrease,
    ActiveOrbitingOrb,
    ActiveThorLightning,
}

/// Resource containing information about the various skill upgrades.
#[derive(Resource, Deref)]
pub struct SkillTooltips(pub [(SkillType, &'static str, &'static str); 6]);

impl Default for SkillTooltips {
    /// Default implentation for `SkillTooltips`
    /// Update this to make changes to whats rendered in the UI during level skill selection.
    fn default() -> Self {
        SkillTooltips([
            (
                SkillType::PassiveDamageIncrease,
                "Damage",
                "Increase All Damage Done By 10%.",
            ),
            (
                SkillType::PassiveMovementSpeedIncrease,
                "Movement Speed",
                "Increase Movement Speed By 10%.",
            ),
            (
                SkillType::PassivePickUpRadiusIncrease,
                "Pick Up Radius",
                "Increase Pickup Radius By 10%.",
            ),
            (
                SkillType::ActiveOrbitingOrb,
                "Orb Jutsu",
                "Spawn an orbiting orb that damages enemies it comes in contact with.",
            ),
            (
                SkillType::PassiveHealthIncrease,
                "Vitality",
                "Increase max health by 1.",
            ),
            (
                SkillType::ActiveThorLightning,
                "Thor's Lightning",
                "Lightning randomly strikes nearby enemies.",
            ),
        ])
    }
}
