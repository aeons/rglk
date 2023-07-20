use crate::prelude::*;

pub fn delete_the_dead(mut cmd: Commands, query: Query<(Entity, &CombatStats)>) {
    for (entity, stats) in query.iter() {
        if stats.hp < 1 {
            cmd.entity(entity).despawn();
        }
    }
}
