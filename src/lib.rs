#![warn(clippy::all)]
#![warn(clippy::cargo)]
#![allow(
    clippy::needless_return,
    clippy::multiple_crate_versions,
    clippy::too_many_arguments,
    clippy::type_complexity
)]
#![warn(dead_code)]

pub mod actors;
pub mod assets;
pub mod camera;
pub mod control;
pub mod game;
pub mod graphics;
pub mod multiplayer;
pub mod ui;
