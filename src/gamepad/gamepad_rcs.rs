use bevy::prelude::*;

// #[derive(Resource)]
// pub struct MyGamepad(pub Gamepad);

#[derive(Resource)]
pub struct MyGamepad {
    pub gamepad: Gamepad,
    pub sensitivity: f32,
}
