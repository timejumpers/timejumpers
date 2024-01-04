use crate::actors::Health;
use bevy::prelude::*;

#[derive(Component)]
pub struct BarFill;

#[derive(Component)]
pub struct BarEmpty;

#[derive(Component)]
pub struct BarLag {
    pub current_percent: f32,
    pub target_percent: f32,
}

const HEALTH_BAR_HEIGHT: f32 = 2.5;
const HEALTH_BAR_WIDTH: f32 = 50.0;

pub fn create_health_bar(commands: &mut Commands) -> Entity {
    let filled = commands
        .spawn((
            SpriteBundle {
                sprite: Sprite {
                    color: Color::RED,
                    custom_size: Some(Vec2::new(HEALTH_BAR_WIDTH, HEALTH_BAR_HEIGHT)),
                    ..default()
                },
                transform: Transform::from_translation(Vec3::new(0., 20., 10.)),
                ..default()
            },
            BarFill,
        ))
        .id();
    let lag = commands
        .spawn((
            SpriteBundle {
                sprite: Sprite {
                    color: Color::WHITE,
                    custom_size: Some(Vec2::new(HEALTH_BAR_WIDTH, HEALTH_BAR_HEIGHT)),
                    ..default()
                },
                transform: Transform::from_translation(Vec3::new(0., 0., -1.)),
                ..default()
            },
            BarLag {
                current_percent: 1.0,
                target_percent: 1.0,
            },
        ))
        .id();
    let empty = commands
        .spawn((
            SpriteBundle {
                sprite: Sprite {
                    color: Color::BLACK,
                    custom_size: Some(Vec2::new(HEALTH_BAR_WIDTH, HEALTH_BAR_HEIGHT)),
                    ..default()
                },
                transform: Transform::from_translation(Vec3::new(0., 0., -10.)),
                ..default()
            },
            BarEmpty,
        ))
        .id();

    commands.entity(filled).add_child(lag).add_child(empty);
    return filled;
}

pub fn update_health_bars(
    health_query: Query<(&Health, &Children)>,
    mut fill: Query<
        (&mut Sprite, &Children),
        (With<BarFill>, Without<BarEmpty>, Without<BarLag>),
    >,
    mut lag: Query<(&mut Sprite, &mut BarLag), (Without<BarFill>, Without<BarEmpty>)>,
    /*empty: Query<&mut Sprite, (With<BarEmpty>, Without<BarFill>)>,*/
) {
    for (health, children) in health_query.iter() {
        for &child in children.iter() {
            let (mut filled, bar_children) = fill.get_mut(child).unwrap();
            let percent = health.current / health.max;
            let size = 50.0 * percent;
            filled.custom_size = Some(Vec2::new(size, HEALTH_BAR_HEIGHT));

            let bar_child = bar_children.first().unwrap();
            let (mut lag_sprite, mut lag) = lag.get_mut(*bar_child).unwrap();
            lag.target_percent = percent;

            if lag.current_percent > lag.target_percent {
                lag.current_percent -= 0.005;
                lag_sprite.custom_size =
                    Some(Vec2::new(lag.current_percent * 50.0, HEALTH_BAR_HEIGHT));
            }
        }
    }
}
