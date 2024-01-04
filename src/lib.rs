#![warn(clippy::all)]
#![warn(clippy::cargo)]
#![allow(
    clippy::needless_return,
    clippy::multiple_crate_versions,
    clippy::too_many_arguments,
    clippy::type_complexity
)]
#![warn(dead_code)]

pub mod animation;
pub mod assets;
pub mod camera;
pub mod control;
pub mod damage;
pub mod enemy;
pub mod entities;
pub mod game;
pub mod multiplayer;
pub mod player;
pub mod projectile;
pub mod ui;
