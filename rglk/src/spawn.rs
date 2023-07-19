use crate::prelude::*;

pub fn player(cmd: &mut Commands, position: &Point) {
    cmd.spawn((
        Player,
        Position(*position),
        Renderable('@'.fg(Color::YELLOW)),
        Viewshed::new(8),
    ));
}

pub fn monster(cmd: &mut Commands, position: &Point, global_rng: &mut GlobalRng) {
    let glyph = global_rng
        .sample(&['g', 'o'])
        .expect("monster sample list should not be empty");
    cmd.spawn((
        Monster,
        Position(*position),
        Renderable(glyph.fg(Color::RED)),
        Viewshed::new(8),
    ));
}
