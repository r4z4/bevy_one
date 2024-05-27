use bevy::{prelude::*, window::PrimaryWindow, input::keyboard::KeyboardInput};

fn main() {
    App::new()
        // .add_plugins(MinimalPlugins.set(ScheduleRunnerPlugin::run_once()))
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, spawn_camera)
        .add_systems(Startup, spawn_player)
        .add_systems(Startup, player_movement)
        .run()
}

pub struct PeoplePlugin;

impl Plugin for PeoplePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup)
            .add_systems(Update, hello_world)
            .add_systems(Update, print_names)
            .add_systems(Update, people_with_jobs)
            .add_systems(Update, people_ready_for_hire)
            .add_systems(Update, person_does_job)
            .add_systems(Update, print_keyboard_event_system);
    }
}

#[derive(Component)]
pub struct Player {}

pub fn spawn_player(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
) {
    let window = window_query.get_single().unwrap();

    let spawn = commands.spawn((
        SpriteBundle {
            // Centers it in the window
            transform: Transform::from_xyz(window.width() / 2.0, window.height() / 2.0, 0.0),
            // Set texture we want to use
            texture: asset_server.load("sprites/ball_blue_large.png"),
            ..default()
        },
        Player {},
    ));
}

pub fn spawn_camera(mut commands: Commands, window_query: Query<&Window, With<PrimaryWindow>>) {
    let window = window_query.get_single().unwrap();
    commands.spawn(Camera2dBundle {
        transform: Transform::from_xyz(window.width() / 2.0, window.height() / 2.0, 0.0),
        ..default()
    });
}
/// This system prints out all keyboard events as they come in
pub fn print_keyboard_event_system(mut keyboard_input_events: EventReader<KeyboardInput>) {
    for event in keyboard_input_events.read() {
        info!("{:?}", event);
    }
}
pub const PLAYER_SPEED: f32 = 500.0;

pub fn player_movement(keyboard_input: Res<ButtonInput<KeyCode>>, mut player_query: Query<&mut Transform, With<Player>>, time: Res<Time>,) {
    if let Ok(mut transform) = player_query.get_single_mut() {
        let mut direction = Vec3::ZERO;

        if keyboard_input.pressed(KeyCode::ArrowLeft) || keyboard_input.pressed(KeyCode::KeyA) {
            direction += Vec3::new(-1.0, 0.0, 0.0);
        }
        if keyboard_input.pressed(KeyCode::ArrowRight) || keyboard_input.pressed(KeyCode::KeyD) {
            direction += Vec3::new(1.0, 0.0, 0.0);
        }
        if keyboard_input.pressed(KeyCode::ArrowUp) || keyboard_input.pressed(KeyCode::KeyW) {
            direction += Vec3::new(0.0, 1.0, 0.0);
        }
        if keyboard_input.pressed(KeyCode::ArrowDown) || keyboard_input.pressed(KeyCode::KeyS) {
            direction += Vec3::new(0.0, -1.0, 0.0);
        }

        if direction.length() > 0.0 {
            direction = direction.normalize();
        }

        transform.translation += direction * PLAYER_SPEED * time.delta_seconds();
    }
}

pub fn hello_world() {
    println!("Hello World");
}

pub fn setup(mut commands: Commands) {
    commands.spawn((
        Person {
            name: "Erik".to_string(),
        },
        Employed {
            job: Job::Programmer,
        },
    ));
    commands.spawn((
        Person {
            name: "Steve".to_string(),
        },
        Employed { job: Job::Doctor },
    ));
    commands.spawn((
        Person {
            name: "Laura".to_string(),
        },
        Employed { job: Job::Lawyer },
    ));
    commands.spawn(Person {
        name: "Jobless Joe".to_string(),
    });
    commands.spawn((
        Person {
            name: "Matt".to_string(),
        },
        Employed {
            job: Job::Accountant,
        },
    ));
}

#[derive(Component)]
pub struct Person {
    pub name: String,
}

pub fn print_names(person_query: Query<&Person>) {
    for person in person_query.iter() {
        println!("Name: {}", person.name);
    }
}

pub fn people_with_jobs(person_query: Query<&Person, With<Employed>>) {
    for person in person_query.iter() {
        println!("{} has a job.", person.name);
    }
}

pub fn people_ready_for_hire(person_query: Query<&Person, Without<Employed>>) {
    for person in person_query.iter() {
        println!("{} is ready for hire.", person.name)
    }
}

pub fn person_does_job(person_query: Query<(&Person, &Employed)>) {
    for (person, employed) in person_query.iter() {
        let job_name = match employed.job {
            Job::Doctor => "Doctor",
            Job::Programmer => "Programmer",
            Job::Lawyer => "Lawyer",
            Job::Accountant => "Accountant",
        };
        println!("{0} is a {1}", person.name, job_name)
    }
}

#[derive(Component)]
pub struct Employed {
    pub job: Job,
}

#[derive(Debug)]
pub enum Job {
    Doctor,
    Accountant,
    Programmer,
    Lawyer,
}
