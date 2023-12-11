use bevy::{ecs::system::{Query, Res}, transform::components::Transform, math::{Quat, EulerRot}};

use crate::resources::Weather;
use super::ui_setup::UICompass;

pub fn update_compass(
    weather: Res<Weather>,
    mut compass_q: Query<(&UICompass, &mut Transform)>
) {
    match compass_q.get_single_mut() {
        Ok((_, mut trans)) => {
            trans.rotation = Quat::from_euler(EulerRot::XYZ, 
                0.0, 
                0.0, 
                weather.wind_direction.to_radians()
            );
        },
        Err(_) => {},
    }
}