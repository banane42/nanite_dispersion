use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use components::clickable::OnClickEvents;
use resources::{GameEntitiesClickable, MapState};
use systems::{game::{startup_systems::{setup_camera, setup_assets, spawn_hexagons, setup}, continuous_systems::map_state_material_static}, game::input_systems::{calc_world_coords, on_game_entity_click, keyboard_input, mouse_input, zoom_camera}, game::continuous_systems::{nanite_material_update, nanite_dispersion, nanite_wind, nanite_introduction, nanite_transient_apply}, ui::{ui_setup::ui_setup, ui_continuous::{update_compass, ui_hex_click, ui_button_system, reset_game_entities_clickable, update_nanite_info_pane}}};

mod resources;
mod systems;
mod components;

fn main() {

    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(1.0 ))
        .add_event::<OnClickEvents>()
        //Startup
        //Game Systems
        .add_systems(PreStartup, setup)
        .add_systems(PreStartup, setup_assets)
        .add_systems(Startup, setup_camera)
        .add_systems(Startup, spawn_hexagons)
        //UI Systems
        .add_systems(Startup, ui_setup)
        // Update Loop
        .add_systems(First, calc_world_coords)
        .add_systems(First, keyboard_input)
        .add_systems(First, mouse_input)
        .add_systems(First, ui_button_system.before(reset_game_entities_clickable))
        .add_systems(PreUpdate, zoom_camera)
        .add_systems(PreUpdate, on_game_entity_click.run_if(game_entities_clickable))
        //Nanite systems
        .add_systems(Update, ((
                nanite_introduction,
                nanite_wind,
                nanite_dispersion
            )).run_if(time_passed(1.0))
        )
        .add_systems(Update, ui_hex_click)
        .add_systems(PostUpdate, nanite_transient_apply)
        //Graphics update
        .add_systems(Last, map_state_material_static.run_if(map_state_changed))
        .add_systems(Last, nanite_material_update.run_if(
            resource_exists::<MapState>().and_then(|state: Res<MapState>| *state == MapState::Nanite))
        )
        .add_systems(Last, update_compass)
        .add_systems(Last, update_nanite_info_pane)
        .add_systems(Last, reset_game_entities_clickable)
        .run();
}

fn time_passed(t: f32) -> impl FnMut(Local<f32>, Res<Time>) -> bool {
    move |mut timer: Local<f32>, time: Res<Time>| {
        // Tick the timer
        *timer += time.delta_seconds();
        // Return true if the timer has passed the time
        let run  = *timer >= t;
        if run {
            *timer = 0.0;
        }
        run
    }
}

fn game_entities_clickable(clickable: Res<GameEntitiesClickable>) -> bool {
    clickable.0
}

fn map_state_changed(
    map_state: Res<MapState>
) -> bool {
    map_state.is_changed()
}