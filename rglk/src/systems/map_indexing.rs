use crate::prelude::*;

pub fn map_indexing(q: Query<&Position, With<BlocksTile>>, mut map: ResMut<Map>) {
    map.populate_blocked();

    for position in q.iter() {
        let idx = map.point2d_to_index(**position);
        map.blocked.insert(idx);
    }
}
