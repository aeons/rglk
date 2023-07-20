use crate::prelude::*;

pub fn damage(mut query: Query<(&mut CombatStats, &SufferDamage)>) {
    for (mut stats, damage) in query.iter_mut() {
        stats.hp -= damage.amount.iter().sum::<i32>();
    }
}
