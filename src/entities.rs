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

#[derive(Component)]
pub struct Health(pub i32);

#[derive(Component)]
pub struct ReceiveDamage;

#[derive(Component)]
pub struct EntityAtlas {
    pub forward: Handle<TextureAtlas>,
    pub backward: Handle<TextureAtlas>,
}

pub fn sprite_facing(mut query: Query<(&Facing, &EntityAtlas, &mut Handle<TextureAtlas>)>) {
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

pub fn move_entities(mut query: Query<(&mut Transform, &MoveVector, Option<&MoveSpeed>)>) {
    for (mut transform, mv, movement_speed) in query.iter_mut() {
        let mut ms: f32 = 1.0;

        if let Some(val) = movement_speed {
            ms = val.0;
        }
        transform.translation.x += mv.0.x * ms;
        transform.translation.y += mv.0.y * ms;
    }
}
