use bevy::ecs::component::Component;
use bevy::render::color::Color;
use rand::Rng;

#[derive(Component)]
pub enum Terrain {
    Land, Water
}

impl Terrain {
    pub fn from_random() -> Self {
        let mut rng = rand::thread_rng();
        match rng.gen_range(0..=1) {
            0 => Terrain::Land,
            _ => Terrain::Water
        }
    }
}

impl From<&Terrain> for Color {
    fn from(value: &Terrain) -> Self {
        match value {
            Terrain::Land => Color::GREEN,
            Terrain::Water => Color::BLUE,
        }
    }
}

// impl Into<Color> for Terrain {
//     fn into(self) -> Color {
//         match self {
//             Terrain::Land => Color::GREEN,
//             Terrain::Water => Color::BLUE
//         }
//     }
// }