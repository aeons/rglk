use crate::prelude::*;

pub fn melee_combat(
    mut cmd: Commands,
    q_wants_melee: Query<(Entity, &Name, &WantsToMelee, &CombatStats)>,
    mut q_combat_stats: Query<(&Name, &CombatStats, Option<&mut SufferDamage>)>,
) {
    for (entity, name, wants_melee, stats) in q_wants_melee.iter() {
        if stats.hp > 0 {
            if let Ok((target_name, target_stats, suffer_damage)) =
                q_combat_stats.get_mut(wants_melee.target)
            {
                if target_stats.hp > 0 {
                    let damage = i32::max(0, stats.power - target_stats.defense);

                    if damage == 0 {
                        info!("{name} is unable to hurt {target_name}");
                    } else {
                        info!("{name} hits {target_name} for {damage} hp");

                        match suffer_damage {
                            Some(mut suffer_damage) => suffer_damage.amount.push(damage),
                            None => {
                                cmd.entity(wants_melee.target).insert(SufferDamage {
                                    amount: vec![damage],
                                });
                            }
                        };
                    }
                }
            }
        }

        cmd.entity(entity).remove::<WantsToMelee>();
    }
}
