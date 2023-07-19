mod components;
mod map;
pub mod spawn;
mod state;
mod systems;
mod player;

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
    pub use crate::state::RunState;
    pub use crate::{spawn, Map};
}

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins.set(WindowPlugin {
                primary_window: Some(Window {
                    title: "rglk".into(),
                    resolution: (1320., 850.).into(),
                    resizable: false,
                    ..default()
                }),
                ..default()
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
            systems::player_movement.run_if(in_state(RunState::Paused)),
        )
        .add_systems(
            Update,
            (
                systems::visibility,
                systems::monster_ai,
                systems::map_indexing,
            )
                .chain()
                .run_if(in_state(RunState::Running)),
        )
        .add_systems(PostUpdate, (systems::render, systems::wait_for_input))
        .run();
}
