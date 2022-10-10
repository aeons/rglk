mod components;
mod map;
mod player;
mod rect;
mod systems;

use bevy_ecs::prelude::*;
use bevy_ecs::system::SystemState;
use bracket_lib::prelude::*;
use components::{BlocksTile, CombatStats, Monster, Name, Viewshed};
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
        Commands<'w, 's>,
        ResMut<'w, Map>,
        ResMut<'w, Point>,
        Query<'w, 's, (Entity, &'static mut Position), With<Player>>,
        Query<'w, 's, &'static CombatStats>,
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
    gs.schedule.add_stage(
        "after_update",
        SystemStage::parallel()
            .with_system(systems::melee_combat)
            .with_system(systems::map_indexing),
    );

    let map = Map::new_rooms_and_corridors();
    let (player_x, player_y) = map.rooms[0].center();

    let mut rng = RandomNumberGenerator::new();
    for room in map.rooms.iter().skip(1) {
        let (x, y) = room.center();

        let (glyph, name) = rng
            .random_slice_entry(&[('g', "Goblin"), ('o', "Orc")])
            .unwrap();

        gs.world
            .spawn()
            .insert(Monster)
            .insert(Name {
                name: name.to_string(),
            })
            .insert(Position { x, y })
            .insert(BlocksTile)
            .insert(Renderable::new(*glyph, RED, BLACK))
            .insert(Viewshed::new(8))
            .insert(CombatStats {
                max_hp: 16,
                hp: 16,
                power: 4,
                defense: 1,
            });
    }

    gs.world.insert_resource(map);
    gs.world.insert_resource(Point::new(player_x, player_y));

    let map_and_player_position: SystemState<(
        Commands,
        ResMut<Map>,
        ResMut<Point>,
        Query<(Entity, &mut Position), With<Player>>,
        Query<&CombatStats>,
    )> = SystemState::new(&mut gs.world);
    gs.world.insert_resource(MovePlayerState {
        state: map_and_player_position,
    });

    gs.world
        .spawn()
        .insert(Player)
        .insert(Name {
            name: "Player".to_string(),
        })
        .insert(Position {
            x: player_x,
            y: player_y,
        })
        .insert(Renderable::new('@', YELLOW, BLACK))
        .insert(Viewshed::new(8))
        .insert(CombatStats {
            max_hp: 30,
            hp: 30,
            power: 5,
            defense: 2,
        });

    main_loop(bterm, gs)
}
