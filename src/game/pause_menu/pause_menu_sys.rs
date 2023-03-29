use bevy::prelude::*;

use super::pause_menu_cmps::{ExitBtn, PauseMenu, ResumeBtn};
use crate::{game::GameState, AppState};

pub fn spawn(mut cmds: Commands, assets: Res<AssetServer>) {
    let menu = (
        NodeBundle {
            style: Style {
                align_items: AlignItems::Center,
                align_self: AlignSelf::Center,
                flex_direction: FlexDirection::Column,
                position: UiRect {
                    left: Val::Percent(30.0),
                    ..default()
                },
                size: Size::new(Val::Percent(40.0), Val::Percent(60.0)),
                ..default()
            },
            background_color: Color::Rgba {
                red: 0.22,
                green: 0.25,
                blue: 0.31,
                alpha: 0.95,
            }
            .into(),
            ..default()
        },
        PauseMenu,
        Name::new("Pause Menu"),
    );

    let menu_txt = (
        TextBundle::from_section(
            "Paused",
            TextStyle {
                color: Color::WHITE,
                font: assets.load("fonts/ZillaSlab-Medium.ttf"),
                font_size: 40.0,
                ..default()
            },
        ),
        Name::new("Paused Text"),
    );

    let resume_btn = (
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
        ResumeBtn,
        Name::new("Resume Button"),
    );

    let resume_txt = (
        TextBundle::from_section(
            "Resume - Start",
            TextStyle {
                color: Color::WHITE,
                font: assets.load("fonts/ZillaSlab-Medium.ttf"),
                font_size: 25.0,
                ..default()
            },
        ),
        Name::new("Resume Text"),
    );

    let exit_btn = (
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
        ExitBtn,
        Name::new("Exit Button"),
    );

    let exit_txt = (
        TextBundle::from_section(
            "Exit - X",
            TextStyle {
                color: Color::WHITE,
                font: assets.load("fonts/ZillaSlab-Medium.ttf"),
                font_size: 25.0,
                ..default()
            },
        ),
        Name::new("Exit Text"),
    );

    // container
    cmds.spawn(menu)
        // "Paused" txt
        .with_children(|menu_parent| {
            menu_parent.spawn(menu_txt);
        })
        // Resume Btn
        .with_children(|menu_parent| {
            menu_parent
                .spawn(resume_btn)
                .with_children(|resume_btn_parent| {
                    resume_btn_parent.spawn(resume_txt);
                });
        })
        // Exit Btn
        .with_children(|menu_parent| {
            menu_parent
                .spawn(exit_btn)
                .with_children(|exit_btn_parent| {
                    exit_btn_parent.spawn(exit_txt);
                });
        });
}

pub fn resume(mut cmds: Commands, mut interact_q: Query<&Interaction, With<ResumeBtn>>) {
    for interaction in &mut interact_q {
        match *interaction {
            Interaction::Clicked => cmds.insert_resource(NextState(Some(GameState::Running))),
            Interaction::Hovered => (),
            _ => (),
        }
    }
}

pub fn exit(mut cmds: Commands, mut interact_q: Query<&Interaction, With<ExitBtn>>) {
    for interaction in &mut interact_q {
        match *interaction {
            Interaction::Clicked => {
                cmds.insert_resource(NextState(Some(AppState::MainMenu)));
                cmds.insert_resource(NextState(Some(GameState::Running)))
            }
            Interaction::Hovered => (),
            _ => (),
        }
    }
}

pub fn despawn(mut cmds: Commands, pause_menu_q: Query<Entity, With<PauseMenu>>) {
    if let Ok(menu) = pause_menu_q.get_single() {
        cmds.entity(menu).despawn_recursive();
    }
}
