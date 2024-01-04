use bevy::prelude::*;

use crate::actors::damage::DamageMask;
use crate::{actors::enemy::ContactDamage, actors::MoveVector, assets::AssetPath};

#[derive(Event)]
pub struct SpawnProjectile {
    pub origin: Vec3,
    pub speed: f32,
    pub kind: ProjectileKind,
    pub mask: DamageMask,
}

pub enum ProjectileKind {
    MusketBall,
    Arrow,
}

pub fn spawn_projectiles(
    mut events: EventReader<SpawnProjectile>,
    mut commands: Commands,
    asset_path: Res<AssetPath>,
    asset_server: Res<AssetServer>,
) {
    for ev in events.read() {
        match ev.kind {
            ProjectileKind::MusketBall => {
                let asset_path = asset_path.0.join("sprites").join("musket_ball.png");
                commands.spawn((
                    SpriteBundle {
                        texture: asset_server
                            .load(bevy::asset::AssetPath::from_path(&asset_path)),
                        transform: Transform::from_translation(
                            ev.origin + Vec3::new(0.0, 0.0, 0.0),
                        ),
                        ..default()
                    },
                    MoveVector(Vec2::new(0.0, ev.speed)),
                    ContactDamage(25.0),
                    ev.mask,
                ));
            }
            ProjectileKind::Arrow => todo!(),
        }
    }
}
