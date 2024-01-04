use bevy::prelude::*;

pub fn move_camera(mut camera_query: Query<&mut Transform, With<Camera>>) {
    for mut transform in camera_query.iter_mut() {
        transform.translation.y += 0.0;
    }
}
