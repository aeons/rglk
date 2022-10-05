mod components;
mod map;
mod player;
mod rect;
mod systems;

use bevy_ecs::prelude::*;
use bracket_lib::prelude::*;
use components::Viewshed;
use map::{Map, TileType};
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

        let mut query = self.world.query::<(&Position, &Renderable)>();

        for (pos, render) in query.iter(&self.world) {
            ctx.set(pos.x, pos.y, render.fg, render.bg, render.glyph);
        }
    }
}

fn draw_map(world: &mut World, ctx: &mut BTerm) {
    world.resource_scope(|world, map: Mut<Map>| {
        for (_player, viewshed) in world.query::<(With<Player>, &Viewshed)>().iter(&world) {
            let mut x = 0;
            let mut y = 0;

            for tile in map.tiles.iter() {
                if viewshed.visible_tiles.contains(&Point::new(x, y)) {
                    match tile {
                        TileType::Floor => {
                            ctx.set(x, y, RGB::named(WEBGREY), RGB::named(BLACK), to_cp437('.'))
                        }
                        TileType::Wall => {
                            ctx.set(x, y, RGB::named(GREEN), RGB::named(BLACK), to_cp437('#'))
                        }
                    }
                }

                x += 1;
                if x > 79 {
                    x = 0;
                    y += 1;
                }
            }
        }
    })
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

    gs.world.insert_resource(map);

    gs.world
        .spawn()
        .insert(Player)
        .insert(Position {
            x: player_x,
            y: player_y,
        })
        .insert(Renderable::new('@', YELLOW, BLACK))
        .insert(Viewshed {
            visible_tiles: Vec::new(),
            range: 8,
        });

    main_loop(bterm, gs)
}
