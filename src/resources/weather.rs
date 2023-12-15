use bevy::ecs::system::Resource;
use rand::{thread_rng, Rng};

use crate::resources::hex::HexDirection;

#[derive(Resource)]
pub struct Weather {
    pub wind_strength: f32,
    pub wind_direction: f32
}

impl Weather {
    pub fn adjust_wind(&mut self) {
        let mut rng = thread_rng();
        let mut dir = self.wind_direction + rng.gen_range(-90.0..90.0);
        if dir < 0.0 {
            dir += 360.0;
        } else if dir > 360.0 {
            dir -= 360.0;
        }
        println!("Adjusting wind to {}", dir);
        self.wind_direction = dir;
    }

    pub fn debug_wind_adj(&mut self, left: bool) {
        let mut dir = if left {
            self.wind_direction - 40.0
        } else {
            self.wind_direction + 40.0
        };
        if dir < 0.0 {
            dir += 360.0
        } else if dir > 360.0 {
            dir -= 360.0
        }
        println!("Adjusting wind to {:?}:  {}", HexDirection::from_angle(dir), dir);
        self.wind_direction = dir;
    }
}