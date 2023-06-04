use bevy::prelude::*;
use crate::entities::{MotionVector, Facing};
use crate::player::Player;
use crate::math::{max, min};

pub fn keyboard_input(keys: Res<Input<KeyCode>>, mut query: Query<(&mut Facing, &mut MotionVector), With<Player>>) {
    let (mut facing, mut mv) = query.single_mut();
    if keys.pressed(KeyCode::Comma) {
        *facing = Facing::Backward;
        let new_y = min(mv.0.y + 1.0, 1.0);
        mv.0 = Vec2::new(mv.0.x, new_y);
    }
    
    if keys.pressed(KeyCode::O) {
        *facing = Facing::Forward;
        let new_y = max(mv.0.y - 1.0, -1.0);
        mv.0 = Vec2::new(mv.0.x, new_y);
    }

    if keys.pressed(KeyCode::A) {
        let new_x = max(mv.0.x - 1.0, -1.0);
        mv.0 = Vec2::new(new_x, mv.0.y);
    }

    if keys.pressed(KeyCode::E) {
        let new_x = min(mv.0.x + 1.0, 1.0);
        mv.0 = Vec2::new(new_x, mv.0.y);
    }

    if keys.just_released(KeyCode::Comma) {
        let new_y = min(mv.0.y - 1.0, 0.0);
        mv.0 = Vec2::new(mv.0.x, new_y);
    }

    if keys.just_released(KeyCode::O) {
        let new_y = max(mv.0.y - 1.0, 0.0);
        mv.0 = Vec2::new(mv.0.x, new_y);
    }

    if keys.just_released(KeyCode::E) {
        let new_x = max(mv.0.x - 1.0, 0.0);
        mv.0 = Vec2::new(new_x, mv.0.y);
    }

    if keys.just_released(KeyCode::A) {
        let new_x = min(mv.0.x + 1.0, 0.0);
        mv.0 = Vec2::new(new_x, mv.0.y);
    }
}
