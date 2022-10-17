use bevy::prelude::*;

use crate::components::{InBackpack, Position, WantsToPickupItem, Player};
use crate::gamelog::GameLog;

pub fn item_collection(
    mut commands: Commands,
    log: ResMut<GameLog>,
    query: Query<(&mut WantsToPickupItem)>,
    player: Query<Entity, With<Player>>,
) {
    let player = player.single_mut();

    for pickup in &query {
        commands
            .entity(pickup.item)
            .remove::<Position>()
            .insert(InBackpack {
                owner: pickup.collected_by,
            });

        if pickup.collected_by == player {
            log.append(format!("You pick up the {}"));
        }
    }
}
