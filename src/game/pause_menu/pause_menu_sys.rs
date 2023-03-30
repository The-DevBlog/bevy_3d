use bevy::prelude::*;

use super::pause_menu_cmps::{ExitBtn, PauseMenu, ResumeBtn};
use crate::{
    game::{gamepad::gamepad_rcs::MyGamepad, GameState},
    AppState,
};

pub fn spawn(mut cmds: Commands, assets: Res<AssetServer>) {
    let menu_container = (
        NodeBundle {
            background_color: Color::Rgba {
                red: 0.22,
                green: 0.25,
                blue: 0.31,
                alpha: 0.95,
            }
            .into(),
            style: Style {
                align_items: AlignItems::Center,
                align_self: AlignSelf::Center,
                flex_direction: FlexDirection::Column,
                gap: Size::height(Val::Percent(15.0)),
                position: UiRect::left(Val::Percent(30.0)),
                padding: UiRect::all(Val::Px(20.0)),
                size: Size::new(Val::Percent(40.0), Val::Percent(60.0)),
                ..default()
            },
            ..default()
        },
        PauseMenu,
        Name::new("Pause Menu"),
    );

    let btn = |name: &str| -> (ButtonBundle, Name) {
        (
            ButtonBundle {
                style: Style {
                    size: Size::new(Val::Px(150.0), Val::Px(60.0)),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    ..default()
                },

                background_color: Color::GRAY.into(),
                ..default()
            },
            Name::new(name.to_string()),
        )
    };

    let txt = |txt: &str, font_size: f32, name: &str| -> (TextBundle, Name) {
        (
            TextBundle::from_section(
                txt.to_string(),
                TextStyle {
                    color: Color::WHITE,
                    font: assets.load("fonts/ZillaSlab-Medium.ttf"),
                    font_size,
                },
            ),
            Name::new(name.to_string()),
        )
    };

    // container
    cmds.spawn(menu_container)
        // "Paused" txt
        .with_children(|menu_parent| {
            menu_parent.spawn(txt("Paused", 40.0, "Paused Text"));
        })
        // Resume Btn
        .with_children(|menu_parent| {
            menu_parent
                .spawn((btn("Resume Button"), ResumeBtn))
                .with_children(|resume_btn_parent| {
                    resume_btn_parent.spawn(txt("resume - start", 25.0, "Resume Text"));
                });
        })
        // Exit Btn
        .with_children(|menu_parent| {
            menu_parent
                .spawn((btn("Exit Button"), ExitBtn))
                .with_children(|exit_btn_parent| {
                    exit_btn_parent.spawn(txt("exit - x", 25.0, "Exit Text"));
                });
        });
}

pub fn resume(
    mut interact_q: Query<&Interaction, With<ResumeBtn>>,
    mut next_game_state: ResMut<NextState<GameState>>,
) {
    // mouse click
    for interaction in &mut interact_q {
        match *interaction {
            Interaction::Clicked => next_game_state.set(GameState::Running),
            Interaction::Hovered => (),
            _ => (),
        }
    }
}

pub fn exit(
    mut interact_q: Query<&Interaction, With<ExitBtn>>,
    btns: Res<Input<GamepadButton>>,
    my_gamepad: Option<Res<MyGamepad>>,
    mut next_game_state: ResMut<NextState<GameState>>,
    mut next_app_state: ResMut<NextState<AppState>>,
) {
    // mouse click
    for interaction in &mut interact_q {
        match *interaction {
            Interaction::Clicked => {
                next_app_state.set(AppState::MainMenu);
                next_game_state.set(GameState::Running);
            }
            Interaction::Hovered => (),
            _ => (),
        }
    }

    // gamepad
    let gamepad_input = my_gamepad
        .map(|gp| btns.just_pressed(GamepadButton::new(gp.gamepad, GamepadButtonType::West)))
        .unwrap_or(false);

    if gamepad_input {
        next_app_state.set(AppState::MainMenu);
        next_game_state.set(GameState::Running);
    }
}

pub fn toggle_menu(
    btns: Res<Input<GamepadButton>>,
    my_gamepad: Option<Res<MyGamepad>>,
    keys: Res<Input<KeyCode>>,
    cur_game_state: Res<State<GameState>>,
    mut next_game_state: ResMut<NextState<GameState>>,
) {
    let gamepad_input = my_gamepad
        .map(|gp| btns.just_pressed(GamepadButton::new(gp.gamepad, GamepadButtonType::Start)))
        .unwrap_or(false);

    let keys_input = keys.just_pressed(KeyCode::Escape);

    if keys_input || gamepad_input {
        if cur_game_state.0 == GameState::Running {
            next_game_state.set(GameState::Paused);
            // println!("GameState: Paused");
        }
        if cur_game_state.0 == GameState::Paused {
            next_game_state.set(GameState::Running);
            // println!("GameState: Running");
        }
    }
}

pub fn despawn(mut cmds: Commands, pause_menu_q: Query<Entity, With<PauseMenu>>) {
    if let Ok(menu) = pause_menu_q.get_single() {
        cmds.entity(menu).despawn_recursive();
    }
}
