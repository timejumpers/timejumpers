use crate::{
    entities::{Facing, MoveVector},
    player::Player,
};

use action_maps::get_scan_code;
use action_maps::input_type::UniversalInput;
use action_maps::prelude::*;
use bevy::prelude::*;

#[derive(Clone, Copy)]
pub enum Actions {
    Forward,
    Backward,
    Left,
    Right,
}

impl Actions {
    pub fn get_variants() -> Vec<Actions> {
        vec![
            Actions::Forward,
            Actions::Backward,
            Actions::Left,
            Actions::Right,
        ]
    }
}

#[derive(Component)]
pub enum ControlType {
    KeyboardWasd,
    KeyboardArrow,
    Gamepad,
}

impl ControlType {
    pub fn get_binding(&self, action: &Actions) -> UniversalInput {
        match self {
            ControlType::KeyboardWasd => match action {
                Actions::Forward => ScanCode(get_scan_code("W")).into(),
                Actions::Left => ScanCode(get_scan_code("A")).into(),
                Actions::Backward => ScanCode(get_scan_code("S")).into(),
                Actions::Right => ScanCode(get_scan_code("D")).into(),
            },
            ControlType::KeyboardArrow => match action {
                Actions::Forward => ScanCode(get_scan_code("Up")).into(),
                Actions::Left => ScanCode(get_scan_code("Left")).into(),
                Actions::Backward => ScanCode(get_scan_code("Down")).into(),
                Actions::Right => ScanCode(get_scan_code("Right")).into(),
            },
            ControlType::Gamepad => todo!(),
        }
    }
}

pub fn bind_keys(mut control_scheme: ResMut<ControlScheme>) {
    // KeyboardWasd bindings
    control_scheme.insert(
        "KeyboardWasdForward",
        ControlType::KeyboardWasd.get_binding(&Actions::Forward),
    );

    control_scheme.insert(
        "KeyboardWasdLeft",
        ControlType::KeyboardWasd.get_binding(&Actions::Left),
    );

    control_scheme.insert(
        "KeyboardWasdBackward",
        ControlType::KeyboardWasd.get_binding(&Actions::Backward),
    );

    control_scheme.insert(
        "KeyboardWasdRight",
        ControlType::KeyboardWasd.get_binding(&Actions::Right),
    );

    // KeyboardArrow bindings
    control_scheme.insert(
        "KeyboardArrowForward",
        ControlType::KeyboardArrow.get_binding(&Actions::Forward),
    );

    control_scheme.insert(
        "KeyboardArrowLeft",
        ControlType::KeyboardArrow.get_binding(&Actions::Left),
    );

    control_scheme.insert(
        "KeyboardArrowBackward",
        ControlType::KeyboardArrow.get_binding(&Actions::Backward),
    );

    control_scheme.insert(
        "KeyboardArrowRight",
        ControlType::KeyboardArrow.get_binding(&Actions::Right),
    );
}

pub fn handle_input(
    inputs: Res<ActionInput>,
    mut query: Query<(&mut Facing, &mut MoveVector, &ControlType), With<Player>>,
) {
    for (mut facing, mut mv, control_type) in query.iter_mut() {
        let mut new_mv = Vec2::ZERO;
        match control_type {
            ControlType::KeyboardWasd => {
                if inputs.pressed("KeyboardWasdForward") {
                    new_mv.y += 1.0;
                }

                if inputs.pressed("KeyboardWasdLeft") {
                    new_mv.x -= 1.0;
                }

                if inputs.pressed("KeyboardWasdBackward") {
                    new_mv.y -= 1.0;
                }

                if inputs.pressed("KeyboardWasdRight") {
                    new_mv.x += 1.0;
                }
            }
            ControlType::KeyboardArrow => {
                if inputs.pressed("KeyboardArrowForward") {
                    new_mv.y += 1.0;
                }

                if inputs.pressed("KeyboardArrowLeft") {
                    new_mv.x -= 1.0;
                }

                if inputs.pressed("KeyboardArrowBackward") {
                    new_mv.y -= 1.0;
                }

                if inputs.pressed("KeyboardArrowRight") {
                    new_mv.x += 1.0;
                }
            }
            ControlType::Gamepad => todo!(),
        }

        mv.0 = new_mv;

        if mv.0.y < 0.0 {
            *facing = Facing::Forward;
        } else if mv.0.y > 0.0 {
            *facing = Facing::Backward;
        }
    }
}
