use bevy::prelude::*;

use crate::components::{CombatStats, Player};

pub fn death(mut commands: Commands, query: Query<(Entity, &CombatStats), Without<Player>>) {
    for (entity, stats) in &query {
        if stats.hp  < 1 {
            commands.entity(entity).despawn();
        }
    }
}