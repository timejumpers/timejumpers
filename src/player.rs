use bevy::prelude::*;

#[derive(Component)]
struct Player;

pub fn setup_player(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((
        Player,
        SpriteBundle {
            texture: asset_server.load("sprites/Zchoop.png"),
            ..default()
        },
    ));
}

pub fn make_tiles(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    let img_path = "sprites/Houston Front Walk Cycle.png".to_string();
    let texture_handle = asset_server.load(&img_path);
    let texture_atlas =
        TextureAtlas::from_grid(texture_handle, Vec2::new(16.0, 16.0), 1, 5, None, None);
    let texture_atlas_handle = texture_atlases.add(texture_atlas);
    for i in 0..10 {
        commands.spawn(SpriteSheetBundle {
            sprite: TextureAtlasSprite {
                index: 0,
                ..default()
            },
            texture_atlas: texture_atlas_handle.clone(),
            ..default()
        });
    }
}
