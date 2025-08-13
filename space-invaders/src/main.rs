use bevy::{
    ecs::schedule::{ExecutorKind, ScheduleLabel},
    prelude::*,
    time::common_conditions::on_timer,
    window::PrimaryWindow,
};

use std::time::Duration;

// Frame
const WINDOW_HEIGHT: f32 = 580.0;
const WINDOW_WIDTH: f32 = 1000.0;

// Player
const PLAYER_SIZE: Vec2 = Vec2::new(50.0, 20.0);
const GAP_BETWEEN_PLAYER_AND_BOTTOM: f32 = 30.0;
const PLAYER_SPEED: f32 = 500.0;
const PLAYER_PADDING: f32 = 10.0;

// Bullets
const BULLET_SIZE: Vec2 = Vec2::new(5.0, 15.0);
const BULLET_SPEED: f32 = 100.0;

// Ships
const SHIP_SIZE: Vec2 = Vec2::new(25.0, 25.0);
const GAP_BETWEEN_SHIPS_AND_TOP: f32 = 25.0;
const GAP_BETWEEN_SHIPS_AND_SIDE: f32 = 25.0;
const GAP_BETWEEN_SHIPS_ROW: f32 = 25.0;
const GAP_BETWEEN_SHIPS_COL: f32 = 25.0;
const SHIP_INITIAL_SPEED: f32 = 500.0;
const SHIP_PADDING: f32 = 5.0;

const SHIP_COLS: u8 = 11;
const SHIP_ROWS: u8 = 5;

const INITIAL_SHIP_DURATION: u64 = 2;

// Colors
// const BACKGROUND_COLOR: Color = Color::srgb(0.9, 0.9, 0.9);
const PLAYER_COLOR: Color = Color::srgb(0.3, 0.7, 0.3);
const BULLET_COLOR: Color = Color::srgb(1.0, 1.0, 1.0);
const SHIP_COLOR: Color = Color::srgb(0.7, 0.3, 0.3);
const TEXT_COLOR: Color = Color::srgb(0.5, 0.5, 1.0);
const SCORE_COLOR: Color = Color::srgb(1.0, 0.5, 0.5);

pub struct SpaceInvaderPlugin;

impl Plugin for SpaceInvaderPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(Score(0));
        app.insert_resource(Direction::Right);
        app.add_systems(Startup, (setup_camera, setup_ships, setup_player));
        app.add_systems(
            FixedUpdate,
            (
                move_player,
                fire_bullet,
                move_ships.run_if(on_timer(Duration::from_secs_f32(2.0))),
                move_bullets,
            ),
        );
    }
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(SpaceInvaderPlugin)
        .run();
}

#[derive(Component)]
struct Player;

#[derive(Component)]
struct Ship;

#[derive(Component)]
struct Bullet;

#[derive(Resource, Deref, DerefMut)]
struct Score(usize);

#[derive(Resource)]
enum Direction {
    Right,
    Left,
}

#[derive(Component, Default)]
struct Collider;

fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2d);
}

fn setup_player(
    mut commands: Commands,
    mut _meshes: ResMut<Assets<Mesh>>,
    mut _materials: ResMut<Assets<ColorMaterial>>,
    _asset_server: Res<AssetServer>,
) {
    let player_y = WINDOW_HEIGHT / -2.0 + GAP_BETWEEN_PLAYER_AND_BOTTOM;

    commands.spawn((
        Sprite::from_color(PLAYER_COLOR, Vec2::ONE),
        Transform {
            translation: Vec3::new(0.0, player_y, 0.0),
            scale: PLAYER_SIZE.extend(1.0),
            ..default()
        },
        Player,
        Collider,
    ));
}

