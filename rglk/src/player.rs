use bevy::prelude::*;

#[derive(Debug, Component, PartialEq, Eq)]
pub enum PlayerInput {
    Idle,
    Movement { x: i32, y: i32 },
    PickupItem,
}

impl PlayerInput {
    pub fn movement(x: i32, y: i32) -> Self {
        Self::Movement { x, y }
    }
}

pub fn get_player_input(keyboard: &Input<KeyCode>) -> PlayerInput {
    // Cardinal directions
    if keyboard.any_just_pressed([KeyCode::Left, KeyCode::Numpad4, KeyCode::H]) {
        PlayerInput::movement(-1, 0)
    } else if keyboard.any_just_pressed([KeyCode::Right, KeyCode::Numpad6, KeyCode::L]) {
        PlayerInput::movement(1, 0)
    } else if keyboard.any_just_pressed([KeyCode::Up, KeyCode::Numpad8, KeyCode::K]) {
        PlayerInput::movement(0, -1)
    } else if keyboard.any_just_pressed([KeyCode::Down, KeyCode::Numpad2, KeyCode::J]) {
        PlayerInput::movement(0, 1)
    } else if keyboard.any_just_pressed([KeyCode::Y, KeyCode::Numpad9]) {
        // Diagonals
        PlayerInput::movement(-1, -1)
    } else if keyboard.any_just_pressed([KeyCode::U, KeyCode::Numpad7]) {
        PlayerInput::movement(1, -1)
    } else if keyboard.any_just_pressed([KeyCode::N, KeyCode::Numpad3]) {
        PlayerInput::movement(1, 1)
    } else if keyboard.any_just_pressed([KeyCode::B, KeyCode::Numpad1]) {
        PlayerInput::movement(-1, 1)
    }
    // else if keyboard.just_pressed(KeyCode::G) {
    // PlayerInput::PickupItem
    // } else {
    // PlayerInput::Idle
    // }
}
