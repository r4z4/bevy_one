use bevy::{prelude::*, window::PrimaryWindow};
use game::GamePlugin;
use main_menu::MainMenuPlugin;
pub mod events;
pub mod game;
pub mod main_menu;

use crate::events::*;

fn main() {
    App::new()
        // .add_plugins(MinimalPlugins.set(ScheduleRunnerPlugin::run_once()))
        .add_plugins(DefaultPlugins)
        .init_state::<AppState>()
        .add_plugins((MainMenuPlugin, GamePlugin))
        .add_systems(Startup, spawn_camera)
        .add_systems(
            Update,
            (
                handle_game_over,
                exit_game,
                transition_to_game_state,
                transition_to_main_menu_state,
            ),
        )
        .run()
}

#[derive(States, Clone, Eq, Hash, Debug, PartialEq, Default)]
pub enum AppState {
    #[default]
    MainMenu,
    Game,
    GameOver,
}

pub fn spawn_camera(mut commands: Commands, window_query: Query<&Window, With<PrimaryWindow>>) {
    let window = window_query.get_single().unwrap();
    commands.spawn(Camera2dBundle {
        transform: Transform::from_xyz(window.width() / 2.0, window.height() / 2.0, 0.0),
        ..default()
    });
}
