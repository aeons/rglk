mod map;

use std::cmp::{max, min};

use bevy_ecs::prelude::*;
use bracket_lib::prelude::*;
use map::Map;

#[derive(Component, Debug)]
struct Player;

#[derive(Component)]
struct Position {
    x: i32,
    y: i32,
}

#[derive(Component)]
struct Renderable {
    glyph: FontCharType,
    fg: RGB,
    bg: RGB,
}

impl Renderable {
    pub fn new(glyph: char, fg: (u8, u8, u8), bg: (u8, u8, u8)) -> Self {
        Self {
            glyph: to_cp437(glyph),
            fg: RGB::named(fg),
            bg: RGB::named(bg),
        }
    }
}

fn try_move_player(delta_x: i32, delta_y: i32, world: &mut World) {
    world.resource_scope(|world: &mut World, map: Mut<Map>| {
        for (mut pos, _) in world
            .query::<(&mut Position, With<Player>)>()
            .iter_mut(world)
        {
            if map.is_accessible(pos.x + delta_x, pos.y + delta_y) {
                pos.x = min(79, max(0, pos.x + delta_x));
                pos.y = min(49, max(0, pos.y + delta_y));
            }
        }
    })
}

fn player_input(gs: &mut State, ctx: &mut BTerm) {
    match ctx.key {
        Some(VirtualKeyCode::Left) => try_move_player(-1, 0, &mut gs.world),
        Some(VirtualKeyCode::Right) => try_move_player(1, 0, &mut gs.world),
        Some(VirtualKeyCode::Up) => try_move_player(0, -1, &mut gs.world),
        Some(VirtualKeyCode::Down) => try_move_player(0, 1, &mut gs.world),
        _ => {}
    }
}

struct State {
    world: World,
    schedule: Schedule,
}

impl GameState for State {
    fn tick(&mut self, ctx: &mut BTerm) {
        ctx.cls();

        player_input(self, ctx);
        self.schedule.run(&mut self.world);

        self.world.resource::<Map>().draw(ctx);

        let mut query = self.world.query::<(&Position, &Renderable)>();

        for (pos, render) in query.iter(&self.world) {
            ctx.set(pos.x, pos.y, render.fg, render.bg, render.glyph);
        }
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

    gs.schedule.add_stage("update", SystemStage::parallel());

    gs.world.insert_resource(Map::new());

    gs.world
        .spawn()
        .insert(Player)
        .insert(Position { x: 40, y: 25 })
        .insert(Renderable::new('@', YELLOW, BLACK));

    main_loop(bterm, gs)
}
