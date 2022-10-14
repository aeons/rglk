use bevy::prelude::*;
use bracket_pathfinding::prelude::*;

use crate::components::{Monster, Player, Position, Viewshed, WantsToMelee};
use crate::map::Map;
use crate::RunState;

pub fn monster_ai(
    mut commands: Commands,
    run_state: Res<RunState>,
    mut map: ResMut<Map>,
    player_query: Query<(Entity, &Position), (With<Player>, Without<Monster>)>,
    mut monsters_query: Query<(Entity, &Viewshed, &mut Position), With<Monster>>,
) {
    if *run_state != RunState::MonsterTurn {
        return;
    }

    let (player, player_pos) = player_query.single();
    let player_pos = player_pos.as_point();

    for (monster, viewshed, mut pos) in &mut monsters_query {
        let distance = DistanceAlg::Pythagoras.distance2d(pos.as_point(), player_pos);
        if distance < 1.5 {
            commands
                .entity(monster)
                .insert(WantsToMelee { target: player });
        } else if viewshed.visible_tiles.contains(&player_pos) {
            let start = map.xy_idx(pos.x, pos.y);
            let path = a_star_search(start, map.xy_idx(player_pos.x, player_pos.y), &*map);

            if path.success && path.steps.len() > 1 {
                let (x, y) = map.idx_xy(path.steps[1]);
                pos.x = x;
                pos.y = y;
                map.blocked_tiles.remove(start);
                map.blocked_tiles.insert(path.steps[1]);
            }
        }
    }
}
