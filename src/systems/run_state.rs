use bevy::prelude::*;

use crate::RunState;

pub fn run_state(mut run_state: ResMut<RunState>) {
    *run_state = match *run_state {
        RunState::PreRun | RunState::AwaitingInput | RunState::MonsterTurn => {
            RunState::AwaitingInput
        }
        RunState::PlayerTurn => RunState::MonsterTurn,
    }
}
