use bevy::{ecs::{system::Resource, entity::Entity}, math::Vec2};
use rand::{thread_rng, Rng};

use crate::components::grid_pos::GridPos;

#[derive(Resource, Default)]
pub struct MouseWorldCoords(pub Vec2);

#[derive(PartialEq, Debug)]
pub enum HexDirection {
    TopLeft,
    TopRight,
    Right,
    BottomRight,
    BottomLeft,
    Left
}

impl HexDirection {
    pub fn from_angle(angle: f32) -> Self {
        match angle % 360.0 {
            ang if (0.0..30.0).contains(&ang) => HexDirection::Right,
            ang if (330.0..=360.0).contains(&ang) => HexDirection::Right,
            ang if (30.0..90.0).contains(&ang) => HexDirection::BottomRight,
            ang if (90.0..150.0).contains(&ang) => HexDirection::BottomLeft,
            ang if (150.0..210.0).contains(&ang) => HexDirection::Left,
            ang if (210.0..270.0).contains(&ang) => HexDirection::TopLeft,
            ang if (270.0..330.0).contains(&ang) => HexDirection::TopRight,
            _ => HexDirection::Right
        }
    }
}

#[derive(Resource)]
pub struct HexGrid {
    pub grid: Vec<Vec<Entity>>
}

impl HexGrid {

    //Even Masks
    const MASK_EVEN_TOP_LEFT: (i32, i32) = (1, 0);
    const MASK_EVEN_TOP_RIGHT: (i32, i32) = (1, 1);
    const MASK_EVEN_RIGHT: (i32, i32) = (0, 1);
    const MASK_EVEN_BOTTOM_RIGHT: (i32, i32) = (-1, 1);
    const MASK_EVEN_BOTTOM_LEFT: (i32, i32) = (-1, 0);
    const MASK_EVEN_LEFT: (i32, i32) = (0, -1);

    //Odd Masks
    const MASK_ODD_TOP_LEFT: (i32, i32) = (1, -1);
    const MASK_ODD_TOP_RIGHT: (i32, i32) = (1, 0);
    const MASK_ODD_RIGHT: (i32, i32) = (0, 1);
    const MASK_ODD_BOTTOM_RIGHT: (i32, i32) = (-1, 0);
    const MASK_ODD_BOTTOM_LEFT: (i32, i32) = (-1, -1);
    const MASK_ODD_LEFT: (i32, i32) = (0, -1);

    pub fn get_neigbors(&self, grid_pos: &GridPos) -> HexNeighbors {
        let pos = grid_pos.to_int();
        if grid_pos.row_is_even() {
            //Use even mask
            HexNeighbors {
                top_left: self.get_entity_offset(pos, HexGrid::MASK_EVEN_TOP_LEFT),
                top_right: self.get_entity_offset(pos, HexGrid::MASK_EVEN_TOP_RIGHT),
                left: self.get_entity_offset(pos, HexGrid::MASK_EVEN_LEFT),
                right: self.get_entity_offset(pos, HexGrid::MASK_EVEN_RIGHT),
                bottom_left: self.get_entity_offset(pos, HexGrid::MASK_EVEN_BOTTOM_LEFT),
                bottom_right: self.get_entity_offset(pos, HexGrid::MASK_EVEN_BOTTOM_RIGHT),
            }
        } else {
            //Use odd mask
            HexNeighbors {
                top_left: self.get_entity_offset(pos, HexGrid::MASK_ODD_TOP_LEFT),
                top_right: self.get_entity_offset(pos, HexGrid::MASK_ODD_TOP_RIGHT),
                left: self.get_entity_offset(pos, HexGrid::MASK_ODD_LEFT),
                right: self.get_entity_offset(pos, HexGrid::MASK_ODD_RIGHT),
                bottom_left: self.get_entity_offset(pos, HexGrid::MASK_ODD_BOTTOM_LEFT),
                bottom_right: self.get_entity_offset(pos, HexGrid::MASK_ODD_BOTTOM_RIGHT),
            }
        }
    }

    pub fn get_wind_neighors_new(&self, grid_pos: &GridPos, direction: f32) -> Option<Entity> {
        let pos = grid_pos.to_int();
        if grid_pos.row_is_even() {
            match HexDirection::from_angle(direction) {
                HexDirection::TopLeft => self.get_entity_offset(pos, HexGrid::MASK_EVEN_TOP_LEFT),
                HexDirection::TopRight => self.get_entity_offset(pos, HexGrid::MASK_EVEN_TOP_RIGHT),
                HexDirection::Right => self.get_entity_offset(pos, HexGrid::MASK_EVEN_RIGHT),
                HexDirection::BottomRight => self.get_entity_offset(pos, HexGrid::MASK_EVEN_BOTTOM_RIGHT),
                HexDirection::BottomLeft => self.get_entity_offset(pos, HexGrid::MASK_EVEN_BOTTOM_LEFT),
                HexDirection::Left => self.get_entity_offset(pos, HexGrid::MASK_EVEN_LEFT),
            }
        } else {
            match HexDirection::from_angle(direction) {
                HexDirection::TopLeft => self.get_entity_offset(pos, HexGrid::MASK_ODD_TOP_LEFT),
                HexDirection::TopRight => self.get_entity_offset(pos, HexGrid::MASK_ODD_TOP_RIGHT),
                HexDirection::Right => self.get_entity_offset(pos, HexGrid::MASK_ODD_RIGHT),
                HexDirection::BottomRight => self.get_entity_offset(pos, HexGrid::MASK_ODD_BOTTOM_RIGHT),
                HexDirection::BottomLeft => self.get_entity_offset(pos, HexGrid::MASK_ODD_BOTTOM_LEFT),
                HexDirection::Left => self.get_entity_offset(pos, HexGrid::MASK_ODD_LEFT),
            }
        }
    }

