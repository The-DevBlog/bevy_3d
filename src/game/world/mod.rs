use bevy::prelude::*;

mod world_cmps;
mod world_sys;

use world_sys::*;

use crate::AppState;

pub struct WorldPlugin;

impl Plugin for WorldPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(ground.in_schedule(OnEnter(AppState::Game)))
            .add_system(light.in_schedule(OnEnter(AppState::Game)))
            .add_system(despawn.in_schedule(OnExit(AppState::Game)));
    }
}
