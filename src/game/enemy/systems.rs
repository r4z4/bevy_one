use bevy::{
    asset::AssetServer,
    audio::AudioBundle,
    ecs::{
        entity::Entity,
        query::With,
        system::{Commands, Query, Res, ResMut},
    },
    math::{Vec2, Vec3},
    sprite::SpriteBundle,
    time::Time,
    transform::components::Transform,
    utils::default,
    window::{PrimaryWindow, Window},
};
use rand::random;

// use crate::enemy::components::*
use super::components::*;
use super::resources::*;

pub const ENEMY_SPEED: f32 = 200.0;
pub const ENEMY_SIZE: f32 = 64.0; // Enemy sprite size
pub const NUM_ENEMIES: usize = 4;

pub fn spawn_enemies(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
) {
    // We know there will be only, and know it will exist, so can safely unwrap
    let window: &Window = window_query.get_single().unwrap();

    // For loop that runs over num enemies constant to spawn them
    for _ in 0..NUM_ENEMIES {
        // Random positions w/in window
        let rand_x: f32 = random::<f32>() * window.width();
        let rand_y: f32 = random::<f32>() * window.height();

        commands.spawn((
            SpriteBundle {
                transform: Transform::from_xyz(rand_x, rand_y, 0.0),
                texture: asset_server.load("sprites/ball_red_large.png"),
                ..default()
            },
            Enemy {
                direction: Vec2::new(random::<f32>(), random::<f32>()).normalize(),
            },
        ));
    }
}

pub fn despawn_enemies(mut commands: Commands, enemy_query: Query<Entity, With<Enemy>>) {
    for enemy_entity in enemy_query.iter() {
        commands.entity(enemy_entity).despawn();
    }
}

pub fn enemy_movement(mut enemy_query: Query<(&mut Transform, &Enemy)>, time: Res<Time>) {
    for (mut transform, enemy) in enemy_query.iter_mut() {
        let direction: Vec3 = Vec3::new(enemy.direction.x, enemy.direction.y, 0.0);
        transform.translation += direction * ENEMY_SPEED * time.delta_seconds();
    }
}

pub fn update_enemy_direction(
    mut enemy_query: Query<(&Transform, &mut Enemy)>,
    window_query: Query<&Window, With<PrimaryWindow>>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    let window = window_query.get_single().unwrap();

    let half_enemy_size: f32 = ENEMY_SIZE / 2.0; // 32.0
    let x_min: f32 = 0.0 + half_enemy_size;
    let x_max: f32 = window.width() - half_enemy_size;
    let y_min: f32 = 0.0 + half_enemy_size;
    let y_max: f32 = window.height() - half_enemy_size;

    for (transform, mut enemy) in enemy_query.iter_mut() {
        let mut direction_changed: bool = false;

        // Get translation and check if it is outside the bounds we wrote
        let translation: Vec3 = transform.translation;
        // Flip dir by * -1
        if translation.x <= x_min || translation.x >= x_max {
            enemy.direction.x *= -1.0;
            direction_changed = true;
        }
        if translation.y <= y_min || translation.y >= y_max {
            enemy.direction.y *= -1.0;
            direction_changed = true;
        }

        // Play SFX
        if direction_changed {
            let sound_effect_1 = asset_server.load("audio/pluck_001.ogg");
            let sound_effect_2 = asset_server.load("audio/pluck_002.ogg");
            // Randomly play one
            let sound_effect = if random::<f32>() > 0.5 {
                sound_effect_1
            } else {
                sound_effect_2
            };
            commands.spawn(AudioBundle {
                source: sound_effect,
                ..default()
            });
        }
    }
}

pub fn confine_enemy_movement(
    mut enemy_query: Query<&mut Transform, With<Enemy>>,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    let window = window_query.get_single().unwrap();

    let half_enemy_size: f32 = ENEMY_SIZE / 2.0; // 32.0
    let x_min: f32 = 0.0 + half_enemy_size;
    let x_max: f32 = window.width() - half_enemy_size;
    let y_min: f32 = 0.0 + half_enemy_size;
    let y_max: f32 = window.height() - half_enemy_size;

    for mut transform in enemy_query.iter_mut() {
        let mut translation: Vec3 = transform.translation;

        // Bound the enemy X position
        if translation.x < x_min {
            translation.x = x_min;
        } else if translation.x > x_max {
            translation.x = x_max;
        }

        // Bound the enemy Y position
        if translation.y < y_min {
            translation.y = y_min;
        } else if translation.y > y_max {
            translation.y = y_max;
        }

        transform.translation = translation;
    }
}

pub fn tick_enemy_spawn_timer(mut enemy_spawn_timer: ResMut<EnemySpawnTimer>, time: Res<Time>) {
    // Tick method takes in duration value so use time.delta()
    enemy_spawn_timer.timer.tick(time.delta());
}

pub fn spawn_enemies_over_time(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
    enemy_spawn_timer: Res<EnemySpawnTimer>,
) {
    // Means timer has hit 0
    // We set to repeating, so when it hits 0 it restarts
    if enemy_spawn_timer.timer.finished() {
        let window = window_query.get_single().unwrap();
        let rand_x: f32 = random::<f32>() * window.width();
        let rand_y: f32 = random::<f32>() * window.height();
        commands.spawn((
            SpriteBundle {
                transform: Transform::from_xyz(rand_x, rand_y, 0.0),
                texture: asset_server.load("sprites/ball_red_large.png"),
                ..default()
            },
            Enemy {
                direction: Vec2::new(random::<f32>(), random::<f32>()).normalize(),
            },
        ));
    }
}
