use bevy::prelude::*;

mod pause_menu_cmps;
mod pause_menu_sys;

use super::GameState;
use crate::AppState;
use pause_menu_sys::*;

pub struct PauseMenuPlugin;

impl Plugin for PauseMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(
            spawn
                .in_schedule(OnEnter(GameState::Paused))
                .run_if(|state: Res<State<AppState>>| state.0 == AppState::Game),
        )
        .add_system(despawn.in_schedule(OnExit(GameState::Paused)));
    }
}
