use rand::{rngs::ThreadRng, Rng};

use super::PARTICLES_TYPES_AMOUNT;

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
                self.rules[y][x] = ((rng.gen::<f32>() - 0.5) * 100.).floor();
            }
        }
    }
    pub fn get(&self, a: usize, b: usize) -> &f32 {
        unsafe { self.rules.get_unchecked(a).get_unchecked(b) }
    }
    pub fn get_mut(&mut self, x: usize, y: usize) -> &mut f32 {
        &mut self.rules[y][x]
    }
}
