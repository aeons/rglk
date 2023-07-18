use crate::components::{Player, Position, Renderable, Viewshed};
use crate::prelude::*;

pub fn player(cmd: &mut Commands, position: Point) {
    cmd.spawn((
        Player,
        Position(position),
        Renderable('@'.fg(Color::YELLOW)),
        Viewshed::new(8),
    ));
}

pub fn monster(cmd: &mut Commands, position: Point) {
    cmd.spawn((
        Position(position),
        Renderable('g'.fg(Color::RED)),
        Viewshed::new(8),
    ));
}
