use bevy::prelude::*;

use super::main_menu_cmps::{MainMenu, MainMenuCamera};
use crate::{game::gamepad::gamepad_rcs::MyGamepad, AppState};

pub const MENU_COLOR: Color = Color::rgb(0.22, 0.25, 0.31);
pub const PLAY_BTN_COLOR: Color = Color::GRAY;
pub const PLAY_BTN_COLOR_HOVER: Color = Color::rgb(0.65, 0.65, 0.65);

pub fn to_game_state(
    keys: Res<Input<KeyCode>>,
    btns: Res<Input<GamepadButton>>,
    cur_app_state: Res<State<AppState>>,
    mut next_app_state: ResMut<NextState<AppState>>,
    my_gamepad: Option<Res<MyGamepad>>,
) {
    let gamepad_input = my_gamepad
        .map(|gp| btns.just_pressed(GamepadButton::new(gp.gamepad, GamepadButtonType::South)))
        .unwrap_or(false);

    let keys_input = keys.just_pressed(KeyCode::G);

    if gamepad_input || keys_input {
        if cur_app_state.0 != AppState::Game {
            next_app_state.set(AppState::Game);
        }
    }
}

pub fn spawn(mut cmds: Commands, assets: Res<AssetServer>) {
    cmds.spawn((Camera3dBundle::default(), MainMenuCamera));

    cmds.spawn((
        NodeBundle {
            background_color: MENU_COLOR.into(),
            style: Style {
                align_self: AlignSelf::Center,
                align_items: AlignItems::Center,
                flex_direction: FlexDirection::Column,
                justify_content: JustifyContent::Center,
                size: Size::all(Val::Percent(100.0)),
                ..default()
            },
            ..default()
        },
        MainMenu,
        Name::new("Main Menu"),
    ))
    .with_children(|parent| {
        // start game button
        parent
            .spawn((
                ButtonBundle {
                    background_color: PLAY_BTN_COLOR.into(),
                    style: Style {
                        size: Size::new(Val::Px(150.0), Val::Px(75.0)),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    ..default()
                },
                Name::new("Play Button"),
            ))
            .with_children(|parent| {
                // play text
                parent.spawn((
                    TextBundle::from_section(
                        "Play",
                        TextStyle {
                            color: Color::WHITE,
                            font: assets.load("fonts/ZillaSlab-Medium.ttf"),
                            font_size: 40.0,
                            ..default()
                        },
                    ),
                    Name::new("Play Text"),
                ));

                // "A" button img
                parent.spawn((
                    ImageBundle {
                        image: assets.load("imgs/aButton.png").into(),
                        style: Style {
                            size: Size::width(Val::Px(37.0)),
                            margin: UiRect::left(Val::Px(15.0)),
                            ..default()
                        },
                        ..default()
                    },
                    Name::new("A Button Image"),
                ));
            });
    });
}

pub fn select(
    mut interact_q: Query<
        (&Interaction, &mut BackgroundColor),
        (Changed<Interaction>, With<Button>),
    >,
    mut next_app_state: ResMut<NextState<AppState>>,
) {
    for (interaction, mut background_clr) in &mut interact_q {
        match *interaction {
            Interaction::Clicked => next_app_state.set(AppState::Game),
            Interaction::Hovered => *background_clr = PLAY_BTN_COLOR_HOVER.into(),
            Interaction::None => *background_clr = PLAY_BTN_COLOR.into(),
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
