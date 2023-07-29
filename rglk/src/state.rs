use crate::prelude::*;

#[derive(States, Debug, Clone, PartialEq, Eq, Hash, Default)]
pub enum RunState {
    #[default]
    PreRun,
    AwaitingInput,
    PlayerTurn,
    MonsterTurn,
}
