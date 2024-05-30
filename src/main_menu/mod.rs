use bevy::prelude::*;

pub struct MainMenuPlugin;
pub mod components;
pub mod styles;
mod systems;
use systems::layout::*;

use crate::AppState;

use self::systems::interactions::{
    button_system, interact_with_play_button, interact_with_quit_button,
};

impl Plugin for MainMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::MainMenu), spawn_main_menu)
            .add_systems(
                Update,
                (interact_with_play_button, interact_with_quit_button),
            )
            .add_systems(OnExit(AppState::MainMenu), despawn_main_menu);
    }
}
