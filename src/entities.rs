use bevy::prelude::*;

#[derive(Component, Debug)]
pub enum Facing {
    Forward,
    Backward,
}

impl Facing {
    pub fn swap(&mut self) -> Self {
        return match self {
            Facing::Forward => Facing::Backward,
            Facing::Backward => Facing::Forward,
        };
    }
}

#[derive(Component)]
pub struct MoveVector(pub Vec2);

#[derive(Component)]
pub struct MoveSpeed(pub f32);

#[derive(Component, Debug)]
pub struct Health {
    pub max: f32,
    pub current: f32,
    pub last_hit: Option<f32>,
    pub immunity_time: f32,
}

impl Health {
    pub fn new(max_health: f32, immunity_time: f32) -> Self {
        Health {
            max: max_health,
            current: max_health,
            last_hit: None,
            immunity_time,
        }
    }

    pub fn damage(&mut self, amount: f32) {
        self.current -= amount;
    }
}

#[derive(Component)]
pub struct EntityAtlas {
    pub forward: Handle<TextureAtlas>,
    pub backward: Handle<TextureAtlas>,
}

pub fn sprite_facing(
    mut query: Query<(&Facing, &EntityAtlas, &mut Handle<TextureAtlas>)>,
) {
    for (facing, atlas, mut handle) in query.iter_mut() {
        match facing {
            Facing::Forward => {
                *handle = atlas.forward.clone();
            }
            Facing::Backward => {
                *handle = atlas.backward.clone();
            }
        }
    }
}

pub fn move_entities(
    mut query: Query<(&mut Transform, &MoveVector, Option<&MoveSpeed>)>,
) {
    for (mut transform, mv, movement_speed) in query.iter_mut() {
        let mut ms: f32 = 1.0;

        if let Some(val) = movement_speed {
            ms = val.0;
        }
        transform.translation.x += mv.0.x * ms;
        transform.translation.y += mv.0.y * ms;
    }
}

pub fn tick_health(mut query: Query<&mut Health>, time: Res<Time>) {
    for mut health in query.iter_mut() {
        if let Some(last_hit) = health.last_hit {
            let new = last_hit + time.delta_seconds();
            health.last_hit = Some(new);
        }
    }
}

pub fn check_alive(mut commands: Commands, mut query: Query<(&mut Health, Entity)>) {
    for (health, entity) in query.iter_mut() {
        if health.current <= 0.0 {
            commands.entity(entity).despawn_recursive();
        }
    }
}
