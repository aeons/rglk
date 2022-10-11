use bevy::prelude::*;
use bracket_pathfinding::prelude::*;
use bracket_terminal::console;

use crate::components::{Monster, Player, Position, Viewshed};
use crate::map::Map;
use crate::RunState;

pub fn monster_ai(
    run_state: Res<RunState>,
    mut map: ResMut<Map>,
    player_pos: Query<&Position, (With<Player>, Without<Monster>)>,
    mut monsters: Query<(&Viewshed, &mut Position, &Name), With<Monster>>,
) {
    if *run_state != RunState::MonsterTurn {
        return;
    }

    let player_pos = player_pos.single().as_point();

    for (viewshed, mut pos, name) in &mut monsters {
        let distance = DistanceAlg::Pythagoras.distance2d(pos.as_point(), player_pos);
        if distance < 1.5 {
            console::log(format!("{} shouts insults", name));
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
