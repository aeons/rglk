use crate::prelude::*;

pub fn monster_ai(
    q_monsters: Query<(&Name, &Position, &Viewshed), With<Monster>>,
    q_player: Query<&Position, With<Player>>,
) {
    let player_pos = q_player.single();

    for (name, pos, viewshed) in q_monsters.iter() {
        if viewshed.visible_tiles.contains(&player_pos) {
            println!("{name} shouts insults!")
        }
    }
}
