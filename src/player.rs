use std::cmp::{max, min};

use bevy_ecs::prelude::*;
use bracket_lib::prelude::*;

use crate::components::{Player, Position};
use crate::{Map, State};

pub fn try_move_player(delta_x: i32, delta_y: i32, world: &mut World) {
    world.resource_scope(|world: &mut World, map: Mut<Map>| {
        for (mut pos, _) in world
            .query::<(&mut Position, With<Player>)>()
            .iter_mut(world)
        {
            if map.is_walkable(pos.x + delta_x, pos.y + delta_y) {
                pos.x = min(79, max(0, pos.x + delta_x));
                pos.y = min(49, max(0, pos.y + delta_y));
            }
        }
    })
}

pub fn player_input(gs: &mut State, ctx: &mut BTerm) {
    match ctx.key {
        None => {}
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
            _ => {}
        },
    }
}
