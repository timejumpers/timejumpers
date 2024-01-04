use crate::{entities::Health, damage::DamageMask};
use bevy::prelude::*;

#[derive(Component)]
pub enum EnemyType {
    Zchoop,
}

#[derive(Component)]
pub struct Enemy;

#[derive(Component, Debug)]
pub struct ContactDamage(pub f32);

pub fn spawn_enemy(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    asset_path_res: Res<crate::assets::AssetPath>,
) {
    let asset_path = asset_path_res.0.join("sprites").join("Zchoop.png");
    let enemy = commands.spawn((
        Enemy,
        EnemyType::Zchoop,
        ContactDamage(20.),
        DamageMask::IgnoresEnemies,
        Health::new(125.0, 0.25),
        SpriteBundle {
            texture: asset_server.load(bevy::asset::AssetPath::from_path(&asset_path)),
            transform: Transform::from_translation(Vec3::new(-75.0, 0.0, 0.0))
                .with_scale(Vec3::splat(3.0)),
            ..default()
        },

    )).id();

    let health_bar = crate::ui::create_health_bar(&mut commands);
    commands.entity(enemy).add_child(health_bar);
}
