use bevy::prelude::*;

use crate::components::{InBackpack, Item, Player, Position, WantsToPickupItem};
use crate::gamelog::GameLog;

pub fn item_collection(
    mut commands: Commands,
    mut log: ResMut<GameLog>,
    query: Query<&mut WantsToPickupItem>,
    player: Query<Entity, With<Player>>,
    item_name: Query<&Name, With<Item>>,
) {
    let player = player.single();

    for pickup in &query {
        commands
            .entity(pickup.item)
            .remove::<Position>()
            .insert(InBackpack {
                owner: pickup.collected_by,
            });

        if pickup.collected_by == player {
            let item = item_name.get(pickup.item).expect("item should exist");
            log.append(format!("You pick up the {item}"));
        }
    }
}
