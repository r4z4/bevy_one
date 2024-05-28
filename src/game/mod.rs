use bevy::prelude::*;

pub mod enemy;
pub mod player;
pub mod score;
pub mod star;

use enemy::EnemyPlugin;
use player::PlayerPlugin;
use score::ScorePlugin;
use star::StarPlugin;

use crate::GameOver;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<GameOver>().add_plugins((
            EnemyPlugin,
            PlayerPlugin,
            ScorePlugin,
            StarPlugin,
        ));
    }
}
