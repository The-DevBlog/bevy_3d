use bevy::prelude::*;
use bevy_inspector_egui::quick::WorldInspectorPlugin;

mod game;

use game::GamePlugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(WorldInspectorPlugin::new())
        .add_plugin(GamePlugin)
        .run();
}
