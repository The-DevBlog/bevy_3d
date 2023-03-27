use bevy::prelude::*;

mod camera;
mod game_sys;
mod gamepad;
mod player;
mod world;

use camera::CameraPlugin;
use game_sys::SystemsPlugin;
use gamepad::GamepadPlugin;
use player::PlayerPlugin;
use world::WorldPlugin;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(CameraPlugin)
            .add_plugin(WorldPlugin)
            .add_plugin(PlayerPlugin)
            .add_plugin(SystemsPlugin)
            .add_plugin(GamepadPlugin)
            .run();
    }
}

#[derive(States, Debug, Clone, Copy, Eq, PartialEq, Hash, Default)]
pub enum GameState {
    Running,
    #[default]
    Paused,
}
