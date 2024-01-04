use action_maps::multiplayer_prelude::*;
use bevy::prelude::*;
use timejumpers::{
    animation, assets, control, damage, enemy, entities, multiplayer, player,
    projectile, ui,
};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .add_plugins(MultiActionMapPlugin)
        .insert_resource(multiplayer::NumPlayers(2))
        .insert_resource(assets::AssetPath(std::path::PathBuf::from("./")))
        .add_event::<projectile::SpawnProjectile>()
        .add_systems(PreStartup, assets::set_asset_path)
        .add_systems(Startup, (setup, enemy::spawn_enemy, control::bind_keys))
        .add_plugins(player::PlayerSetup)
        .add_systems(
            PreUpdate,
            (
                control::handle_input.in_set(ActionMapSet::HandleActions),
                timejumpers::camera::move_camera,
            ),
        )
        .add_systems(
            Update,
            (
                animation::animate_sprites,
                entities::sprite_facing,
                entities::move_entities,
                entities::tick_health,
                damage::check_for_collisions,
                projectile::spawn_projectiles,
                ui::update_health_bars,
            ),
        )
        .add_systems(PostUpdate, entities::check_alive)
        .run()
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}
