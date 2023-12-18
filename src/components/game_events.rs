use bevy::{ecs::{event::Event, entity::Entity}, math::Vec2};

#[derive(Event)]
pub enum GameEvents {
    HexSelect(Entity),
    MaccSelect(Entity),
    MaccMoveOrder(Vec2)
}