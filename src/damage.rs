use crate::entities::Health;
use bevy::prelude::*;
use bevy::sprite::collide_aabb::collide;

use crate::enemy::{ContactDamage, Enemy};
use crate::player::PlayerId;

#[derive(Clone, Copy, Component)]
pub enum DamageMask {
    IgnoresEnemies,
    IgnoresPlayers,
    IgnoresBoth,
}

pub fn check_for_collisions(
    damager_query: Query<(&Transform, &ContactDamage, &DamageMask, Entity)>,
    mut receiver_query: Query<(
        &Transform,
        &mut Health,
        Option<&PlayerId>,
        Option<&Enemy>,
        Entity,
    )>,
) {
    for (transform, mut health, opt_player, opt_enemy, entity) in
        receiver_query.iter_mut()
    {
        for (d_transform, damage, mask, d_entity) in damager_query.iter() {
            if entity == d_entity {
                continue;
            }

            match mask {
                DamageMask::IgnoresEnemies => {
                    if opt_enemy.is_some() {
                        continue;
                    }
                }
                DamageMask::IgnoresPlayers => {
                    if opt_player.is_some() {
                        continue;
                    }
                }
                DamageMask::IgnoresBoth => {
                    if opt_player.is_some() || opt_enemy.is_some() {
                        continue;
                    }
                }
            }

            let collision = collide(
                transform.translation,
                Vec2::new(18., 18.),
                d_transform.translation,
                Vec2::new(18., 18.),
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
            info!("Damaged {:?} for {}", entity, damage.0);
        }
    }
}
