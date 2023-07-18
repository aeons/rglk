use crate::components::{Player, Position, Renderable, Viewshed};
use crate::prelude::*;
use crate::Map;

pub fn setup(mut cmd: Commands, map: Res<Map>) {
    cmd.spawn((TerminalBundle::new().with_size([80, 50]), AutoCamera));

    let player_pos = map.rooms[0].center();
    cmd.spawn((
        Player,
        Position(player_pos),
        Renderable('@'.fg(Color::YELLOW)),
        Viewshed::new(8),
    ));
}
