use bevy_ecs::prelude::*;
use bracket_lib::prelude::*;

use crate::components::{Player, Position, Viewshed};
use crate::map::Map;

pub fn visibility(
    mut map: ResMut<Map>,
    mut query: Query<(&mut Viewshed, &Position, Option<&Player>), Changed<Position>>,
) {
    for (mut viewshed, pos, player) in &mut query {
        viewshed.visible_tiles.clear();
        viewshed.visible_tiles = field_of_view(Point::new(pos.x, pos.y), viewshed.range, &*map);
        viewshed
            .visible_tiles
            .retain(|p| p.x >= 0 && p.x < map.width && p.y >= 0 && p.y < map.height);

        if player.is_some() {
            map.visible_tiles.clear();
            for vis in viewshed.visible_tiles.iter() {
                map.reveal_tile(vis.x, vis.y);
            }
        }
    }
}
