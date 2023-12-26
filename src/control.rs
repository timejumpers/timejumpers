use crate::{
    entities::{Facing, MoveVector},
    player::{Player, PlayerId},
};

use action_maps::get_scan_code;
use action_maps::multiplayer::*;
use bevy::prelude::*;

#[derive(Clone, Copy)]
pub enum Actions {
    Forward,
    Backward,
    Left,
    Right,
}

impl From<Actions> for Action {
    fn from(value: Actions) -> Self {
        match value {
            Actions::Forward => Action::from("Forward"),
            Actions::Backward => Action::from("Backward"),
            Actions::Left => Action::from("Left"),
            Actions::Right => Action::from("Right"),
        }
    }
}

pub fn bind_keys(
    mut multi_scheme: ResMut<MultiScheme>,
    mut multi_input: ResMut<MultiInput>,
) {
    make_multi_input!(
        multi_input,
        multi_scheme,
        (
            (Actions::Forward, ScanCode(get_scan_code("W"))),
            (Actions::Backward, ScanCode(get_scan_code("S"))),
            (Actions::Left, ScanCode(get_scan_code("A"))),
            (Actions::Right, ScanCode(get_scan_code("D"))),
        ),
        (
            (Actions::Forward, ScanCode(get_scan_code("Up"))),
            (Actions::Backward, ScanCode(get_scan_code("Down"))),
            (Actions::Left, ScanCode(get_scan_code("Left"))),
            (Actions::Right, ScanCode(get_scan_code("Right"))),
        )
    );
}

pub fn handle_input(
    inputs: Res<MultiInput>,
    mut query: Query<(&mut Facing, &mut MoveVector, &PlayerId), With<Player>>,
) {
    for (mut facing, mut mv, PlayerId(id)) in query.iter_mut() {
        let mut new_mv = Vec2::ZERO;

        let actions = inputs.get(*id).unwrap();

        if actions.pressed(Actions::Forward) {
            new_mv.y += 1.0;
        }
        if actions.pressed(Actions::Backward) {
            new_mv.y -= 1.0;
        }
        if actions.pressed(Actions::Left) {
            new_mv.x -= 1.0;
        }
        if actions.pressed(Actions::Right) {
            new_mv.x += 1.0;
        }

        mv.0 = new_mv;

        if mv.0.y < 0.0 {
            *facing = Facing::Forward;
        } else if mv.0.y > 0.0 {
            *facing = Facing::Backward;
        }
    }
}
