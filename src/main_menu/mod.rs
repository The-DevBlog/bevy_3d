use bevy::prelude::*;

mod main_menu_cmps;
pub mod main_menu_sys;

use crate::ui::ui_sys::select;
use crate::AppState;
use main_menu_sys::*;

pub struct MainMenuPlugin;

impl Plugin for MainMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(spawn.in_schedule(OnEnter(AppState::MainMenu)))
            .add_system(despawn.in_schedule(OnExit(AppState::MainMenu)))
            .add_systems((select, to_game_state).in_set(OnUpdate(AppState::MainMenu)));
    }
}
