#![allow(clippy::type_complexity)]

mod components;
mod map;
mod player;
mod rect;
mod systems;

use bevy::prelude::*;
use bracket_bevy::prelude::*;
use map::Map;

#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub enum RunState {
    #[default]
    PreRun,
    AwaitingInput,
    PlayerTurn,
    MonsterTurn,
}

fn main() {
    use systems::*;

    App::new()
        .insert_resource(WindowDescriptor {
            title: "rglk".into(),
            width: 800.,
            height: 500.,
            ..default()
        })
        .insert_resource(ClearColor(Color::BLACK))
        .add_plugins(DefaultPlugins)
        .add_plugin(
            BTermBuilder::simple_80x50()
                .with_random_number_generator(true)
                .with_scaling_mode(TerminalScalingMode::Stretch),
        )
        .init_resource::<RunState>()
        .init_resource::<Map>()
        .add_startup_system(setup)
        .add_system_to_stage(CoreStage::First, run_state)
        .add_system(visibility)
        .add_system(player_movement)
        .add_system(monster_ai.after(player_movement))
        .add_system(melee_combat.after(monster_ai))
        .add_system_to_stage(CoreStage::PostUpdate, damage)
        .add_system_to_stage(CoreStage::PostUpdate, death.after(damage))
        .add_system_to_stage(CoreStage::Last, map_indexing)
        .add_system_to_stage(CoreStage::Last, render)
        .run()
}
