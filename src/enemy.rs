use crate::entities::Health;
use bevy::prelude::*;
use bevy::sprite::collide_aabb::collide;

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
    commands.spawn((
        Enemy,
        EnemyType::Zchoop,
        ContactDamage(5.),
        SpriteBundle {
            texture: asset_server.load(bevy::asset::AssetPath::from_path(&asset_path)),
            transform: Transform::from_scale(Vec3::splat(3.0)),
            ..default()
        },
    ));
}

pub fn check_for_collisions(
    damager_query: Query<(&Transform, &ContactDamage, Entity)>,
    mut receiver_query: Query<(&Transform, &mut Health, Entity)>,
    time: Res<Time>,
) {
    for (transform, mut health, entity) in receiver_query.iter_mut() {
        for (d_transform, damage, d_entity) in damager_query.iter() {
            if entity == d_entity {
                continue;
            }

            let collision = collide(
                transform.translation,
                transform.scale.truncate(),
                d_transform.translation,
                d_transform.scale.truncate(),
            );

            let Some(_) = collision else {
                continue;
            };

            if let Some(last_hit) = health.last_hit {
                // entity may be in immunity time
                if last_hit <= health.immunity_time {
                    info!(
                        "IMMUNE for {} more seconds",
                        health.immunity_time - last_hit
                    );
                    continue;
                }
            }
            health.damage(damage.0);
            health.last_hit = Some(0.0);
            dbg!(&health.current);
        }
    }
}
