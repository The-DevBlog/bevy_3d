use bevy::prelude::*;

use super::pause_menu_cmps::PauseMenu;

pub fn spawn(mut cmds: Commands, assets: Res<AssetServer>) {
    cmds.spawn((
        NodeBundle {
            style: Style {
                size: Size::width(Val::Percent(100.0)),
                margin: UiRect {
                    top: Val::Px(50.0),
                    ..default()
                },
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::Center,
                ..default()
            },
            ..default()
        },
        PauseMenu,
        Name::new("Pause Menu"),
    ))
    .with_children(|parent| {
        parent.spawn((
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
        ));
        // .spawn((
        //     ButtonBundle {
        //         style: Style {
        //             size: Size::new(Val::Px(150.0), Val::Px(75.0)),
        //             justify_content: JustifyContent::Center,
        //             align_items: AlignItems::Center,
        //             ..default()
        //         },
        //         background_color: Color::GRAY.into(),
        //         ..default()
        //     },
        //     // CustomBtn,
        //     Name::new("Paused"),
        // ))
        // .spawn(|parent| {
        //     parent.spawn((
        //         TextBundle::from_section(
        //             "Paused",
        //             TextStyle {
        //                 color: Color::WHITE,
        //                 font: assets.load("fonts/ZillaSlab-Medium.ttf"),
        //                 font_size: 40.0,
        //                 ..default()
        //             },
        //         ),
        //         Name::new("Paused Text"),
        //     ));
        // });
    });
}

pub fn despawn(mut cmds: Commands, pause_menu_q: Query<Entity, With<PauseMenu>>) {
    if let Ok(menu) = pause_menu_q.get_single() {
        cmds.entity(menu).despawn_recursive();
    }
}
