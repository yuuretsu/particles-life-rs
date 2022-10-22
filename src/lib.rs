mod helpers;
mod lerp;
mod random;

use ::lerp::Lerp;
use egui::Vec2;
use helpers::is_same_pointer;
use rand::rngs::ThreadRng;
use random::Random;
use std::f32::consts::PI;

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
                self.rules[y][x] = (f32::random(rng) - 0.5) * 100.;
            }
        }
    }
    pub fn get(&self, a: &Particle, b: &Particle) -> f32 {
        self.rules[a.rule as usize][b.rule as usize]
    }
}

#[derive(Clone, Copy, Debug, Default)]
pub struct Particle {
    pub real_pos: Vec2,
    pub visual_pos: Vec2,
    pub rule: u8,
}

impl Particle {
    pub fn new(pos: Vec2, t: u8) -> Self {
        Self {
            real_pos: pos,
            visual_pos: pos,
            rule: t,
        }
    }
    fn get_force(&self, other: &Particle, rules: &Rules) -> Vec2 {
        if is_same_pointer(self, other) {
            Vec2::ZERO
        } else {
            let rule = rules.get(self, other);
            let dx_dy = self.real_pos - other.real_pos;
            let d2 = dx_dy.length_sq();
            let normalised_d2 = d2.max(100.);
            let distance = d2.sqrt();
            if distance > 50. || distance == 0. {
                return Vec2::ZERO;
            }
            let cur = (-10.).lerp(rule, distance.powf(0.8) * 0.03) * 20.;
            let angle = dx_dy.angle();
            let speed = (1. / normalised_d2) * -cur;

            Vec2::angled(angle) * speed
        }
    }
    fn apply_force(&mut self, rng: &mut ThreadRng, force: Vec2) {
        let force: Vec2 = force.into();
        self.real_pos += force;
        let speed = force.length();
        if speed > 10. {
            let angle = f32::random(rng) * PI * 2.;
            let dist = 120.;
            self.real_pos += Vec2::angled(angle) * dist;
            return;
        }
        self.visual_pos.x = self.visual_pos.x.lerp(self.real_pos.x, 0.2);
        self.visual_pos.y = self.visual_pos.y.lerp(self.real_pos.y, 0.2);
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
            let angle = f32::random(rng) * PI * 2.;
            let distance = f32::random(rng).sqrt() * 250.;
            let x = angle.cos() * distance;
            let y = angle.sin() * distance;
            list[i] = Particle::new(
                Vec2::new(x, y),
                u8::random(rng) % PARTICLES_TYPES_AMOUNT as u8,
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

impl<'a> IntoIterator for &'a Particles {
    type Item = Particle;
    type IntoIter = std::array::IntoIter<Particle, PARTICLES_AMOUNT>;

    fn into_iter(self) -> Self::IntoIter {
        self.particles.into_iter()
    }
}

const PARTICLES_AMOUNT: usize = 2000;
pub const PARTICLES_TYPES_AMOUNT: usize = 4;
