use bevy::prelude::*;

pub mod resources;
pub mod systems;

use systems::*;

use crate::AppState;

use self::resources::{HighScores, Score};

use super::SimulationState;

pub struct ScorePlugin;

impl Plugin for ScorePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<HighScores>()
            .add_systems(OnEnter(AppState::Game), insert_score)
            .add_systems(
                Update,
                (update_score, update_high_scores, high_scores_updated)
                    .run_if(in_state(AppState::Game))
                    .run_if(in_state(SimulationState::Running)),
            )
            .add_systems(OnExit(AppState::Game), remove_score);
    }
}
