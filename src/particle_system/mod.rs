mod particle;
mod rules;
use std::f32::consts::PI;

use egui::Vec2;
use rand::{rngs::ThreadRng, Rng};
pub use rules::Rules;

use self::particle::Particle;

pub struct ParticleSystem {
    particles: [Particle; PARTICLES_AMOUNT],
}

impl ParticleSystem {
    pub fn new(rng: &mut ThreadRng) -> Self {
        let particle = Particle::default();
        let mut list = [particle; PARTICLES_AMOUNT];
        for i in 0..PARTICLES_AMOUNT {
            let angle = rng.gen::<f32>() * PI * 2.;
            let distance = rng.gen::<f32>().sqrt() * 250.;
            list[i] = Particle::new(
                Vec2::angled(angle) * distance,
                rng.gen::<u8>() % PARTICLES_TYPES_AMOUNT as u8,
            );
        }
        Self { particles: list }
    }
    fn get_forces(&self, rules: &Rules) -> [Vec2; PARTICLES_AMOUNT] {
        let mut forces = [Vec2::ZERO; PARTICLES_AMOUNT];
        for (i, particle) in self.particles.iter().enumerate() {
            for other_particle in &self.particles {
                forces[i] += particle.get_force(other_particle, rules);
            }
        }
        forces
    }
    pub fn step(&mut self, rng: &mut ThreadRng, rules: &Rules) {
        let forces = self.get_forces(rules);
        for i in 0..forces.len() {
            self.particles[i].apply_force(rng, forces[i]);
        }
    }
}

impl<'a> IntoIterator for &'a ParticleSystem {
    type Item = Particle;
    type IntoIter = std::array::IntoIter<Particle, PARTICLES_AMOUNT>;

    fn into_iter(self) -> Self::IntoIter {
        self.particles.into_iter()
    }
}

pub const PARTICLES_AMOUNT: usize = 2000;
pub const PARTICLES_TYPES_AMOUNT: usize = 4;
