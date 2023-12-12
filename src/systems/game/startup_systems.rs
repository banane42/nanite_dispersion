use bevy::{ecs::{component::Component, system::{Commands, ResMut, Resource, Res}, entity::Entity}, core_pipeline::{core_2d::{Camera2dBundle, Camera2d}, clear_color::ClearColorConfig}, prelude::default, render::{color::Color, mesh::{Mesh, shape}}, math::{Vec2, Vec3}, sprite::{Mesh2dHandle, ColorMaterial, MaterialMesh2dBundle}, asset::Assets, transform::components::Transform, hierarchy::BuildChildren};
use bevy_rapier2d::geometry::Collider;

use crate::{resources::{MouseWorldCoords, HexGrid, Weather, NaniteReserve, GameEntitiesClickable, MapState}, components::{nanite::Nanite, grid_pos::GridPos, clickable::{Clickable, OnClickEvents}, terrain::Terrain}};

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
    commands.init_resource::<MapState>()
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
    mut meshes: ResMut<Assets<Mesh>>
) {
    //Meshes
    let outer_shape: Mesh = shape::RegularPolygon::new(50., 6).into(); 
    let collider_positions: Vec<Vec2> = outer_shape.attribute(Mesh::ATTRIBUTE_POSITION).unwrap().as_float3().unwrap().iter()
    .map(|x| {
        Vec2 {
            x: x[0],
            y: x[1],
        }
    }).collect();

    let outer_shape_handle = meshes.add(outer_shape);
    let inner_shape_handle: Mesh2dHandle = meshes.add(shape::RegularPolygon::new(49., 6).into()).into();

    commands.insert_resource(MeshHandles {
        out_hex_handle: outer_shape_handle.into(),
        inner_hex_handle: inner_shape_handle,
        hex_collider_positions: collider_positions,
    })
}

pub fn spawn_hexagons(
    mut commands: Commands,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mesh_handles: Res<MeshHandles>
) {
    let grid_width: usize = 16;
    let size: f32 = 50.0;
    let w = 3.0_f32.sqrt() * size;
    let h = 2.0 * size;

    let hex_grid: Vec<Vec<Entity>> = (0..grid_width).map(|row| {
        (0..grid_width).map(|col| {
            
            let ent = commands.spawn((MaterialMesh2dBundle {
                mesh: mesh_handles.get_out_hex_handle(),
                material: materials.add(ColorMaterial::from(Color::WHITE)),
                transform: Transform::from_translation(Vec3 {
                    x: (col as f32 * w - ((row % 2) as f32 * (w * 0.5))),
                    y: (row as f32 * h * 0.75),
                    z: 0.,
                }),
                ..default()
            },
            Collider::convex_polyline(mesh_handles.get_hex_collider_positions()).unwrap(),
            GridPos {
                pos: (row, col),
            },
            Nanite::new_empty(),
            Terrain::from_random(),
            )).with_children(|parent| {
                parent.spawn(MaterialMesh2dBundle {
                    mesh: mesh_handles.get_inner_hex_handle(),
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
    })

}

#[derive(Resource)]
pub struct MeshHandles {
    out_hex_handle: Mesh2dHandle,
    inner_hex_handle: Mesh2dHandle,
    hex_collider_positions: Vec<Vec2>
}

impl MeshHandles {
    pub fn get_out_hex_handle(&self) -> Mesh2dHandle {
        self.out_hex_handle.clone()
    }

    pub fn get_inner_hex_handle(&self) -> Mesh2dHandle {
        self.inner_hex_handle.clone()
    }

    pub fn get_hex_collider_positions(&self) -> Vec<Vec2> {
        self.hex_collider_positions.clone()
    }
}