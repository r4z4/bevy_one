use bevy::{
    asset::AssetServer,
    ecs::{
        query::With,
        system::{Commands, Query, Res, ResMut},
    },
    sprite::SpriteBundle,
    time::Time,
    transform::components::Transform,
    utils::default,
    window::{PrimaryWindow, Window},
};
use rand::random;

use super::{components::Star, resources::StarSpawnTimer};

pub const NUM_STARS: usize = 10;
pub const STAR_SIZE: f32 = 30.0; // 30x30 pixels

pub fn spawn_stars(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
) {
    let window: &Window = window_query.get_single().unwrap();

    // For loop that runs over num enemies constant to spawn them
    for _ in 0..NUM_STARS {
        // Random positions w/in window
        let rand_x: f32 = random::<f32>() * window.width();
        let rand_y: f32 = random::<f32>() * window.height();

        commands.spawn((
            SpriteBundle {
                transform: Transform::from_xyz(rand_x, rand_y, 0.0),
                texture: asset_server.load("sprites/star.png"),
                ..default()
            },
            Star {},
        ));
    }
}

pub fn tick_star_spawn_timer(mut star_spawn_timer: ResMut<StarSpawnTimer>, time: Res<Time>) {
    // Tick method takes in duration value so use time.delta()
    star_spawn_timer.timer.tick(time.delta());
}

pub fn spawn_stars_over_time(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
    star_spawn_timer: Res<StarSpawnTimer>,
) {
    // Means timer has hit 0
    // We set to repeating, so when it hits 0 it restarts
    if star_spawn_timer.timer.finished() {
        let window = window_query.get_single().unwrap();
        let rand_x: f32 = random::<f32>() * window.width();
        let rand_y: f32 = random::<f32>() * window.height();
        commands.spawn((
            SpriteBundle {
                transform: Transform::from_xyz(rand_x, rand_y, 0.0),
                texture: asset_server.load("sprites/star.png"),
                ..default()
            },
            Star {},
        ));
    }
}
