use bevy_ecs::prelude::*;
use bracket_lib::prelude::*;

use crate::components::{Monster, Position, Viewshed};

pub fn monster_ai(query: Query<(&Viewshed, &Position), With<Monster>>) {
    for (viewshed, pos) in &query {
        console::log("Monster considers their own existence");
    }
}
