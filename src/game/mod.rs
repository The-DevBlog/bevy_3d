use bevy::prelude::*;

mod camera;
mod game_sys;
pub mod gamepad;
mod player;
mod world;

use camera::CameraPlugin;
use game_sys::*;
use gamepad::GamepadPlugin;
use player::PlayerPlugin;
use world::WorldPlugin;

use crate::AppState;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_state::<GameState>()
            .add_plugin(CameraPlugin)
            .add_plugin(WorldPlugin)
            .add_plugin(PlayerPlugin)
            .add_plugin(GamepadPlugin)
            .add_system(toggle_game_state.run_if(in_state(AppState::Game)));
    }
}

#[derive(States, Debug, Clone, Copy, Eq, PartialEq, Hash, Default)]
pub enum GameState {
    Running,
    #[default]
    Paused,
}
