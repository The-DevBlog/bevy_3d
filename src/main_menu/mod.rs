use bevy::prelude::*;

mod main_menu_cmps;

use main_menu_cmps::*;

use crate::AppState;

pub struct MainMenuPlugin;

impl Plugin for MainMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(menu.in_schedule(OnEnter(AppState::MainMenu)))
            .add_system(despawn.in_schedule(OnExit(AppState::MainMenu)));
    }
}

fn menu(mut cmds: Commands) {
    let text = TextBundle::from_section(
        "Super Cool Game",
        TextStyle {
            font_size: 100.0,
            color: Color::WHITE,
            ..default()
        },
    )
    .with_text_alignment(TextAlignment::Center)
    .with_style(Style {
        position_type: PositionType::Absolute,
        position: UiRect {
            bottom: Val::Px(50.0),
            right: Val::Px(50.0),
            ..default()
        },
        ..default()
    });

    cmds.spawn((
        Camera3dBundle::default(),
        UiCameraConfig { show_ui: true },
        MainMenuCamera,
    ))
    .insert(text);

    // cmds.spawn(text);
}

fn despawn(mut cmds: Commands, main_menu_camera_q: Query<Entity, With<MainMenuCamera>>) {
    if let Ok(camera) = main_menu_camera_q.get_single() {
        cmds.entity(camera).despawn_recursive();
    }
}
