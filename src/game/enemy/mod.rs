use bevy::prelude::*;

pub mod components;
pub mod resources;
pub mod systems;

use resources::*;
use systems::*;

use crate::AppState;

use super::SimulationState;

pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<EnemySpawnTimer>()
            .add_systems(OnEnter(AppState::Game), spawn_enemies)
            .add_systems(
                Update,
                (
                    enemy_movement,
                    update_enemy_direction,
                    confine_enemy_movement,
                    tick_enemy_spawn_timer,
                    spawn_enemies_over_time,
                )
                    .run_if(in_state(AppState::Game))
                    .run_if(in_state(SimulationState::Running)),
            )
            // Exit state systems
            .add_systems(OnExit(AppState::Game), despawn_enemies);
    }
}
