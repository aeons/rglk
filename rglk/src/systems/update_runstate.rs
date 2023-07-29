use crate::prelude::*;

pub fn update_runstate(
    state_resource: Res<State<RunState>>,
    mut next_state_resource: ResMut<NextState<RunState>>,
) {
    let next_state = match state_resource.get() {
        RunState::PreRun => RunState::AwaitingInput,
        RunState::AwaitingInput => RunState::AwaitingInput,
        RunState::PlayerTurn => RunState::MonsterTurn,
        RunState::MonsterTurn => RunState::AwaitingInput,
    };
    next_state_resource.set(next_state);
}
