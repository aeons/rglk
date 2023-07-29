use crate::prelude::*;

pub fn delete_the_dead(mut cmd: Commands, query: Query<(Entity, &CombatStats, Option<&Player>)>) {
    debug!("running");

    for (entity, stats, player) in query.iter() {
        if stats.hp < 1 {
            if player.is_some() {
                info!("You are dead!");
            } else {
                cmd.entity(entity).despawn();
            }
        }
    }
}
