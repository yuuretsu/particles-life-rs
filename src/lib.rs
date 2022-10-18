use std::f64::consts::PI;

use rand::{rngs::ThreadRng, Rng};

pub trait Random {
    fn random(rng: &mut ThreadRng) -> Self;
}

impl Random for f32 {
    fn random(rng: &mut ThreadRng) -> Self {
        rng.gen()
    }
}

impl Random for f64 {
    fn random(rng: &mut ThreadRng) -> Self {
        rng.gen()
    }
}

impl Random for u8 {
    fn random(rng: &mut ThreadRng) -> Self {
        rng.gen()
    }
}

impl Random for macroquad::prelude::Color {
    fn random(rng: &mut ThreadRng) -> Self {
        let r = f32::random(rng);
        let g = f32::random(rng);
        let b = f32::random(rng);
        macroquad::prelude::Color::new(r, g, b, 1.)
    }
}

trait Lerp {
    fn lerp(self, b: Self, t: f64) -> Self;
}

impl Lerp for f64 {
    fn lerp(self, b: Self, t: f64) -> Self {
        self * (1. - t) + b * t
    }
}

pub struct Rules {
    rules: [[f64; PARTICLES_TYPES]; PARTICLES_TYPES],
}

impl Rules {
    pub fn new(rng: &mut ThreadRng) -> Self {
        let mut rules = [[0.0; PARTICLES_TYPES]; PARTICLES_TYPES];
        for y in 0..PARTICLES_TYPES {
            for x in 0..PARTICLES_TYPES {
                rules[y][x] = (f64::random(rng) - 0.5) * 100.;
            }
        }
        Self { rules }
    }
    fn get(&self, a: &Particle, b: &Particle) -> f64 {
        self.rules[a.t as usize][b.t as usize]
    }
}

#[derive(Clone, Copy)]
pub struct Particle {
    pub x: f64,
    pub y: f64,
    pub t: u8,
}

impl Particle {
    pub fn new(x: f64, y: f64, t: u8) -> Self {
        Self { x, y, t }
    }
    fn apply_force(&mut self, rng: &mut ThreadRng, force: (f64, f64)) {
        let mut sx = force.0;
        let mut sy = force.1;
        let speed = sx.hypot(sy);
        if speed > 10. {
            let angle = f64::random(rng) * PI * 2.;
            let dist = 60.;
            sx = angle.cos() * dist;
            sy = angle.sin() * dist;
        }
        self.x += sx;
        self.y += sy;
    }
}

impl Default for Particle {
    fn default() -> Self {
        Self::new(0.0, 0.0, 0)
    }
}

pub struct Particles {
    particles: [Particle; PARTICLES_AMOUNT],
}

impl Particles {
    pub fn new(rng: &mut ThreadRng) -> Self {
        let mut list = [Particle::default(); PARTICLES_AMOUNT];
        for i in 0..PARTICLES_AMOUNT {
            let angle = f64::random(rng) * PI * 2.;
            let distance = f64::random(rng).sqrt() * 250.;
            let x = angle.cos() * distance;
            let y = angle.sin() * distance;
            list[i] = Particle::new(x, y, u8::random(rng) % PARTICLES_TYPES as u8);
        }
        Self { particles: list }
    }
    pub fn list(&self) -> &[Particle; PARTICLES_AMOUNT] {
        &self.particles
    }
    fn get_forces(&self, rules: &Rules) -> [(f64, f64); PARTICLES_AMOUNT] {
        let mut forces = [(0.0, 0.0); PARTICLES_AMOUNT];
        for (i, particle) in self.particles.iter().enumerate() {
            for other_particle in &self.particles {
                if (&particle as *const _) == (&other_particle as *const _) {
                    continue;
                };
                let rule = rules.get(particle, other_particle);
                let dx = particle.x - other_particle.x;
                let dy = particle.y - other_particle.y;
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
            self.particles[i].apply_force(rng, forces[i])
        }
    }
}

const PARTICLES_AMOUNT: usize = 2000;
pub const PARTICLES_TYPES: usize = 6;
