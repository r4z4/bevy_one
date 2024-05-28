use bevy::{
    app::AppExit,
    ecs::{
        event::{Event, EventReader, EventWriter},
        system::Res,
    },
    input::{keyboard::KeyCode, ButtonInput},
};

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

pub fn handle_game_over(mut game_over_event_reader: EventReader<GameOver>) {
    for event in game_over_event_reader.read() {
        println!("Your final score is {}", event.score.to_string());
    }
}
