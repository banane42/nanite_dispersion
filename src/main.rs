use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use systems::{game::startup_systems::{setup_camera, setup_assets, spawn_hexagons, setup}, game::input_systems::{calc_world_coords, on_click, keyboard_input, mouse_input, zoom_camera}, game::continuous_systems::{nanite_material_update, nanite_dispersion, nanite_wind, nanite_introduction, nanite_transient_apply}, ui::{ui_setup::ui_setup, ui_continuous::update_compass}};

mod nanite;
mod resources;
mod systems;

fn main() {

    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(1.0 ))
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
        .add_systems(PreUpdate, on_click)
        .add_systems(PreUpdate, zoom_camera)
        //Nanite systems
        .add_systems(Update, ((
                nanite_introduction,
                nanite_wind,
                nanite_dispersion
            )).run_if(time_passed(1.0))
        )
        .add_systems(PostUpdate, nanite_transient_apply)
        //Graphics update
        .add_systems(PostUpdate, nanite_material_update)
        .add_systems(PostUpdate, update_compass)
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