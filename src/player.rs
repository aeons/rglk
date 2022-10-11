use bevy::prelude::*;

#[derive(Component)]
pub enum PlayerInput {
    Idle,
    Delta { x: i32, y: i32 },
}

impl PlayerInput {
    pub fn delta(x: i32, y: i32) -> Self {
        Self::Delta { x, y }
    }
}

pub fn player_input(keyboard: &Input<KeyCode>) -> PlayerInput {
    // Cardinal directions
    if keyboard.any_just_pressed([KeyCode::Left, KeyCode::Numpad4, KeyCode::H]) {
        PlayerInput::delta(-1, 0)
    } else if keyboard.any_just_pressed([KeyCode::Right, KeyCode::Numpad6, KeyCode::L]) {
        PlayerInput::delta(1, 0)
    } else if keyboard.any_just_pressed([KeyCode::Up, KeyCode::Numpad8, KeyCode::K]) {
        PlayerInput::delta(0, -1)
    } else if keyboard.any_just_pressed([KeyCode::Down, KeyCode::Numpad2, KeyCode::J]) {
        PlayerInput::delta(0, 1)
    } else if keyboard.any_just_pressed([KeyCode::Y, KeyCode::Numpad9]) {
        // Diagonals
        PlayerInput::delta(-1, -1)
    } else if keyboard.any_just_pressed([KeyCode::U, KeyCode::Numpad7]) {
        PlayerInput::delta(1, -1)
    } else if keyboard.any_just_pressed([KeyCode::N, KeyCode::Numpad3]) {
        PlayerInput::delta(1, 1)
    } else if keyboard.any_just_pressed([KeyCode::B, KeyCode::Numpad1]) {
        PlayerInput::delta(-1, 1)
    } else {
        PlayerInput::Idle
    }
}
