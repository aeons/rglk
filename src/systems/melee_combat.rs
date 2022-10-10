use bevy_ecs::prelude::*;
use bracket_lib::terminal::console;

use crate::components::{CombatStats, Name, WantsToMelee};

pub fn melee_combat(
    query: Query<(&WantsToMelee, &Name, &CombatStats)>,
    stats_query: Query<&CombatStats>,
    name_query: Query<&Name>,
) {
    for (wants_melee, name, stats) in query.iter() {
        console::log(format!("{:?} - {:?} - {:?}", wants_melee, name, stats));
        if stats.hp > 0 {
            let target_stats = stats_query
                .get(wants_melee.target)
                .expect("target of melee attack should exist");

            if target_stats.hp > 0 {
                let target_name = name_query
                    .get(wants_melee.target)
                    .expect("target of melee attack should exist");

                let damage = i32::max(0, stats.power - target_stats.defense);

                if damage <= 0 {
                    console::log(format!(
                        "{} is unable to hurt {}",
                        name.name, target_name.name
                    ));
                } else {
                    console::log(format!(
                        "{} hits {} for {damage} hp",
                        name.name, target_name.name
                    ));
                }
            }
        }
    }
}
