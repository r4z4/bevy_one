use bevy::{
    app::AppExit,
    ecs::{
        event::{Event, EventReader, EventWriter},
        schedule::{NextState, State},
        system::{Commands, Res, ResMut},
    },
    input::{keyboard::KeyCode, ButtonInput},
};

use crate::{game::SimulationState, AppState};

#[derive(Event)]
pub struct GameOver {
    pub score: u32,
}

pub fn exit_game(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut app_exit_event_writer: EventWriter<AppExit>,
) {
    // Check if esc has been pressed
    if keyboard_input.pressed(KeyCode::Escape) {
        // If so, send AppExit event
        app_exit_event_writer.send(AppExit);
    }
}

pub fn handle_game_over(
    mut next_sim_state: ResMut<NextState<SimulationState>>,
    mut next_app_state: ResMut<NextState<AppState>>,
    mut game_over_event_reader: EventReader<GameOver>,
) {
    for event in game_over_event_reader.read() {
        println!("Your final score is {}", event.score.to_string());
        next_app_state.set(AppState::GameOver);
        next_sim_state.set(SimulationState::Paused);
    }
}

pub fn transition_to_game_state(
    // mut commands: Commands,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    app_state: Res<State<AppState>>,
    mut next_state: ResMut<NextState<AppState>>,
) {
    if keyboard_input.pressed(KeyCode::KeyG) {
        match app_state.get() {
            AppState::MainMenu => next_state.set(AppState::Game),
            AppState::Game => (),
            AppState::GameOver => next_state.set(AppState::Game),
        }
    }
}

pub fn transition_to_main_menu_state(
    // mut commands: Commands,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    app_state: Res<State<AppState>>,
    mut next_app_state: ResMut<NextState<AppState>>,
    mut next_sim_state: ResMut<NextState<SimulationState>>,
) {
    if keyboard_input.pressed(KeyCode::KeyM) {
        match app_state.get() {
            AppState::MainMenu => (),
            AppState::Game => {
                next_sim_state.set(SimulationState::Paused);
                next_app_state.set(AppState::MainMenu);
            }
            AppState::GameOver => {
                next_sim_state.set(SimulationState::Paused);
                next_app_state.set(AppState::MainMenu);
            }
        }
    }
}
