use bevy::{ecs::component::Component, math::Vec2};

#[derive(Component, Debug)]
pub struct Enemy {
    pub direction: Vec2,
}
