mod components;
mod map;
mod player;
mod rect;
mod systems;

use bevy_ecs::prelude::*;
use bracket_lib::prelude::*;
use components::{Viewshed, Monster};
use map::{draw_map, Map};
use player::player_input;

use crate::components::{Player, Position, Renderable};

pub struct State {
    world: World,
    schedule: Schedule,
}

impl GameState for State {
    fn tick(&mut self, ctx: &mut BTerm) {
        ctx.cls();

        player_input(self, ctx);
        self.schedule.run(&mut self.world);

        draw_map(&mut self.world, ctx);

        self.world.resource_scope(|world, map: Mut<Map>| {
            let mut query = world.query::<(&Position, &Renderable)>();
            for (pos, render) in query.iter(world) {
                if map.is_visible(pos.x, pos.y) {
                    ctx.set(pos.x, pos.y, render.fg, render.bg, render.glyph);
                }
            }
        })
    }
}

fn main() -> BError {
    let bterm = BTermBuilder::simple80x50()
        .with_title("rglk - a roguelike")
        .with_fps_cap(60f32)
        .build()?;

    let mut gs = State {
        world: World::new(),
        schedule: Schedule::default(),
    };

    gs.schedule.add_stage(
        "update",
        SystemStage::parallel().with_system(systems::visibility),
    );

    let map = Map::new_rooms_and_corridors();
    let (player_x, player_y) = map.rooms[0].center();

    let mut rng = RandomNumberGenerator::new();
    for room in map.rooms.iter().skip(1) {
        let (x, y) = room.center();

        let glyph = rng.random_slice_entry(&['g', 'o']).unwrap();

        gs.world
            .spawn()
            .insert(Monster)
            .insert(Position { x, y })
            .insert(Renderable::new(*glyph, RED, BLACK))
            .insert(Viewshed::new(8));
    }

    gs.world.insert_resource(map);

    gs.world
        .spawn()
        .insert(Player)
        .insert(Position {
            x: player_x,
            y: player_y,
        })
        .insert(Renderable::new('@', YELLOW, BLACK))
        .insert(Viewshed::new(8));

    main_loop(bterm, gs)
}
