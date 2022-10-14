use bevy::prelude::*;

use crate::components::{CombatStats, SufferDamage};

pub fn damage(mut commands: Commands, mut query: Query<(Entity, &mut CombatStats, &SufferDamage)>) {
    for (entity, mut stats, suffering) in &mut query {
        stats.hp -= suffering.amount.iter().sum::<i32>();

        commands.entity(entity).remove::<SufferDamage>();
    }
}
