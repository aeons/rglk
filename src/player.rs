use std::cmp::{max, min};

use bevy_ecs::prelude::*;
use bracket_lib::prelude::*;

use crate::{MovePlayerState, RunState, State};

pub fn try_move_player(delta_x: i32, delta_y: i32, world: &mut World) {
    world.resource_scope(|mut world, mut mapp: Mut<MovePlayerState>| {
        let (map, mut player_pos, mut query) = mapp.state.get_mut(&mut world);

        for mut pos in query.iter_mut() {
            if map.is_walkable(pos.x + delta_x, pos.y + delta_y) {
                pos.x = min(79, max(0, pos.x + delta_x));
                pos.y = min(49, max(0, pos.y + delta_y));

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
            _ => return RunState::Paused,
        },
    }
    RunState::Running
}
