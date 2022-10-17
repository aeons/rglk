use std::borrow::Cow;

use bevy::prelude::*;
use bracket_bevy::RandomNumbers;
use bracket_random::rand::distributions::uniform::SampleUniform;

use crate::components::*;
use crate::map;
use crate::rect::Rect;

const MAX_MONSTERS: i32 = 4;
const MAX_ITEMS: i32 = 2;

pub fn player(commands: &mut Commands, x: i32, y: i32) {
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
}

pub fn random_monster(commands: &mut Commands, rng: &RandomNumbers, x: i32, y: i32) {
    match rng.range(0, 2) {
        0 => orc(commands, x, y),
        _ => goblin(commands, x, y),
    }
}

fn orc(commands: &mut Commands, x: i32, y: i32) {
    monster(commands, x, y, 'o', "Orc")
}

fn goblin(commands: &mut Commands, x: i32, y: i32) {
    monster(commands, x, y, 'g', "Goblin")
}

pub fn monster(
    commands: &mut Commands,
    x: i32,
    y: i32,
    glyph: char,
    name: impl Into<Cow<'static, str>>,
) {
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

fn health_potion(commands: &mut Commands, x: i32, y: i32) {
    commands
        .spawn()
        .insert(Item)
        .insert(Position { x, y })
        .insert(Renderable::new('¡', Color::FUCHSIA, Color::BLACK))
        .insert(Name::new("Health Potion"))
        .insert(Potion { heal_amount: 8 });
}

pub fn fill_room(commands: &mut Commands, rng: &RandomNumbers, room: &Rect) {
    let fill = |spawn: &mut dyn FnMut(i32, i32)| {
        let mut spawns = Vec::new();
        let num_spawns = rng.roll_dice(1, MAX_MONSTERS + 2) - 3;
        for _ in 0..num_spawns {
            loop {
                let x = minmax_range(rng, room.x1 + 1, room.x2);
                let y = minmax_range(rng, room.y1 + 1, room.y2);
                let idx = map::xy_idx(x, y);
                if !spawns.contains(&idx) {
                    spawns.push(idx);
                    break;
                }
            }
        }

        for idx in spawns.iter() {
            let (x, y) = map::idx_xy(*idx);
            spawn(x, y);
        }
    };

    fill(&mut |x, y| random_monster(commands, rng, x, y));
    fill(&mut |x, y| health_potion(commands, x, y))
}

fn minmax_range<T>(rng: &RandomNumbers, n1: T, n2: T) -> T
where
    T: SampleUniform + PartialOrd,
{
    if n1 > n2 {
        rng.range(n2, n1)
    } else {
        rng.range(n1, n2)
    }
}
