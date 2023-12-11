use bevy::ecs::{component::Component, event::Event, entity::Entity};

#[derive(Event, Clone)]
pub enum OnClickEvents {
    HexEvent(Entity)
}

#[derive(Component)]
pub struct Clickable {
    event: OnClickEvents,
}

impl Clickable {

    pub fn new(event: OnClickEvents) -> Self {
        Self { event: event }
    }

    pub fn get_event(&self) -> OnClickEvents {
        self.event.clone()
    }
}