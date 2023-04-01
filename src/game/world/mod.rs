use bevy::prelude::*;

pub mod world_cmps;
mod world_sys;

use self::world_cmps::MyWorld;
use crate::AppState;
use world_sys::*;

pub struct WorldPlugin;

impl Plugin for WorldPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<MyWorld>()
            .add_system(ground.in_schedule(OnEnter(AppState::Game)))
            .add_system(light.in_schedule(OnEnter(AppState::Game)))
            .add_system(despawn.in_schedule(OnExit(AppState::Game)));
    }
}
