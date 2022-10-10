use std::cmp::{max, min};

use bevy_ecs::prelude::*;
use bracket_lib::prelude::*;

use crate::components::{WantsToMelee};
use crate::{MovePlayerState, RunState, State};

pub fn try_move_player(delta_x: i32, delta_y: i32, world: &mut World) {
    world.resource_scope(|mut world, mut mapp: Mut<MovePlayerState>| {
        let (mut commands, map, mut player_pos, mut query, combat_stats) =
            mapp.state.get_mut(&mut world);

        for (player, mut pos) in query.iter_mut() {
            let dst = map.xy_idx(pos.x + delta_x, pos.y + delta_y);

            for potential_target in map.tile_content[dst].iter() {
                let target = combat_stats.get(*potential_target);
                if let Ok(_) = target {
                    commands.entity(player).insert(WantsToMelee {
                        target: *potential_target,
                    });
                    return;
                }
            }

            if !map.blocked_tiles.contains(dst) {
                pos.x = min(map.width - 1, max(0, pos.x + delta_x));
                pos.y = min(map.height - 1, max(0, pos.y + delta_y));

                player_pos.x = pos.x;
                player_pos.y = pos.y;
            }
        }
    })
}

pub fn player_input(gs: &mut State, ctx: &mut BTerm) -> RunState {
    match ctx.key {
        None => return RunState::Paused,
        Some(key) => match key {
            VirtualKeyCode::Left | VirtualKeyCode::Numpad4 | VirtualKeyCode::H => {
                try_move_player(-1, 0, &mut gs.world)
            }
            VirtualKeyCode::Right | VirtualKeyCode::Numpad6 | VirtualKeyCode::L => {
                try_move_player(1, 0, &mut gs.world)
            }
            VirtualKeyCode::Up | VirtualKeyCode::Numpad8 | VirtualKeyCode::K => {
                try_move_player(0, -1, &mut gs.world)
            }
            VirtualKeyCode::Down | VirtualKeyCode::Numpad2 | VirtualKeyCode::J => {
                try_move_player(0, 1, &mut gs.world)
            }

            // Diagonals
            VirtualKeyCode::Numpad9 | VirtualKeyCode::Y => try_move_player(-1, -1, &mut gs.world),

            VirtualKeyCode::Numpad7 | VirtualKeyCode::U => try_move_player(1, -1, &mut gs.world),

            VirtualKeyCode::Numpad3 | VirtualKeyCode::N => try_move_player(1, 1, &mut gs.world),

            VirtualKeyCode::Numpad1 | VirtualKeyCode::B => try_move_player(-1, 1, &mut gs.world),

            _ => return RunState::Paused,
        },
    }
    RunState::Running
}
