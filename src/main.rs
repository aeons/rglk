use bevy_ecs::prelude::*;
use bracket_lib::prelude::*;

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
