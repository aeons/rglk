use crate::components::{Player, Position, Viewshed};
use crate::prelude::*;
use crate::Map;

pub fn visibility(mut q: Query<(&Position, &mut Viewshed, Option<&Player>)>, mut map: ResMut<Map>) {
    for (pos, mut viewshed, player) in q.iter_mut() {
        if !viewshed.dirty {
            continue;
        }

        viewshed.dirty = false;
        viewshed.visible_tiles = field_of_view_set(pos.0, viewshed.range, &*map)
            .into_iter()
            .filter(|&p| map.in_bounds(p))
            .collect();

        if player.is_some() {
            map.visible_tiles.clear();
            for vis in viewshed.visible_tiles.iter() {
                let idx = map.point2d_to_index(*vis);
                map.revealed_tiles.insert(idx);
                map.visible_tiles.insert(idx);
            }
        }
    }
}