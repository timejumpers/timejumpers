use bevy::prelude::*;

use crate::animation::{AnimationIndices, AnimationTimer};
use crate::entities::{EntityAtlas, Facing, Health, MoveSpeed, MoveVector};
use crate::ui;

const FRONT_WALK_CYCLE_PATH: &str = "Houston Front Walk Cycle.png";
const BACK_WALK_CYCLE_PATH: &str = "Houston Back Walk Cycle.png";
const SPRITE_SHEET_NUM_COLUMNS: usize = 1;
const SPRITE_SHEET_NUM_ROWS: usize = 8;
const FRAME_WIDTH: f32 = 32.0;
const FRAME_HEIGHT: f32 = 27.0;
const NUM_PAD_ROWS: f32 = 1.0;

const ANIMATION_FRAME_TIME: f32 = 0.15;

const HEALTH: f32 = 100.0;
const MOVESPEED: f32 = 5.0;

#[derive(Component)]
pub struct PlayerId(pub usize);

fn setup_player(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    asset_path_res: Res<crate::assets::AssetPath>,
) {
    let animation_indices = AnimationIndices {
        first: 0,
        last: SPRITE_SHEET_NUM_ROWS - 1,
    };
    let front = create_texture_atlas(
        FRONT_WALK_CYCLE_PATH,
        &asset_server,
        &mut texture_atlases,
        &asset_path_res,
    );
    let back = create_texture_atlas(
        BACK_WALK_CYCLE_PATH,
        &asset_server,
        &mut texture_atlases,
        &asset_path_res,
    );

    let player = commands
        .spawn((
            PlayerId(0),
            Health::new(HEALTH, 0.25),
            EntityAtlas {
                forward: front.clone(),
                backward: back.clone(),
            },
            Facing::Forward,
            MoveVector(Vec2::new(0.0, 0.0)),
            MoveSpeed(MOVESPEED),
            SpriteSheetBundle {
                texture_atlas: front.clone(),
                sprite: TextureAtlasSprite::new(animation_indices.clone().first),
                transform: Transform::from_scale(Vec3::splat(2.0)),
                ..default()
            },
            animation_indices.clone(),
            AnimationTimer(Timer::from_seconds(
                ANIMATION_FRAME_TIME,
                TimerMode::Repeating,
            )),
        ))
        .id();

    let health_bar = ui::create_health_bar(&mut commands);
    commands.entity(player).add_child(health_bar);
}

fn create_texture_atlas(
    asset_path: &str,
    asset_server: &Res<AssetServer>,
    texture_atlases: &mut ResMut<Assets<TextureAtlas>>,
    asset_path_res: &Res<crate::assets::AssetPath>,
) -> bevy::prelude::Handle<bevy::prelude::TextureAtlas> {
    let asset_path = asset_path_res.0.join("sprites").join(asset_path);
    let texture_handle =
        asset_server.load(bevy::asset::AssetPath::from_path(&asset_path));
    let texture_atlas = TextureAtlas::from_grid(
        texture_handle,
        Vec2::new(FRAME_WIDTH, FRAME_HEIGHT),
        SPRITE_SHEET_NUM_COLUMNS,
        SPRITE_SHEET_NUM_ROWS,
        Some(Vec2::new(FRAME_WIDTH, NUM_PAD_ROWS)),
        None,
    );

    return texture_atlases.add(texture_atlas);
}

pub struct PlayerSetup;

impl Plugin for PlayerSetup {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_player);
    }
}
