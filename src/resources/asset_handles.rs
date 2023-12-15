use bevy::{ecs::{system::Resource, schedule::States}, sprite::{Mesh2dHandle, ColorMaterial}, math::Vec2, render::texture::Image, asset::Handle};
use bevy_rapier2d::geometry::Collider;

#[derive(States, Default, Debug, Hash, Eq, PartialEq, Clone)]
pub enum LoadingStates {
    #[default]
    Loading, 
    Complete
}

#[derive(Resource)]
pub struct AssetHandles {
    pub out_hex_handle: Mesh2dHandle,
    pub inner_hex_handle: Mesh2dHandle,
    pub macc_image_handle: Handle<Image>,
    pub color_handle_white: Handle<ColorMaterial>,
}

impl AssetHandles {
    pub fn get_out_hex_handle(&self) -> Mesh2dHandle {
        self.out_hex_handle.clone()
    }

    pub fn get_inner_hex_handle(&self) -> Mesh2dHandle {
        self.inner_hex_handle.clone()
    }

    pub fn get_sprite_handle_macc(&self) -> Handle<Image> {
        self.macc_image_handle.clone()
    }

    pub fn get_color_handle_white(&self) -> Handle<ColorMaterial> {
        self.color_handle_white.clone()
    }
}

#[derive(Resource)]
pub struct ColliderAssets {
    pub macc_collider: Collider,
    pub hex_collider: Collider,
}

impl ColliderAssets {

    pub fn get_hex(&self) -> Collider {
        self.hex_collider.clone()
    }

    pub fn get_macc(&self) -> Collider {
        self.macc_collider.clone()
    }
}