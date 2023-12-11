use bevy::{ecs::{system::{Query, ResMut, Res}, query::Changed, entity::Entity}, hierarchy::Children, asset::{Handle, Assets}, sprite::ColorMaterial, render::color::Color};
use rand::{seq::SliceRandom, thread_rng, Rng};

use crate::{nanite::{Nanite, GridPos}, resources::{HexGrid, Weather, NaniteReserve}};

pub fn nanite_wind(
    hex_grid: Res<HexGrid>,
    weather: Res<Weather>,
    mut nanite_reserve: ResMut<NaniteReserve>,
    grid_pos_q: Query<&GridPos>,
    mut nanite_q: Query<&mut Nanite>
) {
    for rows in hex_grid.grid.iter() {
        for current_entity in rows.iter() {
            let grid_pos = grid_pos_q.get(*current_entity).unwrap();
            let neighbor = hex_grid.get_wind_neighors_new(grid_pos, weather.wind_direction);
            let nanite_pool = nanite_q.get_mut(*current_entity).unwrap().wind_pull(weather.wind_strength);
            
            match neighbor {
                Some(ent) => nanite_q.get_mut(ent).unwrap().add_transient_nanites(nanite_pool),
                None => nanite_reserve.add_nanites(nanite_pool),
            }
        }
    }
}

pub fn nanite_dispersion(
    hex_grid: Res<HexGrid>,
    grid_pos_q: Query<&GridPos>,
    mut nanite_q: Query<&mut Nanite>
) {
    for rows in hex_grid.grid.iter() {
        for current_ent in rows.iter() {
            if !nanite_q.get(*current_ent).unwrap().is_full() {
                continue;
            }

            let grid_pos = grid_pos_q.get(*current_ent).unwrap();
            let neighbors = hex_grid.get_neigbors(grid_pos);
            
            let mut low_neighbors: Vec<Entity> = Vec::with_capacity(6);
            for neighbor_ent in neighbors.some_neighbors() {
                let [curr_nan, neigh_nan] = nanite_q.get_many([current_ent.clone(), neighbor_ent]).unwrap();
                if curr_nan.nanite_total > neigh_nan.nanite_total {
                    low_neighbors.push(neighbor_ent.clone());
                }
            }
            let nanites_to_add = nanite_q.get_mut(*current_ent).unwrap().spill() / low_neighbors.len() as f32;
            
            low_neighbors.iter().for_each(|neigh_ent| {
                nanite_q.get_mut(*neigh_ent).unwrap().add_transient_nanites(nanites_to_add);
            })
        }
    }
}

pub fn nanite_introduction(
    hex_grid: Res<HexGrid>,
    weather: Res<Weather>,
    mut nanite_reserve: ResMut<NaniteReserve>,
    mut nanite_q: Query<&mut Nanite>
) {
    let mut rng = thread_rng();
    let mut nanite_pool = nanite_reserve.pull();
    let edges = hex_grid.direction_edges(weather.wind_direction + 180.0);

    while nanite_pool > 0.0 {
        let ent = edges.choose(&mut rng).unwrap();
        let mut nanite = nanite_q.get_mut(*ent).unwrap();
        
        let amount = if nanite_pool <= 5.0 {
            nanite_pool
        } else {
            nanite.nanite_capacity * rng.gen_range(0.0..1.0)
        };
        nanite_pool -= amount;
        
        nanite.add_transient_nanites(amount);
    }
}

pub fn nanite_transient_apply(
    mut nanite_q: Query<&mut Nanite, Changed<Nanite>>
) {
    for mut nanite in nanite_q.iter_mut() {
        nanite.apply_transient_nanites();
    }
}

pub fn adjust_wind(
    mut weather: ResMut<Weather>
) {
    weather.adjust_wind();
}

pub fn nanite_material_update(
    mut materials: ResMut<Assets<ColorMaterial>>,
    nanite_q: Query<(&Children, &Nanite), Changed<Nanite>>,
    material_q: Query<&mut Handle<ColorMaterial>>
) {
    for (children, nanite) in nanite_q.iter() {
        children.iter().for_each(|child| {
            match material_q.get(*child) {
                Ok(handle) => {
                    let col = if nanite.nanite_total > 0.0 {
                        Color::rgba(
                            0.25, 
                            0.42, 
                            0.85,
                            (nanite.nanite_total as f32 / nanite.nanite_capacity as f32).clamp(0.1, 1.0)
                        )
                    } else {
                        Color::GRAY
                    };
                    materials.get_mut(handle).unwrap().color = col;
                },
                Err(_) => {
                    eprintln!("No material component on nanite's child")
                }
            }
        });
    }
}