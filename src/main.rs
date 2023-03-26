use bevy::prelude::*;
use bevy_inspector_egui::quick::WorldInspectorPlugin;

mod camera;
mod gamepad;
mod player;
mod systems;

use camera::CameraPlugin;
use gamepad::GamepadPlugin;
use player::PlayerPlugin;
use systems::SystemsPlugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(WorldInspectorPlugin::new())
        .add_plugin(CameraPlugin)
        .add_plugin(PlayerPlugin)
        .add_plugin(SystemsPlugin)
        .add_plugin(GamepadPlugin)
        .run();
}
