use bevy::{ecs::bundle::Bundle, prelude::default, sprite::SpriteBundle, math::{Vec2, Vec3}, transform::components::Transform, asset::Handle, render::texture::Image};
use bevy_rapier2d::geometry::Collider;

use crate::{components::macc::{Team, Macc}, resources::asset_handles::{AssetHandles, ColliderAssets}};

#[derive(Bundle)]
pub struct MaccBundle {
    team: Team,
    macc: Macc,
    sprite: SpriteBundle,
    collider: Collider
}

impl MaccBundle {
    pub fn new(position: Vec2, sprite: Handle<Image>, collider: Collider) -> Self {
        let trans = Transform::from_translation(position.extend(3.0)).with_scale(Vec3 {
            x: 0.05,
            y: 0.05,
            z: 1.0,
        });
        let sprite = SpriteBundle {
            transform: trans,
            texture: sprite,
            ..default()
        };
        

        Self {
            team: Team::A,
            macc: Macc {},
            sprite: sprite,
            collider: collider
        }
    }
}