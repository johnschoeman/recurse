use bevy::prelude::*;

use crate::alien;
use crate::bullet;
use crate::player;
use crate::resolution;
use crate::scoreboard;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            alien::AlienPlugin,
            resolution::ResolutionPlugin,
            player::PlayerPlugin,
            bullet::BulletPlugin,
            scoreboard::ScoreboardPlugin,
        ));
        app.add_systems(Startup, setup_scene);
    }
}

fn setup_scene(mut commands: Commands) {
    commands.spawn(Camera2d::default());
}
