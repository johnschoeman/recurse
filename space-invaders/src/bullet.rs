use bevy::math::bounding::{Aabb2d, IntersectsVolume};
use bevy::prelude::*;

use crate::alien;
use crate::resolution;
use crate::scoreboard;

pub const BULLET_SIZE: Vec2 = Vec2::new(3.0, 8.0);
pub const BULLET_COLOR: Color = Color::srgb(1.0, 1.0, 1.0);
const BULLET_SPEED: f32 = 400.0;

pub struct BulletPlugin;

impl Plugin for BulletPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (update_bullets, update_hits));
    }
}

#[derive(Component)]
pub struct Bullet;

fn update_bullets(
    mut commands: Commands,
    mut bullets: Query<(Entity, &mut Transform), With<Bullet>>,
    time: Res<Time>,
    resolution: Res<resolution::Resolution>,
) {
    for (entity, mut bullet_transform) in bullets.iter_mut() {
        bullet_transform.translation.y += BULLET_SPEED * time.delta_secs();

        if bullet_transform.translation.y > resolution.screen_dimensions.y * 0.5 {
            commands.entity(entity).despawn();
        }
    }
}

fn update_hits(
    mut commands: Commands,
    mut alien_speed: ResMut<alien::AlienSpeed>,
    mut score: ResMut<scoreboard::Score>,
    mut aliens: Query<(&mut alien::Alien, &Transform), Without<alien::Dead>>,
    bullets: Query<(Entity, &Transform), With<Bullet>>,
) {
    for (bullet, bullet_transform) in bullets.iter() {
        let bullet_bounding_box = Aabb2d::new(
            bullet_transform.translation.truncate(),
            bullet_transform.scale.truncate() / 2.0,
        );

        for (mut alien, alien_transform) in aliens.iter_mut() {
            let ship_bounding_box = Aabb2d::new(
                alien_transform.translation.truncate(),
                alien_transform.scale.truncate() / 2.0,
            );

            if ship_bounding_box.intersects(&bullet_bounding_box) {
                alien.dead = true; // Maybe despawn instead?
                commands.entity(bullet).despawn();
                **alien_speed += alien::ALIEN_SPEED_INCREASE;
                **score += alien::ALIEN_POINTS;
                break; // bullet can only hit one alien
            }
        }
    }
}
