use bevy::prelude::*;
use bracket_bevy::prelude::*;

use crate::components::Position;
use crate::map::Map;

pub fn draw_tooltips(map: Res<Map>, ctx: Res<BracketContext>, query: Query<(&Name, &Position)>) {
    let mouse_pos = ctx.get_mouse_position_for_current_layer();
    if mouse_pos.x >= map.width || mouse_pos.y >= map.height {
        return;
    }

    let mut tooltip: Vec<String> = Vec::new();
    for (name, position) in &query {
        if position.x == mouse_pos.x
            && position.y == mouse_pos.y
            && map
                .visible_tiles
                .contains(map.xy_idx(position.x, position.y))
        {
            tooltip.push(name.into());
        }
    }

    if tooltip.is_empty() {
        return;
    }

    let width = tooltip
        .iter()
        .map(|s| s.len())
        .max()
        .expect("tooltip is empty");

    let left_tooltip = mouse_pos.x > 40;

    let arrow_pos = {
        let x = mouse_pos.x + if left_tooltip { -2 } else { 1 };
        Point::new(x, mouse_pos.y)
    };

    let line_x = mouse_pos.x + if left_tooltip { -(width as i32) - 3 } else { 4 };
    let padding_x = if left_tooltip {
        arrow_pos.x - 1
    } else {
        arrow_pos.x
    };

    let padding = " ".repeat(3);
    let fg = Color::WHITE;
    let bg = Color::GRAY;

    // Print a line of tooltip with padding for where the arrow goes
    for (y, line) in tooltip.iter().enumerate() {
        let y = mouse_pos.y + y as i32;
        ctx.print_color(line_x, y, format!("{line:<width$}"), fg, bg);
        ctx.print_color(padding_x, y, &padding, fg, bg);
    }

    // Print the arrow
    let arrow = if left_tooltip { "->" } else { "<-" }.to_string();
    ctx.print_color(arrow_pos.x, arrow_pos.y, arrow, fg, bg);
}
