use bevy::prelude::*;

use crate::resolution;

const ALIEN_SIZE: Vec2 = Vec2::new(15., 15.);
const ALIEN_COLOR: Color = Color::srgb(1., 1., 1.);

const ALIEN_INITIAL_SPEED: f32 = 20.;
pub const ALIEN_SPEED_INCREASE: f32 = 1.0;
pub const ALIEN_POINTS: usize = 10;

const ALIEN_COLUMNS: i32 = 10;
const ALIEN_ROWS: i32 = 5;
const ALIEN_SPACING: f32 = 2. * ALIEN_SIZE.x;
const ALIEN_SHIFT_AMOUNT: f32 = ALIEN_SIZE.y;

pub struct AlienPlugin;

impl Plugin for AlienPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(AlienSpeed(ALIEN_INITIAL_SPEED));
        app.add_systems(Startup, setup_aliens);
        app.add_systems(Update, (update_aliens, manage_alien_logic));
    }
}

#[derive(Resource, Deref, DerefMut)]
pub struct AlienSpeed(f32);

#[derive(Component)]
pub struct Alien {
    pub dead: bool,
    pub original_position: Vec3,
}

#[derive(Component)]
pub struct Dead;

#[derive(Resource)]
pub struct AlienManager {
    pub direction: f32,
    pub shift_aliens_down: bool,
    pub dist_from_boundary: f32,
    pub reset: bool,
}

fn setup_aliens(
    mut commands: Commands,
    _asset_server: Res<AssetServer>,
    resolution: Res<resolution::Resolution>,
) {
    commands.insert_resource(AlienManager {
        reset: false,
        dist_from_boundary: 0.0,
        shift_aliens_down: false,
        direction: 1.0,
    });

    // let alien_texture = asset_server.load("alien.png");

    for row_idx in 0..ALIEN_COLUMNS {
        for col_idx in 0..ALIEN_ROWS {
            let position = Vec3::new(
                row_idx as f32 * ALIEN_SPACING,
                col_idx as f32 * ALIEN_SPACING,
                0.0,
            ) - (Vec3::X * ALIEN_COLUMNS as f32 * ALIEN_SPACING * 0.5)
                - (Vec3::Y * ALIEN_ROWS as f32 * ALIEN_SPACING * 1.0)
                + (Vec3::Y * resolution.screen_dimensions.y * 0.5);

            commands.spawn((
                Sprite {
                    color: ALIEN_COLOR,
                    ..Default::default()
                },
                Transform {
                    translation: position,
                    scale: Vec3::new(ALIEN_SIZE.x, ALIEN_SIZE.y, 1.0),
                    ..Default::default()
                },
                Alien {
                    original_position: position,
                    dead: false,
                },
            ));
        }
    }
}

fn update_aliens(
    mut commands: Commands,
    mut alien_query: Query<(Entity, &Alien, &mut Transform, &mut Visibility), Without<Dead>>,
    mut alien_manager: ResMut<AlienManager>,
    speed: Res<AlienSpeed>,
    resolution: Res<resolution::Resolution>,
    time: Res<Time>,
) {
    for (entity, alien, mut transform, mut visibility) in alien_query.iter_mut() {
        transform.translation.x += time.delta_secs() * alien_manager.direction * **speed;

        if transform.translation.x.abs() > resolution.screen_dimensions.x * 0.5 {
            alien_manager.shift_aliens_down = true;

            alien_manager.dist_from_boundary =
                resolution.screen_dimensions.x * alien_manager.direction * 0.5
                    - transform.translation.x;
        }

        if alien.dead {
            commands.entity(entity).insert(Dead {});
            *visibility = Visibility::Hidden;
        } else {
            *visibility = Visibility::Visible;
        }

        if transform.translation.y < -resolution.screen_dimensions.y * 0.5 {
            alien_manager.reset = true;
        }
    }
}

fn manage_alien_logic(
    mut commands: Commands,
    mut alien_query: Query<(Entity, &mut Alien, &mut Transform)>,
    mut alien_manager: ResMut<AlienManager>,
) {
    if alien_manager.shift_aliens_down {
        // In line this to update_alien?
        alien_manager.shift_aliens_down = false;
        alien_manager.direction *= -1.0;

        for (_entity, _alien, mut transform) in alien_query.iter_mut() {
            transform.translation.x += alien_manager.dist_from_boundary;
            transform.translation.y -= ALIEN_SHIFT_AMOUNT;
        }
    }

    if alien_manager.reset {
        alien_manager.reset = false;
        alien_manager.direction = 1.0;
        for (entity, mut alien, mut transform) in alien_query.iter_mut() {
            transform.translation = alien.original_position;
            if alien.dead {
                alien.dead = false;
                commands.entity(entity).remove::<Dead>();
            }
        }
    }
}
