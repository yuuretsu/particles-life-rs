mod helpers;
mod lerp;
mod random;

use helpers::compare_pointers;
pub use lerp::Lerp;
use rand::rngs::ThreadRng;
pub use random::Random;
use std::f64::consts::PI;

#[derive(Default)]
pub struct Rules {
    rules: [[f64; PARTICLES_TYPES_AMOUNT]; PARTICLES_TYPES_AMOUNT],
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
                self.rules[y][x] = (f64::random(rng) - 0.5) * 100.;
            }
        }
    }
    pub fn get(&self, a: &Particle, b: &Particle) -> f64 {
        self.rules[a.rule as usize][b.rule as usize]
    }
}

#[derive(Clone, Copy, Debug, Default)]
pub struct Particle {
    pub real_x: f64,
    pub real_y: f64,
    pub visual_x: f64,
    pub visual_y: f64,
    pub rule: u8,
}

impl Particle {
    pub fn new(x: f64, y: f64, t: u8) -> Self {
        Self {
            real_x: x,
            real_y: y,
            rule: t,
            visual_x: x,
            visual_y: y,
        }
    }
    fn apply_force(&mut self, rng: &mut ThreadRng, force: (f64, f64)) {
        self.real_x += force.0;
        self.real_y += force.1;
        let speed = force.0.hypot(force.1);
        if speed > 10. {
            let angle = f64::random(rng) * PI * 2.;
            let dist = 120.;
            self.real_x += angle.cos() * dist;
            self.real_y += angle.sin() * dist;
            // self.visual_x = self.real_x;
            // self.visual_y = self.real_y;
            return;
        }
        self.visual_x = self.visual_x.lerp(self.real_x, 0.2);
        self.visual_y = self.visual_y.lerp(self.real_y, 0.2);
    }
}

pub struct Particles {
    particles: [Particle; PARTICLES_AMOUNT],
}

impl Particles {
    pub fn new(rng: &mut ThreadRng) -> Self {
        let particle = Particle::default();
        let mut list = [particle; PARTICLES_AMOUNT];
        for i in 0..PARTICLES_AMOUNT {
            let angle = f64::random(rng) * PI * 2.;
            let distance = f64::random(rng).sqrt() * 250.;
            let x = angle.cos() * distance;
            let y = angle.sin() * distance;
            list[i] = Particle::new(x, y, u8::random(rng) % PARTICLES_TYPES_AMOUNT as u8);
        }
        Self { particles: list }
    }
    fn get_forces(&self, rules: &Rules) -> [(f64, f64); PARTICLES_AMOUNT] {
        let mut forces = [(0.0, 0.0); PARTICLES_AMOUNT];
        for (i, particle) in self.particles.iter().enumerate() {
            for other_particle in &self.particles {
                if compare_pointers(particle, other_particle) {
                    continue;
                };
                let rule = rules.get(particle, other_particle);
                let dx = particle.real_x - other_particle.real_x;
                let dy = particle.real_y - other_particle.real_y;
                let d2 = dx * dx + dy * dy;
                let normalised_d2 = d2.max(100.);
                let distance = d2.sqrt();
                if distance > 50. || distance == 0. {
                    continue;
                }
                let cur = (-10.).lerp(rule, distance.powf(0.8) * 0.03) * 20.;
                let angle = dy.atan2(dx);
                let speed = (1. / normalised_d2) * -cur;
                forces[i].0 += angle.cos() * speed;
                forces[i].1 += angle.sin() * speed;
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

impl<'a> IntoIterator for &'a Particles {
    type Item = Particle;
    type IntoIter = std::array::IntoIter<Particle, PARTICLES_AMOUNT>;

    fn into_iter(self) -> Self::IntoIter {
        self.particles.into_iter()
    }
}

const PARTICLES_AMOUNT: usize = 2000;
pub const PARTICLES_TYPES_AMOUNT: usize = 6;
