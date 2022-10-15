use bevy::prelude::*;
use bracket_terminal::console;

use crate::components::{CombatStats, Player};
use crate::gamelog::GameLog;

pub fn death(
    mut commands: Commands,
    mut log: ResMut<GameLog>,
    query: Query<(Entity, &CombatStats, Option<&Name>, Option<&Player>)>,
) {
    for (entity, stats, name, player) in &query {
        if stats.hp < 1 {
            if player.is_some() {
                console::log("You are dead");
            } else {
                if let Some(victim_name) = name {
                    log.append(format!("{victim_name} has died"));
                }
                commands.entity(entity).despawn();
            }
        }
    }
}
