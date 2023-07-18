mod components;
mod map;
mod spawn;
mod systems;

use bevy::prelude::*;
use bevy_ascii_terminal::prelude::*;
use bevy_turborand::prelude::RngPlugin;
pub use map::Map;

mod prelude {
    pub use bevy::prelude::*;
    pub use bevy_ascii_terminal::prelude::*;
    pub use bevy_turborand::prelude::*;
    pub use bracket_geometry::prelude::*;
    pub use bracket_pathfinding::prelude::*;
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
        .add_systems(
            Update,
            (
                systems::visibility,
                systems::player_movement,
                systems::render,
            )
                .chain(),
        )
        .run();
}
