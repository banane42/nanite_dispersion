use bevy::ecs::component::Component;

#[derive(Component, Clone)]
pub enum ClickSignal {
    Hex,
    Macc
}