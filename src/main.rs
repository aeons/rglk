use std::cmp::{max, min};

use bevy_ecs::prelude::*;
use bracket_lib::prelude::*;

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

#[derive(Component)]
struct LeftMover;

fn try_move_player(delta_x: i32, delta_y: i32, world: &mut World) {
    for (_player, mut pos) in world.query::<(&Player, &mut Position)>().iter_mut(world) {
        pos.x = min(79, max(0, pos.x + delta_x));
        pos.y = min(49, max(0, pos.y + delta_y));
    }
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

fn move_left(mut query: Query<(&mut Position, &LeftMover)>) {
    for (mut pos, _) in &mut query {
        pos.x -= 1;
        if pos.x < 0 {
            pos.x = 79;
        }
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

        let mut query = self.world.query::<(&Position, &Renderable)>();

        for (pos, render) in query.iter(&self.world) {
            ctx.set(pos.x, pos.y, render.fg, render.bg, render.glyph);
        }
    }
}

fn main() -> BError {
    let bterm = BTermBuilder::simple80x50()
        .with_title("rglk - a roguelike")
        .build()?;

    let mut gs = State {
        world: World::new(),
        schedule: Schedule::default(),
    };

    gs.schedule
        .add_stage("update", SystemStage::parallel().with_system(move_left));

    gs.world
        .spawn()
        .insert(Player)
        .insert(Position { x: 40, y: 25 })
        .insert(Renderable::new('@', YELLOW, BLACK));

    for i in 0..10 {
        gs.world
            .spawn()
            .insert(Position { x: i * 7, y: 20 })
            .insert(Renderable::new('☺', RED, BLACK))
            .insert(LeftMover);
    }

    main_loop(bterm, gs)
}
