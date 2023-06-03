use bevy::prelude::*;

#[derive(Component)]
struct Player;

#[derive(Component)]
pub struct AnimationIndices {
    first: usize,
    last: usize,
}

#[derive(Component, Deref, DerefMut)]
pub struct AnimationTimer(Timer);

pub fn setup_player(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    let animation_indices = AnimationIndices { first: 0, last: 7 };
    commands.spawn((
        Player,
        SpriteSheetBundle {
            texture_atlas: create_player_texture_handle(asset_server, texture_atlases),
            sprite: TextureAtlasSprite::new(animation_indices.first),
            transform: Transform::from_scale(Vec3::splat(6.0)),
            ..default()
        },
        animation_indices,
        AnimationTimer(Timer::from_seconds(0.1, TimerMode::Repeating)),
    ));
}

pub fn animate_sprite(
    time: Res<Time>,
    mut query: Query<(
        &AnimationIndices,
        &mut AnimationTimer,
        &mut TextureAtlasSprite,
    )>,
) {
    for (indices, mut timer, mut sprite) in &mut query {
        timer.tick(time.delta());
        if timer.just_finished() {
            sprite.index = if sprite.index == indices.last {
                indices.first
            } else {
                sprite.index + 1
            };
        }
    }
}

fn create_player_texture_handle(
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) -> bevy::prelude::Handle<bevy::prelude::TextureAtlas> {
    let img_path = "sprites/Houston Front Walk Cycle.png".to_string();
    let texture_handle = asset_server.load(&img_path);
    let texture_atlas =
        TextureAtlas::from_grid(texture_handle, Vec2::new(32.0, 27.0), 1, 8, Some(Vec2::new(32.0, 1.0)), None);
    return texture_atlases.add(texture_atlas);
}
