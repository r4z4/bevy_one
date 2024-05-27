use bevy::{
    input::keyboard::KeyboardInput, input::mouse::MouseButtonInput, prelude::*,
    window::PrimaryWindow,
};
use rand::prelude::*;

pub const PLAYER_SIZE: f32 = 64.0; // Player sprite size
pub const PLAYER_SPEED: f32 = 500.0;
pub const NUM_ENEMIES: usize = 4;
pub const ENEMY_SPEED: f32 = 200.0;
pub const ENEMY_SIZE: f32 = 64.0; // Enemy sprite size
pub const NUM_STARS: usize = 10;
pub const STAR_SIZE: f32 = 30.0; // 30x30 pixels
pub const STAR_SPAWN_TIME: f32 = 1.0;

fn main() {
    App::new()
        // .add_plugins(MinimalPlugins.set(ScheduleRunnerPlugin::run_once()))
        .add_plugins(DefaultPlugins)
        .init_resource::<Score>()
        .init_resource::<StarSpawnTimer>()
        .add_systems(Startup, spawn_camera)
        .add_systems(Startup, spawn_player)
        .add_systems(Startup, spawn_enemies)
        .add_systems(Startup, spawn_stars)
        .add_systems(
            Update,
            (
                player_movement,
                confine_player_movement,
                enemy_movement,
                update_enemy_direction,
                confine_enemy_movement,
                enemy_hit_player,
                player_hit_star,
                update_score,
                tick_star_spawn_timer,
                spawn_stars_over_time,
                mouse_button_events,
            ),
        )
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

#[derive(Component, Debug)]
pub struct Player {}

pub fn fetch_players(query: Query<&Player>) {
    for player in &query {
        info!("Player: {:?}", player);
    }
}

// Test
#[derive(Component, Debug)]
pub struct Enemy {
    pub direction: Vec2,
}

#[derive(Component, Debug)]
pub struct Star {}

#[derive(Resource)]
pub struct Score {
    pub value: u32,
}

impl Default for Score {
    fn default() -> Score {
        Score { value: 0 }
    }
}

#[derive(Resource)]
pub struct StarSpawnTimer {
    pub timer: Timer,
}

impl Default for StarSpawnTimer {
    fn default() -> StarSpawnTimer {
        StarSpawnTimer {
            timer: Timer::from_seconds(STAR_SPAWN_TIME, TimerMode::Repeating),
        }
    }
}

pub fn spawn_player(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
) {
    let window = window_query.get_single().unwrap();

    let _spawn = commands.spawn((
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

pub fn spawn_enemies(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
) {
    // We know there will be only, and know it will exist, so can safely unwrap
    let window: &Window = window_query.get_single().unwrap();

    // For loop that runs over num enemies constant to spawn them
    for _ in 0..NUM_ENEMIES {
        // Random positions w/in window
        let rand_x: f32 = random::<f32>() * window.width();
        let rand_y: f32 = random::<f32>() * window.height();

        commands.spawn((
            SpriteBundle {
                transform: Transform::from_xyz(rand_x, rand_y, 0.0),
                texture: asset_server.load("sprites/ball_red_large.png"),
                ..default()
            },
            Enemy {
                direction: Vec2::new(random::<f32>(), random::<f32>()).normalize(),
            },
        ));
    }
}

pub fn spawn_stars(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
) {
    let window: &Window = window_query.get_single().unwrap();

    // For loop that runs over num enemies constant to spawn them
    for _ in 0..NUM_STARS {
        // Random positions w/in window
        let rand_x: f32 = random::<f32>() * window.width();
        let rand_y: f32 = random::<f32>() * window.height();

        commands.spawn((
            SpriteBundle {
                transform: Transform::from_xyz(rand_x, rand_y, 0.0),
                texture: asset_server.load("sprites/star.png"),
                ..default()
            },
            Star {},
        ));
    }
}

/// This system prints out all keyboard events as they come in
pub fn print_keyboard_event_system(mut keyboard_input_events: EventReader<KeyboardInput>) {
    for event in keyboard_input_events.read() {
        info!("{:?}", event);
    }
}

pub fn player_movement(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut player_query: Query<&mut Transform, With<Player>>,
    time: Res<Time>,
) {
    if let Ok(mut transform) = player_query.get_single_mut() {
        let mut direction = Vec3::ZERO;

        if keyboard_input.pressed(KeyCode::ArrowLeft) || keyboard_input.pressed(KeyCode::KeyA) {
            eprintln!("left pressed");
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

fn mouse_button_events(mut mousebtn_evr: EventReader<MouseButtonInput>) {
    use bevy::input::ButtonState;

    for ev in mousebtn_evr.read() {
        match ev.state {
            ButtonState::Pressed => {
                println!("Mouse button press: {:?}", ev.button);
            }
            ButtonState::Released => {
                println!("Mouse button release: {:?}", ev.button);
            }
        }
    }
}

pub fn confine_player_movement(
    mut player_query: Query<&mut Transform, With<Player>>,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    // Other places we are just doing unwrap() and assuming it is safe??
    if let Ok(mut player_transform) = player_query.get_single_mut() {
        let window = window_query.get_single().unwrap();

        let half_player_size: f32 = PLAYER_SIZE / 2.0; // 32.0
        let x_min: f32 = 0.0 + half_player_size;
        let x_max: f32 = window.width() - half_player_size;
        let y_min: f32 = 0.0 + half_player_size;
        let y_max: f32 = window.height() - half_player_size;

        let mut translation: Vec3 = player_transform.translation;

        // Bound the player X position
        if translation.x < x_min {
            translation.x = x_min;
        } else if translation.x > x_max {
            translation.x = x_max;
        }

        // Bound the player Y position
        if translation.y < y_min {
            translation.y = y_min;
        } else if translation.y > y_max {
            translation.y = y_max;
        }

        player_transform.translation = translation;
    }
}

pub fn enemy_movement(mut enemy_query: Query<(&mut Transform, &Enemy)>, time: Res<Time>) {
    for (mut transform, enemy) in enemy_query.iter_mut() {
        let direction: Vec3 = Vec3::new(enemy.direction.x, enemy.direction.y, 0.0);
        transform.translation += direction * ENEMY_SPEED * time.delta_seconds();
    }
}

pub fn update_enemy_direction(
    mut enemy_query: Query<(&Transform, &mut Enemy)>,
    window_query: Query<&Window, With<PrimaryWindow>>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    let window = window_query.get_single().unwrap();

    let half_enemy_size: f32 = ENEMY_SIZE / 2.0; // 32.0
    let x_min: f32 = 0.0 + half_enemy_size;
    let x_max: f32 = window.width() - half_enemy_size;
    let y_min: f32 = 0.0 + half_enemy_size;
    let y_max: f32 = window.height() - half_enemy_size;

    for (transform, mut enemy) in enemy_query.iter_mut() {
        let mut direction_changed: bool = false;

        // Get translation and check if it is outside the bounds we wrote
        let translation: Vec3 = transform.translation;
        // Flip dir by * -1
        if translation.x <= x_min || translation.x >= x_max {
            enemy.direction.x *= -1.0;
            direction_changed = true;
        }
        if translation.y <= y_min || translation.y >= y_max {
            enemy.direction.y *= -1.0;
            direction_changed = true;
        }

        // Play SFX
        if direction_changed {
            let sound_effect_1 = asset_server.load("audio/pluck_001.ogg");
            let sound_effect_2 = asset_server.load("audio/pluck_002.ogg");
            // Randomly play one
            let sound_effect = if random::<f32>() > 0.5 {
                sound_effect_1
            } else {
                sound_effect_2
            };
            commands.spawn(AudioBundle {
                source: sound_effect,
                ..default()
            });
        }
    }
}

pub fn confine_enemy_movement(
    mut enemy_query: Query<&mut Transform, With<Enemy>>,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    let window = window_query.get_single().unwrap();

    let half_enemy_size: f32 = ENEMY_SIZE / 2.0; // 32.0
    let x_min: f32 = 0.0 + half_enemy_size;
    let x_max: f32 = window.width() - half_enemy_size;
    let y_min: f32 = 0.0 + half_enemy_size;
    let y_max: f32 = window.height() - half_enemy_size;

    for mut transform in enemy_query.iter_mut() {
        let mut translation: Vec3 = transform.translation;

        // Bound the enemy X position
        if translation.x < x_min {
            translation.x = x_min;
        } else if translation.x > x_max {
            translation.x = x_max;
        }

        // Bound the enemy Y position
        if translation.y < y_min {
            translation.y = y_min;
        } else if translation.y > y_max {
            translation.y = y_max;
        }

        transform.translation = translation;
    }
}

pub fn enemy_hit_player(
    // Entity is just a u32 so we can just copy it around. Do not need a reference.
    mut commands: Commands,
    mut player_query: Query<(Entity, &Transform), With<Player>>,
    enemy_query: Query<&Transform, With<Enemy>>,
    asset_server: Res<AssetServer>,
) {
    if let Ok((player_entity, player_transform)) = player_query.get_single_mut() {
        // iterate on enemy query to look at each enemy transform
        // just looking at transform, not a mutable query
        for enemy_transform in enemy_query.iter() {
            // get distance between the player transform and enemy transform
            let distance = player_transform
                .translation
                .distance(enemy_transform.translation);
            // Determine if enemy and player are touching, so need two local vars
            let player_radius: f32 = PLAYER_SIZE / 2.0;
            let enemy_radius: f32 = ENEMY_SIZE / 2.0;
            // If distance less than the sum or these two, they are touching
            if distance < player_radius + enemy_radius {
                println!("Enemy hit player. Game Over!");
                let sound_effect = asset_server.load("audio/explosionCrunch_000.ogg");
                commands.spawn(AudioBundle {
                    source: sound_effect,
                    ..default()
                });
                commands.entity(player_entity).despawn();
            }
        }
    }
}

pub fn player_hit_star(
    // Entity is just a u32 so we can just copy it around. Do not need a reference.
    mut commands: Commands,
    player_query: Query<&Transform, With<Player>>,
    star_query: Query<(Entity, &Transform), With<Star>>,
    asset_server: Res<AssetServer>,
    mut score: ResMut<Score>,
) {
    if let Ok(player_transform) = player_query.get_single() {
        // iterate on enemy query to look at each enemy transform
        // just looking at transform, not a mutable query
        for (star_entity, star_transform) in star_query.iter() {
            // get distance between the player transform and star transform
            let distance = player_transform
                .translation
                .distance(star_transform.translation);
            // Determine if enemy and player are touching, so need two local vars
            let player_radius: f32 = PLAYER_SIZE / 2.0;
            let star_radius: f32 = STAR_SIZE / 2.0;
            // If distance less than the sum or these two, they are touching
            if distance < player_radius + star_radius {
                println!("Player hit star. Earned points!");
                score.value += 1;
                let sound_effect = asset_server.load("audio/laserLarge_000.ogg");
                commands.spawn(AudioBundle {
                    source: sound_effect,
                    ..default()
                });
                commands.entity(star_entity).despawn();
            }
        }
    }
}

pub fn update_score(score: Res<Score>) {
    if score.is_changed() {
        println!("Score: {}", score.value.to_string());
    }
}

pub fn tick_star_spawn_timer(mut star_spawn_timer: ResMut<StarSpawnTimer>, time: Res<Time>) {
    // Tick method takes in duration value so use time.delta()
    star_spawn_timer.timer.tick(time.delta());
}

pub fn spawn_stars_over_time(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
    star_spawn_timer: Res<StarSpawnTimer>,
) {
    // Means timer has hit 0
    // We set to repeating, so when it hits 0 it restarts
    if star_spawn_timer.timer.finished() {
        let window = window_query.get_single().unwrap();
        let rand_x: f32 = random::<f32>() * window.width();
        let rand_y: f32 = random::<f32>() * window.height();
        commands.spawn((
            SpriteBundle {
                transform: Transform::from_xyz(rand_x, rand_y, 0.0),
                texture: asset_server.load("sprites/star.png"),
                ..default()
            },
            Star {},
        ));
    }
}

//
//
// Hello World e.g.

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
