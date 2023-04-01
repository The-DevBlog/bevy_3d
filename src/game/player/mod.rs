use bevy::prelude::*;

pub mod player_cmps;
mod player_sys;

use self::player_cmps::{Player, Speed};

use super::GameState;
use crate::AppState;
use player_sys::*;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<Speed>()
            .register_type::<Player>()
            .add_system(spawn.in_schedule(OnEnter(AppState::Game)))
            .add_systems(
                (keyboard_movement, gamepad_movement)
                    .in_set(OnUpdate(AppState::Game))
                    .in_set(OnUpdate(GameState::Running)),
            )
            .add_system(despawn.in_schedule(OnExit(AppState::Game)));
    }
}
