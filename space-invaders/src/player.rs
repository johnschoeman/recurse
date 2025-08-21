use ::bevy::prelude::*;

use crate::bullet;
use crate::resolution;

pub struct PlayerPlugin;

const PLAYER_SPEED: f32 = 200.0;
const SHOOT_COOLDOWN: f32 = 0.5;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_player);
        app.add_systems(Update, update_player);
    }
}

#[derive(Component)]
struct Player {
    pub shoot_timer: f32,
}

fn setup_player(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    resolution: Res<resolution::Resolution>,
) {
    let player_image = asset_server.load("player.png");

    commands.spawn((
        Sprite {
            image: player_image.clone(),
            ..Default::default()
        },
        Transform::from_xyz(
            0.0,
            -(resolution.screen_dimensions.y * 0.5) + (resolution.pixel_ratio * 5.0),
            0.0,
        )
        .with_scale(Vec3::splat(resolution.pixel_ratio)),
        Player { shoot_timer: 0.0 },
    ));
}

fn update_player(
    mut commands: Commands,
    mut player_query: Query<(&mut Player, &mut Transform)>,
    time: Res<Time>,
    keys: Res<ButtonInput<KeyCode>>,
    resolution: Res<resolution::Resolution>,
) {
    let (mut player, mut player_transform) = player_query
        .single_mut()
        .expect("there should be exactly one Player");

    // Handle Left / Right Move
    let mut horizontal = 0.0;

    if keys.pressed(KeyCode::KeyA) || keys.pressed(KeyCode::ArrowLeft) {
        horizontal += -1.0;
    }

    if keys.pressed(KeyCode::KeyD) || keys.pressed(KeyCode::ArrowRight) {
        horizontal += 1.0;
    }

    let left_bound = -resolution.screen_dimensions.x * 0.5;
    let right_bound = resolution.screen_dimensions.x * 0.5;

    player_transform.translation.x +=
        (horizontal * PLAYER_SPEED * time.delta_secs()).clamp(left_bound, right_bound);

    player.shoot_timer -= time.delta_secs();

    // Handle Shoot
    if keys.pressed(KeyCode::Space) && player.shoot_timer <= 0.0 {
        player.shoot_timer = SHOOT_COOLDOWN;

        commands.spawn((
            Sprite {
                color: bullet::BULLET_COLOR,
                ..Default::default()
            },
            Transform {
                translation: player_transform.translation,
                scale: Vec3::new(bullet::BULLET_SIZE.x, bullet::BULLET_SIZE.y, 1.0),
                ..Default::default()
            },
            bullet::Bullet,
        ));
    }
}
