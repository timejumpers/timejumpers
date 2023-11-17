use crate::{
    entities::{Facing, MoveVector},
    math::{max, min},
    player::Player,
    keyboard::get_scan_code,
};
use bevy::prelude::*;

#[derive(Component)]
pub struct ControlScheme {
    pub forward: ScanCode,
    pub backward: ScanCode,
    pub left: ScanCode,
    pub right: ScanCode,
}

impl ControlScheme {
    pub fn wasd() -> Self {
        ControlScheme {
            forward: ScanCode(get_scan_code("W")),
            backward: ScanCode(get_scan_code("S")),
            left: ScanCode(get_scan_code("A")),
            right: ScanCode(get_scan_code("D")),
        }
    }

    pub fn arrow() -> Self {
        ControlScheme { 
            forward: ScanCode(get_scan_code("Up")),
            backward: ScanCode(get_scan_code("Down")),
            left: ScanCode(get_scan_code("Left")),
            right: ScanCode(get_scan_code("Right")),
        }
    }
}

pub fn handle_input(
    keys: Res<Input<ScanCode>>,
    mut query: Query<
        (&mut Facing, &mut MoveVector, &ControlScheme),
        With<Player>,
    >,
) {
    let (mut facing, mut mv, cs) = query.single_mut();
    if keys.pressed(cs.forward) {
        *facing = Facing::Backward;
        let new_y = min(mv.0.y + 1.0, 1.0);
        mv.0 = Vec2::new(mv.0.x, new_y);
    }

    if keys.pressed(cs.backward) {
        *facing = Facing::Forward;
        let new_y = max(mv.0.y - 1.0, -1.0);
        mv.0 = Vec2::new(mv.0.x, new_y);
    }

    if keys.pressed(cs.left) {
        let new_x = max(mv.0.x - 1.0, -1.0);
        mv.0 = Vec2::new(new_x, mv.0.y);
    }

    if keys.pressed(cs.right) {
        let new_x = min(mv.0.x + 1.0, 1.0);
        mv.0 = Vec2::new(new_x, mv.0.y);
    }

    if keys.just_released(cs.forward) {
        let new_y = min(mv.0.y - 1.0, 0.0);
        mv.0 = Vec2::new(mv.0.x, new_y);
    }

    if keys.just_released(cs.backward) {
        let new_y = max(mv.0.y - 1.0, 0.0);
        mv.0 = Vec2::new(mv.0.x, new_y);
    }

    if keys.just_released(cs.left) {
        let new_x = min(mv.0.x + 1.0, 0.0);
        mv.0 = Vec2::new(new_x, mv.0.y);
    }

    if keys.just_released(cs.right) {
        let new_x = max(mv.0.x - 1.0, 0.0);
        mv.0 = Vec2::new(new_x, mv.0.y);
    }
}

