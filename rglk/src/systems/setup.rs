use crate::prelude::*;

pub fn setup(mut cmd: Commands, map: Res<Map>, mut global_rng: ResMut<GlobalRng>) {
    debug!("running");

    cmd.spawn((TerminalBundle::new().with_size([80, 50]), AutoCamera));

    let player_pos = map.rooms[0].center();
    let player = spawn::player(&mut cmd, &player_pos);
    cmd.insert_resource(PlayerEntity(player));

    for (index, room) in map.rooms.iter().skip(1).enumerate() {
        spawn::monster(&mut cmd, &room.center(), index + 1, &mut global_rng);
    }
}
