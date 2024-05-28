use bevy::{prelude::*, window::PrimaryWindow};
use enemy::{
    resources::EnemySpawnTimer,
    systems::{
        confine_enemy_movement, enemy_movement, spawn_enemies, spawn_enemies_over_time,
        tick_enemy_spawn_timer, update_enemy_direction,
    },
};
use player::systems::{
    confine_player_movement, enemy_hit_player, player_hit_star, player_movement, spawn_player,
};
use score::{
    resources::{HighScores, Score},
    systems::{high_scores_updated, update_high_scores, update_score},
};
use star::{
    resources::StarSpawnTimer,
    systems::{spawn_stars, spawn_stars_over_time, tick_star_spawn_timer},
};

pub mod enemy;
pub mod events;
pub mod player;
pub mod score;
pub mod star;

use crate::events::*;

fn main() {
    App::new()
        // .add_plugins(MinimalPlugins.set(ScheduleRunnerPlugin::run_once()))
        .add_plugins(DefaultPlugins)
        .init_resource::<Score>()
        .init_resource::<HighScores>()
        .init_resource::<StarSpawnTimer>()
        .init_resource::<EnemySpawnTimer>()
        .add_event::<GameOver>()
        .add_systems(Startup, spawn_camera)
        .add_systems(Startup, spawn_player)
        .add_systems(Startup, spawn_enemies)
        .add_systems(Startup, spawn_stars)
        .add_systems(
            Update,
            (
                player_movement,
                confine_player_movement,
                enemy_movement,
                update_enemy_direction,
                confine_enemy_movement,
                enemy_hit_player,
                player_hit_star,
                update_score,
                tick_star_spawn_timer,
                tick_enemy_spawn_timer,
                spawn_stars_over_time,
                spawn_enemies_over_time,
                update_high_scores,
                high_scores_updated,
                handle_game_over,
                exit_game,
            ),
        )
        .run()
}

pub fn spawn_camera(mut commands: Commands, window_query: Query<&Window, With<PrimaryWindow>>) {
    let window = window_query.get_single().unwrap();
    commands.spawn(Camera2dBundle {
        transform: Transform::from_xyz(window.width() / 2.0, window.height() / 2.0, 0.0),
        ..default()
    });
}
