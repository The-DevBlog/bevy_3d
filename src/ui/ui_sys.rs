use bevy::prelude::*;

use crate::{
    main_menu::main_menu_sys::{PLAY_BTN_COLOR, PLAY_BTN_COLOR_HOVER},
    AppState,
};

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
