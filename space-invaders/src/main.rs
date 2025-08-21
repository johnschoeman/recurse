use bevy::prelude::*;

const WINDOW_HEIGHT: f32 = 512.0;
const WINDOW_WIDTH: f32 = 512.0;

pub mod alien;
pub mod bullet;
pub mod game;
pub mod player;
pub mod resolution;
pub mod scoreboard;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: String::from("Space Invaders"),
                        position: WindowPosition::Centered(MonitorSelection::Primary),
                        resolution: Vec2::new(WINDOW_HEIGHT, WINDOW_WIDTH).into(),
                        ..Default::default()
                    }),
                    ..Default::default()
                })
                .set(ImagePlugin::default_nearest()),
            game::GamePlugin,
        ))
        .run();
}
