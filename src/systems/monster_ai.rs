use bevy_ecs::prelude::*;
use bracket_lib::prelude::*;

use crate::components::{Monster, Name, Position, Viewshed};
use crate::map::Map;

pub fn monster_ai(
    player_pos: Res<Point>,
    mut map: ResMut<Map>,
    mut query: Query<(Entity, &Viewshed, &mut Position, &Name), With<Monster>>,
) {
    for (entity, viewshed, mut pos, name) in &mut query {
        let distance = DistanceAlg::Pythagoras.distance2d(Point::new(pos.x, pos.y), *player_pos);
        if distance < 1.5 {
            console::log(format!("{} shouts insults", name.name));
            return;
        }

        if viewshed.visible_tiles.contains(&player_pos) {
            let start = map.xy_idx(pos.x, pos.y);
            let path = a_star_search(start, map.xy_idx(player_pos.x, player_pos.y), &mut *map);

            if path.success && path.steps.len() > 1 {
                let (x, y) = map.idx_xy(path.steps[1]);
                map.blocked_tiles.remove(start);
                pos.x = x;
                pos.y = y;
                map.blocked_tiles.insert(path.steps[1]);
            }
        }
    }
}
