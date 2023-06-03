mod player;

use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .add_startup_system(setup)
        .add_startup_system(player::setup_player)
        .add_system(player::animate_sprite)
        .run()
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

