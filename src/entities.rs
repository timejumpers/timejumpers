use bevy::prelude::*;

#[derive(Component, Debug)]
pub enum Facing {
    Forward,
    Backward,
}

#[derive(Component)]
pub struct MotionVector(pub Vec2);

impl Facing {
    pub fn swap(&mut self) -> Self {
        return match self {
            Facing::Forward => Facing::Backward,
            Facing::Backward => Facing::Forward,
        };
    }
}

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

pub fn move_entities(mut query: Query<(&mut Transform, &MotionVector)>) {
    for (mut transform, mv) in query.iter_mut() {
        transform.translation.x += mv.0.x;
        transform.translation.y += mv.0.y;
    }
}
