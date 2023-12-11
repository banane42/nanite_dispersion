use bevy::ecs::component::Component;
use rand::{thread_rng, Rng};

#[derive(Component)]
pub struct Nanite {
    pub nanite_capacity: f32,
    pub nanite_total: f32,
    pub transient_nanites: f32
}

impl Nanite {

    pub fn new_empty() -> Self {
        Self {
            nanite_capacity: 20.0,
            nanite_total: 0.0,
            transient_nanites: 0.0,
        }
    }

    pub fn new_rand_filled() -> Self {
        let mut rng = thread_rng();
        
        Self { 
            nanite_capacity: 20.0, 
            nanite_total: 20.0 * rng.gen_range(0.0..1.0), 
            transient_nanites: 0.0
        }
    }

    pub fn add_transient_nanites(&mut self, amount: f32) {
        self.transient_nanites += amount;
    }

    pub fn apply_transient_nanites(&mut self) {
        self.nanite_total += self.transient_nanites;
        self.transient_nanites = 0.0;
    }

    pub fn spill(&mut self) -> f32 {
        let overflow = (self.nanite_total - self.nanite_capacity).max(0.0);
        self.nanite_total -= overflow;
        overflow
    }

    pub fn wind_pull(&mut self, strength: f32) -> f32 {
        let amount = self.nanite_total * strength.clamp(0.0, 1.0);
        self.nanite_total -= amount;
        amount
    }

    pub fn is_full(&self) -> bool {
        self.nanite_total > self.nanite_capacity
    }

}