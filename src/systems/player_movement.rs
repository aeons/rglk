use bevy::prelude::*;

use crate::components::{CombatStats, Player, Position, WantsToMelee};
use crate::map::Map;
use crate::player::{player_input, PlayerInput};
use crate::RunState;

pub fn player_movement(
    mut commands: Commands,
    keyboard: Res<Input<KeyCode>>,
    mut run_state: ResMut<RunState>,
    map: Res<Map>,
    mut player_query: Query<(Entity, &mut Position), With<Player>>,
    target_query: Query<&CombatStats>,
) {
    if *run_state != RunState::AwaitingInput {
        return;
    }

    let (delta_x, delta_y) = match player_input(&keyboard) {
        PlayerInput::Idle => {
            *run_state = RunState::AwaitingInput;
            return;
        }
        PlayerInput::Delta { x, y } => {
            *run_state = RunState::PlayerTurn;
            (x, y)
        }
    };

    let (player, mut pos) = player_query.single_mut();
    let dst_x = pos.x + delta_x;
    let dst_y = pos.y + delta_y;
    let dst = map.xy_idx(dst_x, dst_y);

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
}
