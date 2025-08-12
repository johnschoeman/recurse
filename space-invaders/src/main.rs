use bevy::prelude::*;

const PLAYER_SIZE: Vec2 = Vec2::new(120.0, 20.0);
const GAP_BETWEEN_PLAYER_AND_FLOOR: f32 = 10.0;
const PLAYER_SPEED: f32 = 500.0;
const PLAYER_PADDING: f32 = 10.0;

const BACKGROUND_COLOR: Color = Color::srgb(0.9, 0.9, 0.9);
const PLAYER_COLOR: Color = Color::srgb(0.3, 0.3, 0.7);
const TEXT_COLOR: Color = Color::srgb(0.5, 0.5, 1.0);
const SCORE_COLOR: Color = Color::srgb(1.0, 0.5, 0.5);

pub struct SpaceInvaderPlugin;

impl Plugin for SpaceInvaderPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(Score(0));
        app.add_systems(Startup, setup);
        app.add_systems(FixedUpdate, move_player);
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

#[derive(Resource, Deref, DerefMut)]
struct Score(usize);

#[derive(Component, Default)]
struct Collider;

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    asset_server: Res<AssetServer>,
) {
    commands.spawn(Camera2d);

    let player_y = GAP_BETWEEN_PLAYER_AND_FLOOR;

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

    let left_bound = -70.0;
    let right_bound = 70.0;

    player_transform.translation.x = next_player_position.clamp(left_bound, right_bound);
}
