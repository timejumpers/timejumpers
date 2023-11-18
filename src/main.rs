use bevy::prelude::*;
use timejumpers::{
    animation, assets, control, enemy, entities, multiplayer, player,
};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .insert_resource(multiplayer::NumPlayers(2))
        .insert_resource(assets::AssetPath("./".to_string()))
        .add_systems(PreStartup, (assets::set_asset_path, setup, enemy::spawn_enemy))
        .add_plugins(player::PlayerSetup)
        .add_systems(Update, (animation::animate_sprites, control::handle_input, entities::sprite_facing, entities::move_entities, enemy::check_for_collisions))
        .run()
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}
