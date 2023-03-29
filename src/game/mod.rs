use bevy::prelude::*;

mod camera;
pub mod gamepad;
mod pause_menu;
mod player;
mod world;

use camera::CameraPlugin;
use gamepad::GamepadPlugin;
use pause_menu::PauseMenuPlugin;
use player::PlayerPlugin;
use world::WorldPlugin;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_state::<GameState>()
            .add_plugin(PauseMenuPlugin)
            .add_plugin(CameraPlugin)
            .add_plugin(WorldPlugin)
            .add_plugin(PlayerPlugin)
            .add_plugin(GamepadPlugin);
    }
}

#[derive(States, Debug, Clone, Copy, Eq, PartialEq, Hash, Default)]
pub enum GameState {
    #[default]
    Running,
    Paused,
}
