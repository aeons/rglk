use bevy_ecs::prelude::*;
use bracket_lib::prelude::*;

use crate::components::{Position, Viewshed};
use crate::map::Map;

pub fn visibility(map: Res<Map>, mut query: Query<(&mut Viewshed, &Position)>) {
    for (mut viewshed, pos) in &mut query {
        viewshed.visible_tiles.clear();
        viewshed.visible_tiles = field_of_view(Point::new(pos.x, pos.y), viewshed.range, &*map);
        viewshed
            .visible_tiles
            .retain(|p| p.x >= 0 && p.x < map.width && p.y >= 0 && p.y < map.height);
    }
}
