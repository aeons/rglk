use bevy::prelude::*;
use bracket_bevy::prelude::*;

use crate::components::{CombatStats, Player};
use crate::gamelog::GameLog;

pub fn draw_ui(
    ctx: Res<BracketContext>,
    game_log: Res<GameLog>,
    combat_stats: Query<&CombatStats, With<Player>>,
) {
    ctx.draw_box(0, 43, 79, 6, Color::WHITE, Color::BLACK);

    for stats in &combat_stats {
        let health = format!(" HP: {} / {}", stats.hp, stats.max_hp);
        ctx.print_color(12, 43, &health, Color::YELLOW, Color::BLACK);
        ctx.draw_bar_horizontal(28, 43, 51, stats.hp, stats.max_hp, Color::RED, Color::BLACK);
    }

    let mut y = 44;
    for entry in game_log.entries.iter().rev() {
        if y < 49 {
            ctx.print(2, y, entry);
        }
        y += 1;
    }
}
