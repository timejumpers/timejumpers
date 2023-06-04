mod math;
mod animation;
mod player;
mod entities;
mod control;

use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .add_plugin(player::PlayerSetup)

        .add_startup_system(setup)

        .add_system(animation::animate_sprites)
        .add_system(control::keyboard_input)
        .add_system(entities::sprite_facing)
        .add_system(entities::move_entities)
        .run()
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

