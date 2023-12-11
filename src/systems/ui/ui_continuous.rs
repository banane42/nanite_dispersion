use bevy::{ecs::{system::{Query, Res}, event::EventReader, component::Component, query::With}, transform::components::Transform, math::{Quat, EulerRot}, text::Text};

use crate::{resources::Weather, components::{clickable::OnClickEvents, grid_pos::GridPos}};
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

#[derive(Component)]
pub struct HexPosText;

pub fn ui_hex_click(
    mut reader: EventReader<OnClickEvents>,
    gp_q: Query<&GridPos>,
    mut text_q: Query<&mut Text, With<HexPosText>>
) {
    for event in reader.read() {
        match event {
            OnClickEvents::HexEvent(ent) => {
                match (gp_q.get(*ent), text_q.get_single_mut()) {
                    (Ok(grid_pos), Ok(mut text)) => {
                        text.sections.first_mut().unwrap().value = grid_pos.to_string();
                    },
                    _ => {}
                }
            }
        }
    }
}