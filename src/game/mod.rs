use bevy::prelude::*;

mod camera;
mod game_sys;
mod gamepad;
mod player;

use camera::CameraPlugin;
use game_sys::SystemsPlugin;
use gamepad::GamepadPlugin;
use player::PlayerPlugin;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(CameraPlugin)
            .add_plugin(PlayerPlugin)
            .add_plugin(SystemsPlugin)
            .add_plugin(GamepadPlugin)
            .run();
    }
}
