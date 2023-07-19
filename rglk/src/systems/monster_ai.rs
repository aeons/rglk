use crate::prelude::*;

pub fn monster_ai(
    mut q_monsters: Query<(&Name, &mut Position, &mut Viewshed), (With<Monster>, Without<Player>)>,
    q_player: Query<&Position, With<Player>>,
    map: Res<Map>,
) {
    let player_pos = q_player.single();

    for (name, mut pos, mut viewshed) in q_monsters.iter_mut() {
        if viewshed.visible_tiles.contains(&player_pos) {
            println!("{name} shouts insults!");

            let path = a_star_search(
                map.point2d_to_index(**pos),
                map.point2d_to_index(**player_pos),
                &*map,
            );

            if path.success {
                if let Some(dst_idx) = path.steps.get(1) {
                    pos.0 = map.index_to_point2d(*dst_idx);
                    viewshed.dirty = true;
                }
            }
        }
    }
}
