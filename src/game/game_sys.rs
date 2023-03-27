use bevy::{input::gamepad::GamepadButtonType::*, prelude::*};

use super::{gamepad::gamepad_rcs::MyGamepad, GameState};

pub fn toggle_game_state(
    mut cmds: Commands,
    btns: Res<Input<GamepadButton>>,
    my_gamepad: Option<Res<MyGamepad>>,
    keys: Res<Input<KeyCode>>,
    game_state: Res<State<GameState>>,
) {
    let gamepad_input = my_gamepad
        .map(|gp| btns.just_pressed(GamepadButton::new(gp.gamepad, Start)))
        .unwrap_or(false);

    let keys_input = keys.just_pressed(KeyCode::Space);

    if keys_input || gamepad_input {
        if game_state.0 == GameState::Running {
            cmds.insert_resource(NextState(Some(GameState::Paused)));
            println!("GameState: Paused");
        }
        if game_state.0 == GameState::Paused {
            cmds.insert_resource(NextState(Some(GameState::Running)));
            println!("GameState: Running");
        }
    }
}
