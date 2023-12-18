use bevy::{ecs::{system::{Query, Res, ResMut}, event::EventReader, query::{With, Changed, Without}}, transform::components::Transform, math::{Quat, EulerRot, Vec2}, text::Text, render::view::Visibility, ui::{Interaction, widget::Button}, input::{mouse::MouseButton, Input}};
use bevy_rapier2d::{plugin::RapierContext, pipeline::QueryFilter, geometry::{Collider, CollisionGroups, Group}};
use crate::{components::{grid_pos::GridPos, ui::{HexPosText, UICompass, RightInfoPane, ButtonOnClick, HexTerrainText, HexNaniteText}, terrain::Terrain, nanite::Nanite, macc::Macc, game_events::GameEvents}, resources::{weather::Weather, hex::{HexGrid, MapState}, input::GameEntitiesClickable}};

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

pub fn update_nanite_info_pane(
    hex_grid: Res<HexGrid>,
    nanite_q: Query<&Nanite, Changed<Nanite>>,
    mut nanite_text_q: Query<&mut Text, With<HexNaniteText>>
) {
    if let Some(selected_ent) = hex_grid.get_selected() {
        match (nanite_q.get(selected_ent), nanite_text_q.get_single_mut()) {
            (Ok(nanite), Ok(mut nanite_text)) => {
                nanite_text.sections.first_mut().unwrap().value = format!("Nanites\n{} / {}", nanite.nanite_total, nanite.nanite_capacity);
            },
            _ => {}
        }
    }
}

pub fn ui_game_event_react(
    mut hex_grid: ResMut<HexGrid>,
    rapier_context: Res<RapierContext>,
    mut game_event_reader: EventReader<GameEvents>,
    hex_q: Query<(&GridPos, &Terrain, &Transform, &Collider)>,
    mut pos_text_q: Query<&mut Text, (With<HexPosText>, Without<HexTerrainText>)>,
    mut terrain_text_q: Query<&mut Text, (With<HexTerrainText>, Without<HexPosText>)>,
    mut info_pane_q: Query<&mut Visibility, With<RightInfoPane>>,
    test_q: Query<(Option<&Nanite>, Option<&Macc>)>
) {
    for event in game_event_reader.read() {
        match event {
            GameEvents::HexSelect(ent) => {
                match (hex_q.get(*ent), pos_text_q.get_single_mut(), terrain_text_q.get_single_mut(), info_pane_q.get_single_mut()) {
                    (Ok((grid_pos, terrain, hex_trans, hex_coll)), Ok(mut pos_text), Ok(mut terrain_text), Ok(mut info_pane_vis)) => {
                        *info_pane_vis = Visibility::Visible;
                        pos_text.sections.first_mut().unwrap().value = format!("Coordinates\n{}", grid_pos.to_string());
                        terrain_text.sections.first_mut().unwrap().value = format!("Terrain Type\n{}", terrain.to_string());
                        hex_grid.select_pos(grid_pos.pos.clone());
                        get_maccs_in_hex(
                            &rapier_context, 
                            hex_coll, 
                            hex_trans.translation.truncate(),
                            &test_q
                        );
                    },
                    _ => {}
                }
            },
            GameEvents::MaccSelect(_) => {

            },
            GameEvents::MaccMoveOrder(_) => {

            },
        }
    }
}

fn get_maccs_in_hex(
    rapier_context: &RapierContext,
    hex_collider: &Collider,
    hex_pos: Vec2,
    test_q: &Query<(Option<&Nanite>, Option<&Macc>)>
) {
    rapier_context.intersections_with_shape(
        hex_pos, 
        0.0, 
        hex_collider, 
        QueryFilter::default().groups(CollisionGroups::new(Group::ALL, Group::GROUP_2)), 
        |ent| {
            println!("Get Maccs result\n{:?}", test_q.get(ent));
            true
        }
    );
}

pub fn ui_button_system(
    mut map_state: ResMut<MapState>,
    mut hex_grid: ResMut<HexGrid>,
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
                        hex_grid.deselect_pos();
                    }
                    ButtonOnClick::MapButtonTerrain => *map_state = MapState::Terrain,
                    ButtonOnClick::MapButtonNanite => *map_state = MapState::Nanite,
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