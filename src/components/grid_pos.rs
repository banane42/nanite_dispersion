use std::fmt::Display;
use bevy::ecs::component::Component;

#[derive(Component, Debug)]
pub struct GridPos {
    pub pos: (usize, usize)
}

impl GridPos {
    pub fn row_is_even(&self) -> bool {
        self.pos.0 % 2 == 0
    }

    pub fn to_int(&self) -> (i32, i32) {
        (self.pos.0 as i32, self.pos.1 as i32)
    }
}

impl Display for GridPos {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.pos.0, self.pos.1)
    }
}