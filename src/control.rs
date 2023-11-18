use crate::{
    entities::{Facing, MoveVector},
    keyboard::get_scan_code,
    player::Player,
};
use bevy::prelude::*;

#[derive(Component)]
pub enum ControlScheme {
    Keyboard {
        forward: ScanCode,
        backward: ScanCode,
        left: ScanCode,
        right: ScanCode,
    },
}

impl ControlScheme {
    pub fn wasd() -> Self {
        ControlScheme::Keyboard {
            forward: ScanCode(get_scan_code("W")),
            backward: ScanCode(get_scan_code("S")),
            left: ScanCode(get_scan_code("A")),
            right: ScanCode(get_scan_code("D")),
        }
    }

    pub fn arrow() -> Self {
        ControlScheme::Keyboard {
            forward: ScanCode(get_scan_code("Up")),
            backward: ScanCode(get_scan_code("Down")),
            left: ScanCode(get_scan_code("Left")),
            right: ScanCode(get_scan_code("Right")),
        }
    }
}

pub struct ControlState {
    pub forward: isize,
    pub backward: isize,
    pub left: isize,
    pub right: isize,
}

pub fn print_scan_codes(mut keys: EventReader<bevy::input::keyboard::KeyboardInput>) {
    for ev in keys.read() {
        let sc = format!("{:#02x}", ev.scan_code);
        dbg!(sc);
    }
}

pub fn handle_input(
    keys: Res<Input<ScanCode>>,
    mut query: Query<
        (&mut Facing, &mut MoveVector, &ControlScheme),
        With<Player>,
    >,
) {
    for (mut facing, mut mv, cs) in query.iter_mut() {
        let input = match cs {
            ControlScheme::Keyboard {
                forward,
                backward,
                left,
                right,
            } => ControlState {
                    forward: if keys.pressed(*forward) { 1 } else { 0 },
                    backward: if keys.pressed(*backward) { 1 } else { 0 },
                    left: if keys.pressed(*left) { 1 } else { 0 },
                    right: if keys.pressed(*right) { 1 } else { 0 },
                },
        };

        mv.0 = Vec2::new(
            (input.right - input.left) as f32,
            (input.forward - input.backward) as f32,
        );

        if mv.0.y < 0.0 {
            *facing = Facing::Forward;
        } else if mv.0.y > 0.0 {
            *facing = Facing::Backward;
        }
    } 
}
