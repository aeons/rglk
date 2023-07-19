use crate::prelude::*;

pub fn monster_ai(
    mut q_monsters: Query<(&Name, &mut Position, &mut Viewshed), (With<Monster>, Without<Player>)>,
    q_player: Query<&Position, With<Player>>,
    mut map: ResMut<Map>,
) {
    let player_pos = q_player.single();

    for (name, mut pos, mut viewshed) in q_monsters.iter_mut() {
        let distance = DistanceAlg::Pythagoras.distance2d(**pos, **player_pos);
        if distance < 1.5 {
            info!("{name} shouts insults!");
        } else if viewshed.visible_tiles.contains(&player_pos) {
            let idx = map.point2d_to_index(**pos);
            let path = a_star_search(idx, map.point2d_to_index(**player_pos), &*map);

            if path.success {
                if let Some(dst_idx) = path.steps.get(1) {
                    // Update blocked tiles
                    map.blocked.set(idx, false);
                    map.blocked.set(*dst_idx, true);
                    // Move monster
                    pos.0 = map.index_to_point2d(*dst_idx);
                    viewshed.dirty = true;
                }
            }
        }
    }
}
