use bevy::{ecs::{component::Component, system::{Commands, ResMut, Res}, entity::Entity, schedule::NextState}, core_pipeline::{core_2d::{Camera2dBundle, Camera2d}, clear_color::ClearColorConfig}, prelude::default, render::{color::Color, mesh::{Mesh, shape}, texture::Image}, math::{Vec2, Vec3}, sprite::{Mesh2dHandle, ColorMaterial, MaterialMesh2dBundle}, asset::{Assets, AssetServer, Handle}, transform::components::Transform, hierarchy::BuildChildren};
use bevy_rapier2d::geometry::{Sensor, Collider};
use bevy_rapier_collider_gen::single_convex_polyline_collider_translated;

use crate::{resources::{weather::Weather, hex::{NaniteReserve, MapState, HexGrid}, input::{GameEntitiesClickable, MouseWorldCoords, SelectedMacc}, asset_handles::{AssetHandles, ColliderAssets, LoadingStates}}, bundles::{hex_bundle::HexBundle, macc_bundle::MaccBundle}, components::clickable::{Clickable, OnClickEvents}};

#[derive(Component)]
pub struct MainCamera {
    target_scale: f32,
    start_scale: f32,
    scale_extents: (f32, f32),
    scale_factor: f32,
    start_time: f32,
    zoom_duration: f32,
    pub translation_speed: f32
}

impl MainCamera {
    pub fn adjust_zoom_target(&mut self, up: bool, start_time: f32, start_scale: f32) {
        self.start_time = start_time;
        self.start_scale = start_scale;
        self.target_scale = if up {
            self.target_scale + self.scale_factor
        } else {
            self.target_scale - self.scale_factor
        }.clamp(self.scale_extents.0, self.scale_extents.1)
    }

    pub fn interp_zoom(&mut self, current_time: f32) -> f32 {
        let t = (current_time - self.start_time) / self.zoom_duration;
        let lin_interp = (1.0 - t) * self.start_scale + t * self.target_scale;
        if t >= 1.0 {
            self.target_scale
        } else {
            lin_interp
        }
    }
}

pub fn setup(
    mut commands: Commands
) {
    commands.insert_resource(Weather {
        wind_strength: 1.0,
        wind_direction: 0.0,
    });

    commands.insert_resource(NaniteReserve {
        amount: 1000.0
    });

    commands.init_resource::<GameEntitiesClickable>();
    commands.init_resource::<MapState>();
    commands.init_resource::<SelectedMacc>()
}

pub fn setup_camera(
    mut commands: Commands
) {
    commands.init_resource::<MouseWorldCoords>();

    let cam_bundle = Camera2dBundle {
        camera_2d: Camera2d { clear_color: ClearColorConfig::Custom(Color::BLACK) },
        ..default()
    };

    let main_camera = MainCamera {
        target_scale: cam_bundle.projection.scale,
        start_scale: cam_bundle.projection.scale,
        scale_extents: (0.5, 3.0),
        scale_factor: 0.1,
        start_time: 0.0,
        zoom_duration: 0.25,
        translation_speed: 10.0
    };

    commands.spawn((
        cam_bundle,  
        main_camera
    ));
}

pub fn setup_assets(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    //Meshes
    let outer_shape: Mesh = shape::RegularPolygon::new(50., 6).into(); 

    let outer_shape_handle = meshes.add(outer_shape);
    let inner_shape_handle: Mesh2dHandle = meshes.add(shape::RegularPolygon::new(49., 6).into()).into();

    let macc_handle: Handle<Image> = asset_server.load("macc.png");

    let color_handle_white = materials.add(ColorMaterial::from(Color::WHITE));

    commands.insert_resource(AssetHandles {
        out_hex_handle: outer_shape_handle.into(),
        inner_hex_handle: inner_shape_handle,
        macc_image_handle: macc_handle,
        color_handle_white: color_handle_white,
    });
}

pub fn spawn_hexagons(
    mut commands: Commands,
    mut materials: ResMut<Assets<ColorMaterial>>,
    asset_handles: Res<AssetHandles>,
    colliders: Res<ColliderAssets>
) {
    let grid_width: usize = 16;

    let hex_grid: Vec<Vec<Entity>> = (0..grid_width).map(|row| {
        (0..grid_width).map(|col| {
            let hex_bundle = HexBundle::new(row, col, &asset_handles, &colliders);
            
            let ent = commands.spawn((hex_bundle, Sensor)).with_children(|parent| {
                parent.spawn(MaterialMesh2dBundle {
                    mesh: asset_handles.get_inner_hex_handle(),
                    material: materials.add(ColorMaterial::from(Color::GRAY)),
                    transform: Transform::from_translation(Vec3::new(0., 0., 1.0)),
                    ..default()
                });
            }).id();

            commands.entity(ent).insert(Clickable::new(OnClickEvents::HexEvent(ent)));

            ent
        }).collect()
    }).collect();

    commands.insert_resource(HexGrid {
        grid: hex_grid,
        selected_pos: None
    });

    let macc_1 = commands.spawn(MaccBundle::new(Vec2 {
        x: 0.0,
        y: 0.0,
    }, asset_handles.get_sprite_handle_macc(), colliders.get_macc())).id();
    commands.entity(macc_1).insert(Clickable::new(OnClickEvents::MaccEvent(macc_1)));

    let macc_2 = commands.spawn(MaccBundle::new(Vec2 {
        x: 5.0,
        y: 0.0,
    }, asset_handles.get_sprite_handle_macc(), colliders.get_macc())).id();
    commands.entity(macc_2).insert(Clickable::new(OnClickEvents::MaccEvent(macc_2)));
}

pub fn create_colliders(
    mut commands: Commands,
    images: ResMut<Assets<Image>>,
    asset_handles: Res<AssetHandles>,
    meshes: Res<Assets<Mesh>>,
    mut loading_state: ResMut<NextState<LoadingStates>>
) {
    let hex_collider = match meshes.get(asset_handles.get_out_hex_handle().0) {
        Some(mesh) => {
            let collider_positions: Vec<Vec2> = mesh.attribute(Mesh::ATTRIBUTE_POSITION).unwrap().as_float3().unwrap().iter()
            .map(|x| {
                Vec2 {
                    x: x[0],
                    y: x[1],
                }
            }).collect();
            Collider::convex_polyline(collider_positions).unwrap()
        },
        None => return,
    };

    let macc_collider = match images.get(asset_handles.get_sprite_handle_macc()) {
        Some(img) => {
            single_convex_polyline_collider_translated(img).unwrap()
        },
        None => return,
    };

    commands.insert_resource(ColliderAssets {
        macc_collider,
        hex_collider
    });

    loading_state.set(LoadingStates::Complete);

}