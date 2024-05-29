use bevy::prelude::*;

pub mod components;
pub mod systems;

use systems::*;

use crate::AppState;

use super::SimulationState;

#[derive(SystemSet, Debug, Hash, PartialEq, Eq, Clone)]
pub struct MovementSystemSet;
#[derive(SystemSet, Debug, Hash, PartialEq, Eq, Clone)]
pub struct ConfinementSystemSet;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.configure_sets(Update, ConfinementSystemSet.after(MovementSystemSet))
            .add_systems(OnEnter(AppState::Game), spawn_player)
            .add_systems(
                Update,
                (player_movement)
                    .in_set(MovementSystemSet)
                    .run_if(in_state(AppState::Game))
                    .run_if(in_state(SimulationState::Running)),
            )
            .add_systems(
                Update,
                (confine_player_movement)
                    .in_set(ConfinementSystemSet)
                    .run_if(in_state(AppState::Game))
                    .run_if(in_state(SimulationState::Running)),
            )
            .add_systems(
                Update,
                (enemy_hit_player, player_hit_star)
                    .run_if(in_state(AppState::Game))
                    .run_if(in_state(SimulationState::Running)),
            )
            // Exit state systems
            .add_systems(OnExit(AppState::Game), despawn_player);
    }
}
