use bevy::prelude::*;

use crate::components::Player;
use crate::player::{get_player_input, PlayerInput};
use crate::RunState;

pub fn player_input(
    mut commands: Commands,
    keyboard: Res<Input<KeyCode>>,
    mut run_state: ResMut<RunState>,
    player: Query<Entity, With<Player>>,
) {
    let input = get_player_input(&keyboard);
    if input == PlayerInput::Idle {
        return;
    }
    *run_state = RunState::PlayerTurn;
    dbg!(&input);
    commands.entity(player.single()).insert(input);
}
