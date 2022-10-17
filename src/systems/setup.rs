use bevy::prelude::*;
use bracket_bevy::prelude::*;

use crate::map::Map;
use crate::spawn;

pub fn setup(mut commands: Commands, map: Res<Map>, rng: Res<RandomNumbers>) {
    let (x, y) = map.rooms[0].center();
    spawn::player(&mut commands, x, y);

    for room in map.rooms.iter().skip(1) {
        spawn::fill_room(&mut commands, &rng, room);
    }
}
