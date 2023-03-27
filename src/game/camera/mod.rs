use bevy::prelude::*;

pub mod camera_cmps;
mod camera_sys;

use super::GameState;
use crate::AppState;
use camera_sys::*;

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            (zoom_mouse, zoom_gamepad, orbit_mouse, orbit_gamepad)
                .in_set(OnUpdate(AppState::Game))
                .in_set(OnUpdate(GameState::Running)),
        );
    }
}
