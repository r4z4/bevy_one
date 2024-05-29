use bevy::{
    asset::AssetServer,
    audio::AudioBundle,
    ecs::{
        entity::Entity,
        event::EventWriter,
        query::With,
        system::{Commands, Query, Res, ResMut},
    },
    input::{keyboard::KeyCode, ButtonInput},
    math::Vec3,
    sprite::SpriteBundle,
    time::Time,
    transform::components::Transform,
    utils::default,
    window::{PrimaryWindow, Window},
};

use super::components::*;
use crate::game::{
    enemy::{components::Enemy, systems::ENEMY_SIZE},
    score::resources::Score,
    star::{components::Star, systems::STAR_SIZE},
    GameOver,
};

pub const PLAYER_SIZE: f32 = 64.0; // Player sprite size
pub const PLAYER_SPEED: f32 = 500.0;

pub fn spawn_player(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
) {
    let window = window_query.get_single().unwrap();

    let _spawn = commands.spawn((
        SpriteBundle {
            // Centers it in the window
            transform: Transform::from_xyz(window.width() / 2.0, window.height() / 2.0, 0.0),
            // Set texture we want to use
            texture: asset_server.load("sprites/ball_blue_large.png"),
            ..default()
        },
        Player {},
    ));
}

pub fn despawn_player(mut commands: Commands, player_query: Query<Entity, With<Player>>) {
    if let Ok(player_entity) = player_query.get_single() {
        commands.entity(player_entity).despawn();
    }
}

pub fn player_movement(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut player_query: Query<&mut Transform, With<Player>>,
    time: Res<Time>,
) {
    if let Ok(mut transform) = player_query.get_single_mut() {
        let mut direction = Vec3::ZERO;

        if keyboard_input.pressed(KeyCode::ArrowLeft) || keyboard_input.pressed(KeyCode::KeyA) {
            eprintln!("left pressed");
            direction += Vec3::new(-1.0, 0.0, 0.0);
        }
        if keyboard_input.pressed(KeyCode::ArrowRight) || keyboard_input.pressed(KeyCode::KeyD) {
            direction += Vec3::new(1.0, 0.0, 0.0);
        }
        if keyboard_input.pressed(KeyCode::ArrowUp) || keyboard_input.pressed(KeyCode::KeyW) {
            direction += Vec3::new(0.0, 1.0, 0.0);
        }
        if keyboard_input.pressed(KeyCode::ArrowDown) || keyboard_input.pressed(KeyCode::KeyS) {
            direction += Vec3::new(0.0, -1.0, 0.0);
        }

        if direction.length() > 0.0 {
            direction = direction.normalize();
        }

        transform.translation += direction * PLAYER_SPEED * time.delta_seconds();
    }
}

pub fn confine_player_movement(
    mut player_query: Query<&mut Transform, With<Player>>,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    // Other places we are just doing unwrap() and assuming it is safe??
    if let Ok(mut player_transform) = player_query.get_single_mut() {
        let window = window_query.get_single().unwrap();

        let half_player_size: f32 = PLAYER_SIZE / 2.0; // 32.0
        let x_min: f32 = 0.0 + half_player_size;
        let x_max: f32 = window.width() - half_player_size;
        let y_min: f32 = 0.0 + half_player_size;
        let y_max: f32 = window.height() - half_player_size;

        let mut translation: Vec3 = player_transform.translation;

        // Bound the player X position
        if translation.x < x_min {
            translation.x = x_min;
        } else if translation.x > x_max {
            translation.x = x_max;
        }

        // Bound the player Y position
        if translation.y < y_min {
            translation.y = y_min;
        } else if translation.y > y_max {
            translation.y = y_max;
        }

        player_transform.translation = translation;
    }
}

pub fn enemy_hit_player(
    // Entity is just a u32 so we can just copy it around. Do not need a reference.
    mut commands: Commands,
    mut game_over_event_writer: EventWriter<GameOver>,
    mut player_query: Query<(Entity, &Transform), With<Player>>,
    enemy_query: Query<&Transform, With<Enemy>>,
    asset_server: Res<AssetServer>,
    score: Res<Score>,
) {
    if let Ok((player_entity, player_transform)) = player_query.get_single_mut() {
        // iterate on enemy query to look at each enemy transform
        // just looking at transform, not a mutable query
        for enemy_transform in enemy_query.iter() {
            // get distance between the player transform and enemy transform
            let distance = player_transform
                .translation
                .distance(enemy_transform.translation);
            // Determine if enemy and player are touching, so need two local vars
            let player_radius: f32 = PLAYER_SIZE / 2.0;
            let enemy_radius: f32 = ENEMY_SIZE / 2.0;
            // If distance less than the sum or these two, they are touching
            if distance < player_radius + enemy_radius {
                println!("Enemy hit player. Game Over!");
                let sound_effect = asset_server.load("audio/explosionCrunch_000.ogg");
                commands.spawn(AudioBundle {
                    source: sound_effect,
                    ..default()
                });
                commands.entity(player_entity).despawn();
                game_over_event_writer.send(GameOver { score: score.value });
            }
        }
    }
}

pub fn player_hit_star(
    // Entity is just a u32 so we can just copy it around. Do not need a reference.
    mut commands: Commands,
    player_query: Query<&Transform, With<Player>>,
    star_query: Query<(Entity, &Transform), With<Star>>,
    asset_server: Res<AssetServer>,
    mut score: ResMut<Score>,
) {
    if let Ok(player_transform) = player_query.get_single() {
        // iterate on enemy query to look at each enemy transform
        // just looking at transform, not a mutable query
        for (star_entity, star_transform) in star_query.iter() {
            // get distance between the player transform and star transform
            let distance = player_transform
                .translation
                .distance(star_transform.translation);
            // Determine if enemy and player are touching, so need two local vars
            let player_radius: f32 = PLAYER_SIZE / 2.0;
            let star_radius: f32 = STAR_SIZE / 2.0;
            // If distance less than the sum or these two, they are touching
            if distance < player_radius + star_radius {
                println!("Player hit star. Earned points!");
                score.value += 1;
                let sound_effect = asset_server.load("audio/laserLarge_000.ogg");
                commands.spawn(AudioBundle {
                    source: sound_effect,
                    ..default()
                });
                commands.entity(star_entity).despawn();
            }
        }
    }
}
