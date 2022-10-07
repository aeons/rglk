mod components;
mod map;
mod player;
mod rect;
mod systems;

use bevy_ecs::prelude::*;
use bevy_ecs::system::SystemState;
use bracket_lib::prelude::*;
use components::{Monster, Viewshed};
use map::{draw_map, Map};
use player::player_input;

use crate::components::{Player, Position, Renderable};

#[derive(PartialEq, Copy, Clone)]
pub enum RunState {
    Paused,
    Running,
}

pub struct State {
    pub world: World,
    pub schedule: Schedule,
    pub runstate: RunState,
}

impl GameState for State {
    fn tick(&mut self, ctx: &mut BTerm) {
        ctx.cls();

        match self.runstate {
            RunState::Paused => self.runstate = player_input(self, ctx),
            RunState::Running => {
                self.schedule.run(&mut self.world);
                self.runstate = RunState::Paused
            }
        }

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

pub struct MovePlayerState<'w, 's> {
    state: SystemState<(
        ResMut<'w, Map>,
        ResMut<'w, Point>,
        Query<'w, 's, &'static mut Position, With<Player>>,
    )>,
}

fn main() -> BError {
    let bterm = BTermBuilder::simple80x50()
        .with_title("rglk - a roguelike")
        .with_fps_cap(60f32)
        .build()?;

    let mut gs = State {
        world: World::new(),
        schedule: Schedule::default(),
        runstate: RunState::Running,
    };

    gs.schedule.add_stage(
        "update",
        SystemStage::parallel()
            .with_system(systems::visibility)
            .with_system(systems::monster_ai),
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
    gs.world.insert_resource(Point::new(player_x, player_y));

    let map_and_player_position: SystemState<(
        ResMut<Map>,
        ResMut<Point>,
        Query<&mut Position, With<Player>>,
    )> = SystemState::new(&mut gs.world);
    gs.world.insert_resource(MovePlayerState {
        state: map_and_player_position,
    });

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
