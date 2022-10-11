use bevy::prelude::*;
use bracket_bevy::prelude::*;

use crate::components::*;
use crate::map::Map;

pub fn setup(mut commands: Commands, map: Res<Map>, rng: Res<RandomNumbers>) {
    let (x, y) = map.rooms[0].center();
    commands
        .spawn()
        .insert(Player)
        .insert(Name::new("Player"))
        .insert(Position { x, y })
        .insert(Renderable::new('@', Color::YELLOW, Color::BLACK))
        .insert(Viewshed::new(8))
        .insert(CombatStats {
            max_hp: 30,
            hp: 30,
            power: 5,
            defense: 2,
        });

    for room in map.rooms.iter().skip(1) {
        let (x, y) = room.center();

        let (glyph, name) = *rng
            .random_slice_entry(&[('g', "Goblin"), ('o', "Orc")])
            .unwrap();

        commands
            .spawn()
            .insert(Monster)
            .insert(Name::new(name))
            .insert(Position { x, y })
            .insert(BlocksTile)
            .insert(Renderable::new(glyph, Color::RED, Color::BLACK))
            .insert(Viewshed::new(8))
            .insert(CombatStats {
                max_hp: 16,
                hp: 16,
                power: 4,
                defense: 1,
            });
    }
}
