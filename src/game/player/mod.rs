use bevy::prelude::*;

pub mod player_cmps;
mod player_sys;

use player_sys::*;

use crate::AppState;

use super::GameState;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(spawn).add_systems(
            (keyboard_movement, gamepad_movement)
                .in_set(OnUpdate(AppState::Game))
                .in_set(OnUpdate(GameState::Running)),
        );
    }
}
