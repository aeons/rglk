#![allow(clippy::type_complexity)]

mod components;
mod map;
mod player;
pub mod spawn;
mod state;
mod systems;

use bevy::log::{Level, LogPlugin};
use bevy::prelude::*;
use bevy_ascii_terminal::prelude::*;
use bevy_turborand::prelude::RngPlugin;
pub use map::Map;
use state::RunState;

mod prelude {
    pub use bevy::prelude::*;
    pub use bevy_ascii_terminal::prelude::*;
    pub use bevy_turborand::prelude::*;
    pub use bracket_geometry::prelude::*;
    pub use bracket_pathfinding::prelude::*;

    pub use crate::components::*;
    pub use crate::player::PlayerEntity;
    pub use crate::state::RunState;
    pub use crate::{spawn, Map};
}

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "rglk".into(),
                        resolution: (1320., 850.).into(),
                        resizable: false,
                        ..default()
                    }),
                    ..default()
                })
                .set(LogPlugin {
                    filter: "info,wgpu_core=warn,wgpu_hal=warn,rglk=info".into(),
                    level: Level::INFO,
                }),
            TerminalPlugin,
            RngPlugin::new(),
        ))
        .insert_resource(ClearColor(Color::BLACK))
        .init_resource::<Map>()
        .add_systems(Startup, systems::setup)
        .add_state::<RunState>()
        .add_systems(
            PreUpdate,
            systems::player_movement.run_if(in_state(RunState::AwaitingInput)),
        )
        .add_systems(
            Update,
            (
                systems::visibility,
                systems::monster_ai.run_if(in_state(RunState::MonsterTurn)),
                systems::melee_combat,
                systems::damage,
                systems::delete_the_dead,
                systems::map_indexing,
            )
                .chain()
                .run_if(not(in_state(RunState::AwaitingInput))),
        )
        .add_systems(PostUpdate, (systems::render, systems::update_runstate))
        .run();
}
