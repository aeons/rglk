use bevy::prelude::*;

use crate::components::{CombatStats, SufferDamage, WantsToMelee};
use crate::gamelog::GameLog;

pub fn melee_combat(
    mut commands: Commands,
    mut log: ResMut<GameLog>,
    query: Query<(Entity, &WantsToMelee, &Name, &CombatStats)>,
    mut target_query: Query<(Entity, &CombatStats, &Name, Option<&mut SufferDamage>)>,
) {
    for (entity, wants_melee, name, stats) in query.iter() {
        if stats.hp > 0 {
            let (target, target_stats, target_name, target_dmg) = target_query
                .get_mut(wants_melee.target)
                .expect("target of melee attack should exist");

            if target_stats.hp > 0 {
                let damage = i32::max(0, stats.power - target_stats.defense);

                if damage <= 0 {
                    log.append(format!("{} is unable to hurt {}", name, target_name));
                } else {
                    log.append(format!("{} hits {} for {damage} hp", name, target_name));
                    match target_dmg {
                        Some(mut suffering) => suffering.amount.push(damage),
                        None => {
                            commands.entity(target).insert(SufferDamage {
                                amount: vec![damage],
                            });
                        }
                    }
                }
            }
        }

        commands.entity(entity).remove::<WantsToMelee>();
    }
}
