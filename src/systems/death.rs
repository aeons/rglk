use bevy::prelude::*;
use bracket_terminal::console;

use crate::components::{CombatStats, Player};

pub fn death(mut commands: Commands, query: Query<(Entity, &CombatStats, Option<&Player>)>) {
    for (entity, stats, player) in &query {
        if stats.hp < 1 {
            if player.is_some() {
                console::log("You are dead");
            } else {
                commands.entity(entity).despawn();
            }
        }
    }
}
