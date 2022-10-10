use bevy_ecs::prelude::*;

use crate::components::{CombatStats, Name, WantsToMelee};

pub fn melee_combat(query: Query<(&WantsToMelee, &Name, &CombatStats)>) {
    for (wants_melee, name, stats) in query.iter() {
        
    }
}
