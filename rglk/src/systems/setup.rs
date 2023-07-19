use crate::prelude::*;

pub fn setup(mut cmd: Commands, map: Res<Map>, mut global_rng: ResMut<GlobalRng>) {
    cmd.spawn((TerminalBundle::new().with_size([80, 50]), AutoCamera));

    let player_pos = map.rooms[0].center();
    spawn::player(&mut cmd, &player_pos);

    for room in map.rooms.iter().skip(1) {
        spawn::monster(&mut cmd, &room.center(), &mut global_rng);
    }
}
