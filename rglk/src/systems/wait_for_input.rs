use crate::prelude::*;

pub fn wait_for_input(mut state: ResMut<NextState<RunState>>) {
    state.set(RunState::Paused)
}
