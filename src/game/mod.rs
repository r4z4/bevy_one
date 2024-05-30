use bevy::prelude::*;

pub mod enemy;
pub mod player;
pub mod score;
pub mod star;
mod systems;

use enemy::EnemyPlugin;
use player::PlayerPlugin;
use score::ScorePlugin;
use star::StarPlugin;
use systems::*;

use crate::{AppState, GameOver};

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<SimulationState>()
            .add_event::<GameOver>()
            .add_plugins((EnemyPlugin, PlayerPlugin, ScorePlugin, StarPlugin))
            .add_systems(OnEnter(AppState::Game), pause_simulation)
            .add_systems(Update, toggle_simulation.run_if(in_state(AppState::Game)))
            .add_systems(OnExit(AppState::Game), pause_simulation);
    }
}

#[derive(States, Clone, Eq, Hash, Debug, PartialEq, Default)]
pub enum SimulationState {
    #[default]
    Running,
    Paused,
}
