use bevy::{ecs::system::Resource, math::Vec2};

#[derive(Resource, Default)]
pub struct MouseWorldCoords(pub Vec2);

#[derive(Resource)]
pub struct GameEntitiesClickable(pub bool);

impl Default for GameEntitiesClickable {
    fn default() -> Self {
        GameEntitiesClickable(true)
    }
}