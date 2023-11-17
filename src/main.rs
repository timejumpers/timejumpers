mod enemy;
mod math;
mod animation;
mod player;
mod entities;
mod control;
mod multiplayer;

use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .insert_resource(multiplayer::NumPlayers(2))
        .add_plugin(player::PlayerSetup)

        .add_startup_system(setup)
        .add_startup_system(enemy::spawn_enemy)

        .add_system(animation::animate_sprites)
        .add_system(control::handle_input)
        .add_system(entities::sprite_facing)
        .add_system(entities::move_entities)
        .add_system(enemy::check_for_collisions)

        .run()
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

