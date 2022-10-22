use rand::{rngs::ThreadRng, Rng};

use super::{particle::Particle, PARTICLES_TYPES_AMOUNT};

#[derive(Default)]
pub struct Rules {
    rules: [[f32; PARTICLES_TYPES_AMOUNT]; PARTICLES_TYPES_AMOUNT],
}

impl Rules {
    pub fn new() -> Self {
        Self {
            ..Default::default()
        }
    }
    pub fn fill_random(&mut self, rng: &mut ThreadRng) {
        for y in 0..PARTICLES_TYPES_AMOUNT {
            for x in 0..PARTICLES_TYPES_AMOUNT {
                self.rules[y][x] = (rng.gen::<f32>() - 0.5) * 100.;
            }
        }
    }
    pub fn get(&self, a: &Particle, b: &Particle) -> f32 {
        self.rules[a.rule as usize][b.rule as usize]
    }
}
