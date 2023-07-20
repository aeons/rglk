use crate::prelude::*;

pub fn delete_the_dead(mut cmd: Commands, query: Query<(Entity, &Name, &CombatStats, Option<&Player>)>) {
    for (entity,name, stats, player) in query.iter() {
        info!("{name}: {stats:?}");
        if stats.hp < 1 {
            if player.is_some() {
                info!("You are dead!");
            } else {
                cmd.entity(entity).despawn();
            }
        }
    }
}
