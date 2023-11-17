use crate::entities::{Health, ReceiveDamage};
use bevy::prelude::*;
use bevy::sprite::collide_aabb::collide;

#[derive(Component)]
pub enum EnemyType {
    Zchoop,
}

#[derive(Component)]
pub struct Enemy;

#[derive(Component)]
pub struct ContactDamage(pub i32);

pub fn spawn_enemy(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((
        Enemy,
        EnemyType::Zchoop,
        ContactDamage(5),
        SpriteBundle {
            texture: asset_server.load("sprites/Zchoop.png"),
            transform: Transform::from_scale(Vec3::splat(3.0)),
            ..default()
        },
    ));
}

pub fn check_for_collisions(
    damager_query: Query<(&Transform, &ContactDamage)>,
    mut receiver_query: Query<(&Transform, &mut Health), With<ReceiveDamage>>
) {
    for (transform, mut health) in receiver_query.iter_mut() {
        for (d_transform, damage) in damager_query.iter() {
            let collision = collide(
                transform.translation,
                transform.scale.truncate(),
                d_transform.translation,
                d_transform.scale.truncate(),
            );

            let Some(_) = collision else {
                continue;
            };

            health.0 -= damage.0;
            // dbg!(health.0);
        }
    }
}
