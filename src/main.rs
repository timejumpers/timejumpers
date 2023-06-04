mod enemy;
mod math;
mod animation;
mod player;
mod entities;
mod control;

use bevy::prelude::*;

#[derive(Default)]
pub struct CollisionEvent;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .add_plugin(player::PlayerSetup)

        .add_startup_system(setup)
        .add_startup_system(enemy::spawn_enemy)

        .add_system(animation::animate_sprites)
        .add_system(control::keyboard_input)
        .add_system(entities::sprite_facing)
        .add_system(entities::move_entities)
        .add_system(enemy::check_for_collisions)

        .add_event::<CollisionEvent>()
        .run()
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

