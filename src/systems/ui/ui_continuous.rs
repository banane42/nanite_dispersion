use bevy::{ecs::{system::{Query, Res, ResMut}, event::EventReader, query::{With, Changed}}, transform::components::Transform, math::{Quat, EulerRot}, text::Text, render::view::Visibility, ui::{Interaction, widget::Button}, input::{mouse::MouseButton, Input}};
use crate::{resources::{Weather, GameEntitiesClickable}, components::{clickable::OnClickEvents, grid_pos::GridPos, ui::{HexPosText, UICompass, RightInfoPane, ButtonOnClick}}};

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

pub fn ui_hex_click(
    mut reader: EventReader<OnClickEvents>,
    gp_q: Query<&GridPos>,
    mut text_q: Query<&mut Text, With<HexPosText>>,
    mut info_pane_q: Query<&mut Visibility, With<RightInfoPane>>
) {
    for event in reader.read() {
        match event {
            OnClickEvents::HexEvent(ent) => {
                match (gp_q.get(*ent), text_q.get_single_mut(), info_pane_q.get_single_mut()) {
                    (Ok(grid_pos), Ok(mut text), Ok(mut info_pane_vis)) => {
                        *info_pane_vis = Visibility::Visible;
                        text.sections.first_mut().unwrap().value = grid_pos.to_string();
                    },
                    _ => {}
                }
            }
        }
    }
}

pub fn ui_button_system(
    mut game_entities_clickable: ResMut<GameEntitiesClickable>,
    interaction_query: Query<
        (
            &Interaction,
            &ButtonOnClick
        ),
        (Changed<Interaction>, With<Button>)
    >,
    mut info_pane_vis_q: Query<&mut Visibility, With<RightInfoPane>>
) {
    for (interaction, button_on_click) in interaction_query.iter() {
        match interaction {
            Interaction::Pressed => {
                game_entities_clickable.0 = false;
                match button_on_click {
                    ButtonOnClick::InfoPaneClose => {
                        *info_pane_vis_q.get_single_mut().unwrap() = Visibility::Hidden;
                    }
                }
            },
            Interaction::Hovered => {},
            Interaction::None => {},
        }
    }
}

pub fn reset_game_entities_clickable(
    mut game_entities_clickable: ResMut<GameEntitiesClickable>,
    mut mouse_input: ResMut<Input<MouseButton>>
) {
    if !game_entities_clickable.0 && mouse_input.clear_just_released(MouseButton::Left) {
        game_entities_clickable.0 = true;
    }
}