use bevy::prelude::*;

mod main_menu_cmps;
mod main_menu_sys;

use crate::AppState;
use main_menu_sys::*;

pub struct MainMenuPlugin;

impl Plugin for MainMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(spawn.in_schedule(OnEnter(AppState::MainMenu)))
            .add_system(despawn.in_schedule(OnExit(AppState::MainMenu)))
            .add_system(select)
            .add_system(to_main_menu_state)
            .add_system(to_game_state);
    }
}
