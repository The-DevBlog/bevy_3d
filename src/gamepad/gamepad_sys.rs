use bevy::{
    input::gamepad::{GamepadConnection::*, GamepadConnectionEvent},
    prelude::*,
};

use super::gamepad_rcs::*;

const SENSITIVITY: f32 = 5.0;

pub fn connections(
    mut cmds: Commands,
    gamepad: Option<Res<MyGamepad>>,
    mut gamepad_evr: EventReader<GamepadConnectionEvent>,
) {
    for ev in gamepad_evr.iter() {
        let gp = ev.gamepad;

        match &ev.connection {
            Connected(_info) => {
                // println!("Gamepad connected. ID: {}, name: {}", gp.id, _info.name);

                // if no gamepad is setup yet, use this one
                if gamepad.is_none() {
                    cmds.insert_resource(MyGamepad {
                        gamepad: gp,
                        sensitivity: SENSITIVITY,
                    });
                }
            }
            Disconnected => {
                // println!("Gamepad disconnected: ID: {}", gp.id);

                // remove gamepad resource if gamepad
                let old_gp = MyGamepad {
                    gamepad: gp,
                    sensitivity: SENSITIVITY,
                };

                if old_gp.gamepad == gp {
                    cmds.remove_resource::<MyGamepad>();
                }
            }
        }
    }
}