    pub fn direction_edges(&self, angle: f32) -> Vec<Entity> {
        return match angle % 360.0 {
            // Full Right
            ang if (337.5..360.0).contains(&ang) => {
                let mut rev_vec: Vec<Entity> = Vec::with_capacity(self.grid.len());
                for i in 0..self.grid.len() {
                    let row = self.grid.get(i).unwrap(); 
                    rev_vec.push(
                        row.get(row.len() - 1).unwrap().clone()
                    );
                }
                rev_vec
            },
            // Full Right
            ang if (0.0..22.5).contains(&ang) => {
                let mut rev_vec: Vec<Entity> = Vec::with_capacity(self.grid.len());
                for i in 0..self.grid.len() {
                    let row = self.grid.get(i).unwrap(); 
                    rev_vec.push(
                        row.get(row.len() - 1).unwrap().clone()
                    );
                }
                rev_vec
            },
            // Bottom Right
            ang if (22.5..67.5).contains(&ang) => {
                let bottom = self.grid.first().unwrap().clone().into_iter();
                let mut rev_vec: Vec<Entity> = Vec::with_capacity(self.grid.len() + bottom.len());
                for i in 0..self.grid.len() {
                    let row = self.grid.get(i).unwrap(); 
                    rev_vec.push(
                        row.get(row.len() - 1).unwrap().clone()
                    );
                }
                for entity in bottom {
                    rev_vec.push(entity)
                }
                rev_vec
            },
            // Full Bottom
            ang if (67.5..112.5).contains(&ang) => {
                self.grid.first().unwrap().clone()
            },
            // Bottom Left
            ang if (112.5..157.5).contains(&ang) => {
                let bottom = self.grid.first().unwrap().clone().into_iter();
                let mut rev_vec: Vec<Entity> = Vec::with_capacity(self.grid.len() + bottom.len());
                for i in 0..self.grid.len() {
                    let row = self.grid.get(i).unwrap(); 
                    rev_vec.push(
                        row.get(0).unwrap().clone()
                    );
                }
                for ent in bottom {
                    rev_vec.push(ent)
                }
                rev_vec
            },
            // Full Left
            ang if (157.5..202.5).contains(&ang) => {
                let mut rev_vec: Vec<Entity> = Vec::with_capacity(self.grid.len());
                for i in 0..self.grid.len() {
                    let row = self.grid.get(i).unwrap(); 
                    rev_vec.push(
                        row.get(0).unwrap().clone()
                    );
                }
                rev_vec
            },
            // Top Left
            ang if (202.5..247.5).contains(&ang) => {
                let top = self.grid.last().unwrap().clone().into_iter();
                let mut rev_vec: Vec<Entity> = Vec::with_capacity(self.grid.len() + top.len());
                for i in 0..self.grid.len() {
                    let row = self.grid.get(i).unwrap(); 
                    rev_vec.push(
                        row.get(0).unwrap().clone()
                    );
                }
                for ent in top {
                    rev_vec.push(ent)
                }
                rev_vec
            },
            // Full Top
            ang if (247.5..292.5).contains(&ang) => {
                self.grid.last().unwrap().clone()
            },
            // Top Right
            ang if (292.5..337.5).contains(&ang) => {
                let top = self.grid.last().unwrap().clone().into_iter();
                let mut rev_vec: Vec<Entity> = Vec::with_capacity(self.grid.len() + top.len());
                for i in 0..self.grid.len() {
                    let row = self.grid.get(i).unwrap(); 
                    rev_vec.push(
                        row.get(row.len() - 1).unwrap().clone()
                    );
                }
                for entity in top {
                    rev_vec.push(entity)
                }
                rev_vec      
            },
            _ => {
                eprintln!("angle not covered in match {}", &angle);
                self.grid.get(0).unwrap().clone()
            }
        }
    }

    fn get_entity_offset(&self, pos: (i32, i32), offset: (i32, i32)) -> Option<Entity> {
        let (row, col) = match (usize::try_from(pos.0 + offset.0), usize::try_from(pos.1 + offset.1)) {
            (Ok(r), Ok(c)) => (r, c),
            _ => return None
        };
        self.grid.get(row)?.get(col).cloned()
    }

}

#[derive(Debug)]
pub struct HexNeighbors {
    pub top_left: Option<Entity>,
    pub top_right: Option<Entity>,
    pub left: Option<Entity>,
    pub right: Option<Entity>,
    pub bottom_left: Option<Entity>,
    pub bottom_right: Option<Entity>
}

impl HexNeighbors {
    pub fn some_neighbors(&self) -> impl Iterator<Item = Entity> {
        [
            self.top_left,
            self.top_right,
            self.left,
            self.right,
            self.bottom_left,
            self.bottom_right
        ].into_iter()
        .filter_map(|option| option)
        .into_iter()
    }
}

#[derive(Resource)]
pub struct NaniteReserve {
    pub amount: f32
}

impl NaniteReserve {
    pub fn add_nanites(&mut self, amount: f32) {
        self.amount += amount
    }

    pub fn pull(&mut self) -> f32 {
        let mut rng = thread_rng();
        let amount = self.amount * rng.gen_range(0.0..1.0);
        self.amount -= amount;
        amount
    }
}

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