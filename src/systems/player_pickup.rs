use bevy::prelude::*;

use crate::components::{Item, Player, Position, WantsToPickupItem};
use crate::gamelog::GameLog;
use crate::player::PlayerInput;

pub fn player_pickup(
    mut commands: Commands,
    mut gamelog: ResMut<GameLog>,
    mut query: Query<(Entity, Option<&PlayerInput>, &Position), (With<Player>, With<PlayerInput>)>,
    items: Query<(Entity, &Position), With<Item>>,
) {
    if query.is_empty() {
        return;
    }

    let (player, input, player_pos) = query.single_mut();

    match input {
        Some(PlayerInput::PickupItem) => {}
        _ => return,
    }

    let item = items.iter().find_map(|(item, item_pos)| {
        if item_pos == player_pos {
            Some(item)
        } else {
            None
        }
    });

    if let Some(item) = item {
        commands.entity(player).insert(WantsToPickupItem {
            item,
            collected_by: player,
        });
    } else {
        gamelog.append("There is nothing here to pick up.");
    }

    commands.entity(player).remove::<PlayerInput>();
}
