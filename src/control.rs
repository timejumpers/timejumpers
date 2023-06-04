use bevy::prelude::*;
use crate::entities::Facing;

pub fn keyboard_input(keys: Res<Input<KeyCode>>, mut query: Query<&mut Facing, With<crate::player::Player>>) {
    if keys.just_pressed(KeyCode::Space) {
        let mut facing = query.single_mut();
        *facing = facing.swap(); 
    }
}
