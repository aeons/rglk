use crate::prelude::*;

pub fn map_indexing(q: Query<(Entity, &Position, Option<&BlocksTile>)>, mut map: ResMut<Map>) {
    map.populate_blocked();
    map.clear_content();

    for (entity, position, blocks_tile) in q.iter() {
        let idx = map.point2d_to_index(**position);

        // If the entity blocks a tile, update the blocked list
        if blocks_tile.is_some() {
            map.blocked.insert(idx);
        }

        map.tile_content[idx].push(entity);
    }
}
