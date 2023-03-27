use bevy::{input::gamepad::GamepadButtonType::*, prelude::*};

use crate::game::gamepad::gamepad_rcs::MyGamepad;
use crate::{game::GameState, AppState};

pub struct SystemsPlugin;

impl Plugin for SystemsPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(to_main_menu_state).add_system(to_game_state);
    }
}

pub fn to_main_menu_state(
    mut cmds: Commands,
    keys: Res<Input<KeyCode>>,
    btns: Res<Input<GamepadButton>>,
    app_state: Res<State<AppState>>,
    my_gamepad: Option<Res<MyGamepad>>,
) {
    let gamepad_input = my_gamepad
        .map(|gp| btns.just_pressed(GamepadButton::new(gp.gamepad, West)))
        .unwrap_or(false);

    let keys_input = keys.just_pressed(KeyCode::Escape);

    if gamepad_input || keys_input {
        if app_state.0 != AppState::MainMenu {
            cmds.insert_resource(NextState(Some(AppState::MainMenu)));
            cmds.insert_resource(NextState(Some(GameState::Paused)));
            println!("AppState: Main Menu");
        }
    }
}

pub fn to_game_state(
    mut cmds: Commands,
    keys: Res<Input<KeyCode>>,
    btns: Res<Input<GamepadButton>>,
    app_state: Res<State<AppState>>,
    my_gamepad: Option<Res<MyGamepad>>,
) {
    let gamepad_input = my_gamepad
        .map(|gp| btns.just_pressed(GamepadButton::new(gp.gamepad, South)))
        .unwrap_or(false);

    let keys_input = keys.just_pressed(KeyCode::G);

    if gamepad_input || keys_input {
        if app_state.0 != AppState::Game {
            cmds.insert_resource(NextState(Some(AppState::Game)));
            println!("AppState: Game")
        }
    }
}
