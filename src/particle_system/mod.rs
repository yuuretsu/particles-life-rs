mod particle;
mod rules;
use std::{f32::consts::PI, cmp::max, vec};

use std::time::Instant;

use egui::Vec2;
use rand::{rngs::ThreadRng, Rng};
pub use rules::Rules;

use crate::{gay_tree::{self, GayTree, point}, gay_quad::Borders};

use self::particle::{Particle, INTERACTION_DISTANCE};

pub struct ParticleSystem {
    particles: [Particle; PARTICLES_AMOUNT],
    pub tree_chuck_capacity: usize
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
        Self { particles: list, tree_chuck_capacity:120}
    }
    fn get_forces(&self, rules: &Rules) -> [Vec2; PARTICLES_AMOUNT] {
        let mut ne:Vec2 = Vec2 { x: 600., y: 600. };
        let mut sw:Vec2 = Vec2 { x: -600., y: -600. };

        let mut forces = [Vec2::ZERO; PARTICLES_AMOUNT];
        //let start = Instant::now();
        let mut tree:GayTree = GayTree::new(Borders { SE: (Vec2 { x: (sw.x), y: (ne.y) }), SW: (sw), NW: (Vec2 { x: (ne.x), y: (sw.y) }), NE: (ne) }, self.tree_chuck_capacity);

        for (i, particle) in self.particles.iter().enumerate() 
        {
            tree.insert(point{ id: i, pos: particle.real_pos });
        }

        for (i, particle) in self.particles.iter().enumerate() {
            let other_points: Vec<usize> = tree.retrieve(&particle.real_pos, INTERACTION_DISTANCE);
            for other_point in other_points {
            let other_particle = &self.particles[other_point];
                forces[i] += particle.get_force(other_particle, rules);
            }
        }
        //let end = Instant::now();
        //let elapsed = end.duration_since(start);
        //println!("Elapsed time: {:?}", elapsed);
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
pub const PARTICLES_TYPES_AMOUNT: usize = 12;
