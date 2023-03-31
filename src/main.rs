use bevy::prelude::*;
use bevy_inspector_egui::quick::WorldInspectorPlugin;

mod game;
mod main_menu;
mod systems;
pub mod ui;

use game::GamePlugin;
use main_menu::MainMenuPlugin;
// use ui::UiPlugin;

fn main() {
    App::new()
        .add_state::<AppState>()
        .add_plugins(DefaultPlugins)
        .add_plugin(WorldInspectorPlugin::new())
        // .add_plugin(UiPlugin)
        .add_plugin(MainMenuPlugin)
        .add_plugin(GamePlugin)
        .run();
}

#[derive(States, Debug, Clone, Copy, Eq, PartialEq, Hash, Default)]
pub enum AppState {
    #[default]
    MainMenu,
    Game,
    GameOver,
}
