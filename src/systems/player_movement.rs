use bevy::prelude::*;

use crate::components::{CombatStats, Player, Position, WantsToMelee};
use crate::map::{self, Map};
use crate::player::PlayerInput;

pub fn player_movement(
    mut commands: Commands,
    map: Res<Map>,
    mut query: Query<
        (Entity, Option<&PlayerInput>, &mut Position),
        (With<Player>, With<PlayerInput>),
    >,
    target_query: Query<&CombatStats>,
) {
    if query.is_empty() {
        return;
    }

    let (player, input, mut pos) = query.single_mut();

    if let Some(PlayerInput::Movement { x, y }) = input {
        let dst_x = pos.x + x;
        let dst_y = pos.y + y;
        let dst = map::xy_idx(dst_x, dst_y);

        for potential_target in map.tile_content[dst].iter() {
            if target_query.get(*potential_target).is_ok() {
                commands.entity(player).insert(WantsToMelee {
                    target: *potential_target,
                });
                return;
            }
        }

        if map.is_valid_exit(dst_x, dst_y) {
            pos.x = dst_x;
            pos.y = dst_y;
        }

        commands.entity(player).remove::<PlayerInput>();
    }
}
