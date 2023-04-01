use bevy::prelude::*;

mod pause_menu_cmps;
mod pause_menu_sys;

use self::pause_menu_cmps::PauseMenu;

use super::GameState;
use crate::{ui::ui_sys::select, AppState};
use pause_menu_sys::*;

pub struct PauseMenuPlugin;

impl Plugin for PauseMenuPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<PauseMenu>()
            .add_system(
                spawn
                    .in_schedule(OnEnter(GameState::Paused))
                    .run_if(|s: Res<State<AppState>>| s.0 == AppState::Game),
            )
            .add_systems(
                (resume, exit)
                    .in_set(OnUpdate(AppState::Game))
                    .in_set(OnUpdate(GameState::Paused)),
            )
            .add_system(toggle_menu.run_if(in_state(AppState::Game)))
            .add_system(despawn.in_schedule(OnExit(GameState::Paused)))
            .add_system(select.in_set(OnUpdate(GameState::Paused)))
            .add_system(save);
    }
}
