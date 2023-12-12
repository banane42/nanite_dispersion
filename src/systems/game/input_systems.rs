use bevy::{ecs::{system::{ResMut, Query, Res}, query::With, event::{EventReader, EventWriter}}, window::{PrimaryWindow, Window}, render::camera::{Camera, OrthographicProjection}, transform::components::{GlobalTransform, Transform}, input::{Input, mouse::{MouseButton, MouseWheel}, keyboard::KeyCode}, math::Vec3, time::Time};
use bevy_rapier2d::{plugin::RapierContext, pipeline::QueryFilter};

use crate::{resources::{MouseWorldCoords, Weather, MapState}, components::clickable::{Clickable, OnClickEvents}};

use super::startup_systems::MainCamera;

pub fn calc_world_coords(
    mut mouse_wrld_coords: ResMut<MouseWorldCoords>,
    q_window: Query<&Window, With<PrimaryWindow>>,
    q_camera: Query<(&Camera, &GlobalTransform), With<MainCamera>>,
) {
    let (camera, camera_transform) = q_camera.single();
    let window = q_window.single();

    if let Some(world_position) = window.cursor_position()
        .and_then(|cursor| camera.viewport_to_world(camera_transform, cursor))
        .map(|ray| ray.origin.truncate()) {
        mouse_wrld_coords.0 = world_position;
    }
}

pub fn keyboard_input(
    keys: Res<Input<KeyCode>>,
    mut map_state: ResMut<MapState>,
    mut weather: ResMut<Weather>,
    mut camera_q: Query<(&mut Transform, &MainCamera)>
) {
    let (mut camera_trans, camera) = camera_q.get_single_mut().unwrap();
    //Movement Input
    // Forward/Backward
    if keys.pressed(KeyCode::W) {
        camera_trans.translation += Vec3::Y * camera.translation_speed;
    } else if keys.pressed(KeyCode::S) {
        camera_trans.translation -= Vec3::Y * camera.translation_speed;
    }

    //Left/Right
    if keys.pressed(KeyCode::A) {
        camera_trans.translation -= Vec3::X * camera.translation_speed;
    } else if keys.pressed(KeyCode::D) {
        camera_trans.translation += Vec3::X * camera.translation_speed;
    }

    //TODO Remove test
    if keys.just_released(KeyCode::Left) {
        weather.debug_wind_adj(true);
    } else if keys.just_released(KeyCode::Right) {
        weather.debug_wind_adj(false);
    }

    //TODO Remove Test
    if keys.just_released(KeyCode::Space) {
        let state = map_state.as_ref().clone();
        match state {
            MapState::Terrain => *map_state = MapState::Nanite,
            MapState::Nanite => *map_state = MapState::Terrain,
        }
    }

}

pub fn mouse_input(
    time: Res<Time>,
    mut scroll_events: EventReader<MouseWheel>,
    mut camera_q: Query<(&OrthographicProjection, &mut MainCamera)>
) {
    let (ortho_proj, mut camera) = camera_q.get_single_mut().unwrap();
    
    scroll_events.read().for_each(|event| {
        camera.adjust_zoom_target(event.y.signum() != 1.0, time.elapsed_seconds_wrapped(), ortho_proj.scale);
    });
}

pub fn zoom_camera(
    time: Res<Time>,
    mut camera_q: Query<(&mut OrthographicProjection, &mut MainCamera)>
) {
    let (mut ortho_proj, mut camera) = camera_q.get_single_mut().unwrap();
    ortho_proj.scale = camera.interp_zoom(time.elapsed_seconds_wrapped());
}

pub fn on_game_entity_click(
    mouse_wrld_coords: Res<MouseWorldCoords>,
    mouse_input: Res<Input<MouseButton>>,
    rapier_context: Res<RapierContext>,
    mut event_writer: EventWriter<OnClickEvents>,
    clickable_q: Query<&Clickable>
) {
    if mouse_input.just_released(MouseButton::Left) {
        rapier_context.intersections_with_point(
            mouse_wrld_coords.0, 
            QueryFilter::default(), 
            |entity| {
                match clickable_q.get(entity) {
                    Ok(clickable) => {
                        event_writer.send(clickable.get_event());
                        false
                    },
                    Err(_) => true,
                }
            })
    }
}