use bevy::{sprite::{MaterialMesh2dBundle, ColorMaterial}, prelude::default, transform::components::Transform, math::Vec3, ecs::bundle::Bundle};
use bevy_rapier2d::geometry::Collider;

use crate::{components::{grid_pos::GridPos, terrain::Terrain, nanite::Nanite}, resources::{asset_handles::{AssetHandles, ColliderAssets}, hex::HexGrid}};

#[derive(Bundle)]
pub struct HexBundle {
    material_mesh_bundle: MaterialMesh2dBundle<ColorMaterial>,
    collider: Collider,
    grid_pos: GridPos,
    nanite: Nanite,
    terrain: Terrain
}

impl HexBundle {
    pub fn new(row: usize, col: usize, asset_handles: &AssetHandles, colliders: &ColliderAssets) -> Self {
        let w = 3.0_f32.sqrt() * HexGrid::HEX_RADIUS;
        let h = 2.0 * HexGrid::HEX_RADIUS;
        Self { 
            material_mesh_bundle: MaterialMesh2dBundle {
                mesh: asset_handles.get_out_hex_handle(),
                material: asset_handles.get_color_handle_white(),
                transform: Transform::from_translation(Vec3 {
                    x: (col as f32 * w - ((row % 2) as f32 * (w * 0.5))),
                    y: (row as f32 * h * 0.75),
                    z: 0.,
                }),
                ..default()
            }, 
            collider: colliders.get_hex(),
            grid_pos: GridPos { pos: (row, col) }, 
            nanite: Nanite::new_empty(), 
            terrain: Terrain::from_random()
        }
    }
}