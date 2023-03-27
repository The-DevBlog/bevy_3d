use bevy::prelude::*;

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
    app_state: Res<State<AppState>>,
) {
    if keys.just_pressed(KeyCode::Escape) {
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
    app_state: Res<State<AppState>>,
) {
    if keys.just_pressed(KeyCode::G) {
        if app_state.0 != AppState::Game {
            cmds.insert_resource(NextState(Some(AppState::Game)));
            println!("AppState: Game")
        }
    }
}
