use bevy::prelude::*;

#[derive(Component, Reflect, Default)]
#[reflect(Component)]
pub struct Player;

#[derive(Component, Reflect, Default)]
#[reflect(Component)]
pub struct Speed(pub f32);
