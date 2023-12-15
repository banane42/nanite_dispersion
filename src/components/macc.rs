use bevy::ecs::{component::Component, entity::Entity};

#[derive(Component)]
pub enum Team {
    A, B
}

#[derive(Component, Debug)]
pub struct Macc {

}

#[derive(Component)]
pub struct MaccInfo {
    maccs: Vec<Entity>
}