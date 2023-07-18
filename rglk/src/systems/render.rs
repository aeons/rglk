use crate::components::{Position, Renderable};
use crate::map::Map;
use crate::prelude::*;

pub fn render(
    mut terminal: Query<&mut Terminal>,
    renderables: Query<(&Position, &Renderable)>,
    map: Res<Map>,
) {
    let mut terminal = terminal.single_mut();

    terminal.clear();

    map.render(&mut terminal);

    for (pos, r) in renderables.iter() {
        if map.is_visible(pos) {
            terminal.put_char(*pos, r.0.clone())
        }
    }
}
