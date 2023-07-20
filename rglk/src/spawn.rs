use crate::prelude::*;

pub fn player(cmd: &mut Commands, position: &Point) -> Entity {
    cmd.spawn((
        Player,
        Name::new("Player"),
        Position(*position),
        Renderable('@'.fg(Color::YELLOW)),
        Viewshed::new(8),
        CombatStats {
            hp: 30,
            max_hp: 30,
            power: 5,
            defense: 2,
        },
    ))
    .id()
}

pub fn monster(
    cmd: &mut Commands,
    position: &Point,
    index: usize,
    global_rng: &mut GlobalRng,
) -> Entity {
    let (glyph, name) = global_rng
        .sample(&[('g', "Goblin"), ('o', "Orc")])
        .expect("monster sample list should not be empty");

    cmd.spawn((
        Monster,
        BlocksTile,
        Name::new(format!("{name} #{index}")),
        Position(*position),
        Renderable(glyph.fg(Color::RED)),
        Viewshed::new(8),
        CombatStats {
            hp: 16,
            max_hp: 16,
            power: 4,
            defense: 1,
        },
    ))
    .id()
}
