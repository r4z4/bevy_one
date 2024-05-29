use bevy::prelude::*;

use super::SimulationState;

pub fn toggle_simulation(
    // mut commands: Commands,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    simulation_state: Res<State<SimulationState>>,
    mut next_state: ResMut<NextState<SimulationState>>,
) {
    if keyboard_input.pressed(KeyCode::Space) {
        match simulation_state.get() {
            SimulationState::Paused => next_state.set(SimulationState::Running),
            SimulationState::Running => next_state.set(SimulationState::Paused),
        }
    }
}
