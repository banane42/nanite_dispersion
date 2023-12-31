use bevy::{ecs::{system::Resource, entity::Entity}, math::Vec2};

#[derive(Resource, Default)]
pub struct MouseWorldCoords(pub Vec2);

#[derive(Resource)]
pub struct GameEntitiesClickable(pub bool);

impl Default for GameEntitiesClickable {
    fn default() -> Self {
        GameEntitiesClickable(true)
    }
}

#[derive(Resource)]
pub struct SelectedMacc {
    macc: Option<Entity>
}

impl Default for SelectedMacc {
    fn default() -> Self {
        Self { macc: None }
    }
}

impl SelectedMacc {
    pub fn select(&mut self, ent: Entity) {
        self.macc = Some(ent);
    }

    pub fn deselect(&mut self) {
        self.macc = None;
    }

    pub fn get(&self) -> Option<Entity> {
        self.macc
    }
}