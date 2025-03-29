use bevy::prelude::*;

// Skills component
#[derive(Component, Debug, Clone)]
pub struct Skills {
    pub mining: f32,        // Effectiveness at mining
    pub woodcutting: f32,   // Effectiveness at cutting trees
    pub harvesting: f32,    // Effectiveness at harvesting resources
    #[allow(dead_code)]
    pub combat: f32,        // Combat effectiveness
    pub construction: f32,  // Building construction speed
    #[allow(dead_code)]
    pub crafting: f32,      // Item crafting quality
}

impl Default for Skills {
    fn default() -> Self {
        Self {
            mining: 1.0,
            woodcutting: 1.0,
            harvesting: 1.0,
            combat: 1.0,
            construction: 1.0,
            crafting: 1.0,
        }
    }
}

// Experience gain component
#[derive(Component, Debug)]
pub struct SkillProgression {
    pub mining_xp: f32,
    pub woodcutting_xp: f32,
    pub harvesting_xp: f32,
    #[allow(dead_code)]
    pub combat_xp: f32,
    pub construction_xp: f32,
    #[allow(dead_code)]
    pub crafting_xp: f32,
}

impl Default for SkillProgression {
    fn default() -> Self {
        Self {
            mining_xp: 0.0,
            woodcutting_xp: 0.0,
            harvesting_xp: 0.0,
            combat_xp: 0.0,
            construction_xp: 0.0,
            crafting_xp: 0.0,
        }
    }
}
