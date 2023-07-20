use crate::prelude::*;

pub fn damage(mut cmd: Commands, mut query: Query<(Entity, &mut CombatStats, &SufferDamage)>) {
    for (entity, mut stats, damage) in query.iter_mut() {
        stats.hp -= damage.amount.iter().sum::<i32>();
        cmd.entity(entity).remove::<SufferDamage>();
    }
}
