use bevy::prelude::*;

use crate::animation::{AnimationIndices, AnimationTimer};
use crate::entities::{EntityAtlas, MoveVector, Facing, MoveSpeed};

const FRONT_WALK_CYCLE_PATH: &str = "sprites/Houston Front Walk Cycle.png";
const BACK_WALK_CYCLE_PATH: &str = "sprites/Houston Back Walk Cycle.png";

#[derive(Component)]
pub struct Player;

fn setup_player(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    let animation_indices = AnimationIndices { first: 0, last: 7 };
    let front = create_front_texture_atlas(&asset_server, &mut texture_atlases);
    let back = create_back_texture_atlas(&asset_server, &mut texture_atlases);
    commands.spawn((
        Player,
        EntityAtlas {
            forward: front.clone(),
            backward: back,
        },
        Facing::Forward,
        MoveVector(Vec2::new(0.0, 0.0)),
        MoveSpeed(5.0),
        SpriteSheetBundle {
            texture_atlas: front,
            sprite: TextureAtlasSprite::new(animation_indices.first),
            transform: Transform::from_scale(Vec3::splat(6.0)),
            ..default()
        },
        animation_indices,
        AnimationTimer(Timer::from_seconds(0.15, TimerMode::Repeating)),
    ));
}

fn create_front_texture_atlas(
    asset_server: &Res<AssetServer>,
    texture_atlases: &mut ResMut<Assets<TextureAtlas>>,
) -> bevy::prelude::Handle<bevy::prelude::TextureAtlas> {
    let texture_handle = asset_server.load(FRONT_WALK_CYCLE_PATH.to_string());
    let texture_atlas = TextureAtlas::from_grid(
        texture_handle,
        Vec2::new(32.0, 27.0),
        1,
        8,
        Some(Vec2::new(32.0, 1.0)),
        None,
    );

    return texture_atlases.add(texture_atlas);
}

fn create_back_texture_atlas(
    asset_server: &Res<AssetServer>,
    texture_atlases: &mut ResMut<Assets<TextureAtlas>>,
) -> Handle<TextureAtlas> {
    let back_handle = asset_server.load(BACK_WALK_CYCLE_PATH.to_string());
    let back_atlas = TextureAtlas::from_grid(
        back_handle,
        Vec2::new(32.0, 27.0),
        1,
        8,
        Some(Vec2::new(32.0, 1.0)),
        None,
    );

    return texture_atlases.add(back_atlas);
}

pub struct PlayerSetup;

impl Plugin for PlayerSetup {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup_player);
    }
}
