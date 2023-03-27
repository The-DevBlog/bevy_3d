use bevy::prelude::*;

use super::GameState;

pub fn toggle_game_state(
    mut cmds: Commands,
    keys: Res<Input<KeyCode>>,
    game_state: Res<State<GameState>>,
) {
    if keys.just_pressed(KeyCode::Space) {
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
