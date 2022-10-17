use bevy::prelude::*;

use crate::components::{BlocksTile, Position};
use crate::map::{Map, self};

pub fn map_indexing(mut map: ResMut<Map>, query: Query<(Entity, &Position, Option<&BlocksTile>)>) {
    map.populate_blocked();
    map.clear_content_index();

    for (entity, pos, blocks_tile) in &query {
        let idx = map::xy_idx(pos.x, pos.y);

        if blocks_tile.is_some() {
            map.blocked_tiles.insert(idx);
        }

        map.tile_content[idx].push(entity);
    }
}
