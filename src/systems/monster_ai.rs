use bevy_ecs::prelude::*;
use bracket_lib::prelude::*;

use crate::components::{Monster, Position, Viewshed};

pub fn monster_ai(player_pos: Res<Point>, query: Query<(&Viewshed, &Position), With<Monster>>) {
    for (viewshed, pos) in &query {
        if viewshed.visible_tiles.contains(&*&player_pos) {
            console::log("Monster considers their own existence");
        }
    }
}
