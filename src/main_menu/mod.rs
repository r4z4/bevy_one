use bevy::prelude::*;

pub struct MainMenuPlugin;
pub mod components;
pub mod styles;
mod systems;
use systems::layout::*;

use crate::AppState;

impl Plugin for MainMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::MainMenu), spawn_main_menu)
            .add_systems(OnExit(AppState::MainMenu), despawn_main_menu);
    }
}
