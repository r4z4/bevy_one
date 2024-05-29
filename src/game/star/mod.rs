use bevy::prelude::*;

pub mod components;
pub mod resources;
pub mod systems;

use resources::*;
use systems::*;

use crate::AppState;

use super::SimulationState;

pub struct StarPlugin;

impl Plugin for StarPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<StarSpawnTimer>()
            .add_systems(OnEnter(AppState::Game), spawn_stars)
            .add_systems(
                Update,
                (tick_star_spawn_timer, spawn_stars_over_time)
                    .run_if(in_state(AppState::Game))
                    .run_if(in_state(SimulationState::Running)),
            )
            // Exit state systems
            .add_systems(OnExit(AppState::Game), despawn_stars);
    }
}
