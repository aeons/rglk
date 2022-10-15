#![allow(clippy::type_complexity)]

mod components;
mod map;
mod player;
mod rect;
mod systems;

use bevy::ecs::schedule::ShouldRun;
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

impl RunState {
    pub fn when_awaiting_input(run_state: Res<RunState>) -> ShouldRun {
        if *run_state == Self::AwaitingInput {
            ShouldRun::Yes
        } else {
            ShouldRun::No
        }
    }

    pub fn when_player_turn(run_state: Res<RunState>) -> ShouldRun {
        if *run_state == Self::PlayerTurn {
            ShouldRun::Yes
        } else {
            ShouldRun::No
        }
    }

    pub fn when_monster_turn(run_state: Res<RunState>) -> ShouldRun {
        if *run_state == Self::MonsterTurn {
            ShouldRun::Yes
        } else {
            ShouldRun::No
        }
    }
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
        .add_system(visibility)
        .add_system(
            player_movement
                .after(visibility)
                .with_run_criteria(RunState::when_awaiting_input),
        )
        .add_system(
            monster_ai
                .after(player_movement)
                .with_run_criteria(RunState::when_monster_turn),
        )
        .add_system(melee_combat.after(monster_ai))
        .add_system_to_stage(CoreStage::PostUpdate, damage)
        .add_system_to_stage(CoreStage::PostUpdate, death.after(damage))
        .add_system_to_stage(CoreStage::Last, map_indexing)
        .add_system_to_stage(CoreStage::Last, render)
        .add_system_to_stage(CoreStage::Last, run_state)
        .run()
}