fn setup_ships(
    mut commands: Commands,
    mut _meshes: ResMut<Assets<Mesh>>,
    mut _materials: ResMut<Assets<ColorMaterial>>,
    _asset_server: Res<AssetServer>,
) {
    let ship_offset_x = WINDOW_WIDTH / -2.0 + GAP_BETWEEN_SHIPS_AND_SIDE;
    let ship_offset_y = WINDOW_HEIGHT / 2.0 - GAP_BETWEEN_SHIPS_AND_TOP;

    for row_idx in 0..SHIP_ROWS {
        for col_idx in 0..SHIP_COLS {
            let ship_position = Vec2::new(
                ship_offset_x + (col_idx as f32) * (SHIP_SIZE.x + GAP_BETWEEN_SHIPS_ROW),
                ship_offset_y - (row_idx as f32) * (SHIP_SIZE.y + GAP_BETWEEN_SHIPS_COL),
            );

            commands.spawn((
                Sprite::from_color(SHIP_COLOR, Vec2::ONE),
                Transform {
                    translation: ship_position.extend(0.0),
                    scale: SHIP_SIZE.extend(1.0),
                    ..default()
                },
                Ship,
                Collider,
            ));
        }
    }
}

fn move_player(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut player_transform: Single<&mut Transform, With<Player>>,
    time: Res<Time>,
) {
    let mut direction = 0.0;

    if keyboard_input.pressed(KeyCode::ArrowLeft) {
        direction -= 1.0;
    }

    if keyboard_input.pressed(KeyCode::ArrowRight) {
        direction += 1.0;
    }

    let next_player_position =
        player_transform.translation.x + direction * PLAYER_SPEED * time.delta_secs();

    let left_bound = WINDOW_WIDTH / -2.0;
    let right_bound = WINDOW_WIDTH / 2.0;

    player_transform.translation.x = next_player_position.clamp(left_bound, right_bound);
}

fn fire_bullet(
    mut commands: Commands,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut player_transform: Single<&mut Transform, With<Player>>,
) {
    if keyboard_input.pressed(KeyCode::Space) {
        let bullet_x = player_transform.translation.x;
        let bullet_y = player_transform.translation.y;
        commands.spawn((
            Sprite::from_color(BULLET_COLOR, Vec2::ONE),
            Transform {
                translation: Vec3::new(bullet_x, bullet_y, 0.0),
                scale: BULLET_SIZE.extend(1.0),
                ..default()
            },
            Bullet,
            Collider,
        ));
    }
}

fn move_bullets(
    mut commands: Commands,
    mut bullets: Query<(Entity, &mut Transform), With<Bullet>>,
    time: Res<Time>,
) {
    let top_bound = WINDOW_HEIGHT / 2.0;
    let bottom_bound = WINDOW_HEIGHT / -2.0;

    for (bullet, mut bullet_transform) in bullets.iter_mut() {
        let next_bullet_position =
            bullet_transform.translation.y + BULLET_SPEED * time.delta_secs();

        if next_bullet_position < top_bound {
            bullet_transform.translation.y = next_bullet_position;
        } else {
            commands.entity(bullet).despawn();
        }
    }
}

fn move_ships(
    mut ships: Query<&mut Transform, With<Ship>>,
    mut direction: ResMut<Direction>,
    _time: Res<Time>,
) {
    let left_bound = WINDOW_WIDTH / -2.0 + GAP_BETWEEN_SHIPS_COL;
    let right_bound = WINDOW_WIDTH / 2.0 - GAP_BETWEEN_SHIPS_COL;
    let mut should_move_down: bool = false;

    let mut max_ship_x_pos: f32 = 0.0;
    let mut min_ship_x_pos: f32 = 0.0;

    for ship in ships.iter() {
        if ship.translation.x > max_ship_x_pos {
            max_ship_x_pos = ship.translation.x;
        }
        if ship.translation.x < min_ship_x_pos {
            min_ship_x_pos = ship.translation.x;
        }
    }

    if max_ship_x_pos >= right_bound {
        *direction = Direction::Left;
        should_move_down = true;
    }
    if min_ship_x_pos <= left_bound {
        *direction = Direction::Right;
        should_move_down = true;
    }

    let dir_value = match *direction {
        Direction::Left => -1.0,
        Direction::Right => 1.0,
    };

    for mut ship_transform in ships.iter_mut() {
        let next_ship_position = ship_transform.translation.x + dir_value * 10.0;

        ship_transform.translation.x = next_ship_position.clamp(left_bound, right_bound);

        if should_move_down {
            let next_ship_y = ship_transform.translation.y - 10.0;
            ship_transform.translation.y = next_ship_y;
        }
    }
}
