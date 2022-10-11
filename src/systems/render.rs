use bevy::prelude::*;
use bracket_bevy::prelude::*;

use crate::components::{Position, Renderable};
use crate::map::Map;

pub fn render(ctx: Res<BracketContext>, map: Res<Map>, query: Query<(&Position, &Renderable)>) {
    ctx.cls();

    map.draw(&ctx);

    for (pos, render) in query.iter() {
        if map.is_visible(pos.x, pos.y) {
            ctx.set(pos.x, pos.y, render.fg, render.bg, render.glyph);
        }
    }
}
