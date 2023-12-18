use bevy::{ecs::component::Component, math::Vec2};

#[derive(Component)]
pub enum Team {
    A, B
}

#[derive(Component, Debug)]
pub struct Macc {
    pub target_position: Vec2,
    pub turn_radius: f32 // Max angle to turn in degrees
}

impl Macc {
    pub fn in_position(&self, current_location: Vec2) -> bool {
        self.target_position.distance(current_location) <= 1.0
    }
}