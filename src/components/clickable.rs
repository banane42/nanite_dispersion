use bevy::{ecs::{component::Component, event::Event, entity::Entity}, input::mouse::MouseButton};

#[derive(Event, Clone)]
pub struct OnClickEvent {
    pub signal: ClickSignal,
    pub mouse_button: MouseButton,
    pub entity: Entity
}

#[derive(Component, Clone)]
pub enum ClickSignal {
    Hex,
    Macc
}