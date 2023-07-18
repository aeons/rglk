use bevy::prelude::*;
use bracket_pathfinding::prelude::{Algorithm2D, BaseMap};

use crate::components::{Player, Position, Viewshed};
use crate::map::Map;

pub fn player_movement(
    mut player: Query<(&mut Position, &mut Viewshed), With<Player>>,
    keys: Res<Input<KeyCode>>,
    map: Res<Map>,
) {
    let (mut pos, mut viewshed) = player.single_mut();

    if keys.any_just_pressed([KeyCode::Left, KeyCode::H]) {
        try_move_player(&map, &mut pos, &mut viewshed, -1, 0);
    }
    if keys.any_just_pressed([KeyCode::Right, KeyCode::L]) {
        try_move_player(&map, &mut pos, &mut viewshed, 1, 0);
    }
    if keys.any_just_pressed([KeyCode::Up, KeyCode::K]) {
        try_move_player(&map, &mut pos, &mut viewshed, 0, 1);
    }
    if keys.any_just_pressed([KeyCode::Down, KeyCode::J]) {
        try_move_player(&map, &mut pos, &mut viewshed, 0, -1);
    }
}

fn try_move_player(map: &Map, pos: &mut Position, viewshed: &mut Viewshed, x: i32, y: i32) {
    let dst_x = pos.0.x + x;
    let dst_y = pos.0.y + y;
    let dst = (dst_x, dst_y).into();
    let idx = map.point2d_to_index(dst);

    if map.in_bounds(dst) && !map.is_opaque(idx) {
        pos.0.x = dst_x;
        pos.0.y = dst_y;

        viewshed.dirty = true
    }
}
