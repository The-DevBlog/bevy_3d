use bevy::prelude::*;

use crate::{
    game::{gamepad::gamepad_rcs::MyGamepad, GameState},
    AppState,
};

use super::main_menu_cmps::{MainMenu, MainMenuCamera};

pub fn to_main_menu_state(
    mut cmds: Commands,
    keys: Res<Input<KeyCode>>,
    btns: Res<Input<GamepadButton>>,
    app_state: Res<State<AppState>>,
    my_gamepad: Option<Res<MyGamepad>>,
) {
    let gamepad_input = my_gamepad
        .map(|gp| btns.just_pressed(GamepadButton::new(gp.gamepad, GamepadButtonType::West)))
        .unwrap_or(false);

    let keys_input = keys.just_pressed(KeyCode::Escape);

    if gamepad_input || keys_input {
        if app_state.0 != AppState::MainMenu {
            cmds.insert_resource(NextState(Some(AppState::MainMenu)));
            cmds.insert_resource(NextState(Some(GameState::Paused)));
            println!("AppState: Main Menu");
        }
    }
}

pub fn to_game_state(
    mut cmds: Commands,
    keys: Res<Input<KeyCode>>,
    btns: Res<Input<GamepadButton>>,
    app_state: Res<State<AppState>>,
    my_gamepad: Option<Res<MyGamepad>>,
) {
    let gamepad_input = my_gamepad
        .map(|gp| btns.just_pressed(GamepadButton::new(gp.gamepad, GamepadButtonType::South)))
        .unwrap_or(false);

    let keys_input = keys.just_pressed(KeyCode::G);

    if gamepad_input || keys_input {
        if app_state.0 != AppState::Game {
            cmds.insert_resource(NextState(Some(AppState::Game)));
            println!("AppState: Game")
        }
    }
}

pub fn spawn(mut cmds: Commands, assets: Res<AssetServer>) {
    cmds.spawn((Camera3dBundle::default(), MainMenuCamera));

    cmds.spawn((
        NodeBundle {
            style: Style {
                size: Size::width(Val::Percent(100.0)),
                flex_direction: FlexDirection::Column,
                align_self: AlignSelf::Center,
                align_items: AlignItems::Center,
                ..default()
            },
            ..default()
        },
        MainMenu,
        Name::new("Main Menu"),
    ))
    .with_children(|parent| {
        parent
            .spawn((
                ButtonBundle {
                    style: Style {
                        size: Size::new(Val::Px(150.0), Val::Px(75.0)),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        ..default()
                    },

                    background_color: Color::GRAY.into(),
                    ..default()
                },
                // CustomBtn,
                Name::new("Play Button"),
            ))
            .with_children(|parent| {
                parent.spawn((
                    TextBundle::from_section(
                        "Play - A",
                        TextStyle {
                            color: Color::WHITE,
                            font: assets.load("fonts/ZillaSlab-Medium.ttf"),
                            font_size: 40.0,
                            ..default()
                        },
                    ),
                    Name::new("Play Text"),
                ));
            });
    });
}

pub fn select(mut cmds: Commands, mut interact_q: Query<&Interaction, With<Button>>) {
    for interaction in &mut interact_q {
        match *interaction {
            Interaction::Clicked => cmds.insert_resource(NextState(Some(AppState::Game))),
            Interaction::Hovered => println!("HOVERING"),
            _ => (),
        }
    }
}

pub fn despawn(
    mut cmds: Commands,
    main_menu_camera_q: Query<Entity, With<MainMenuCamera>>,
    main_menu_q: Query<Entity, With<MainMenu>>,
) {
    if let Ok(camera) = main_menu_camera_q.get_single() {
        cmds.entity(camera).despawn_recursive();
    }

    if let Ok(menu) = main_menu_q.get_single() {
        cmds.entity(menu).despawn_recursive();
    }
}
