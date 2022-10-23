use egui::Vec2;
use lerp::Lerp;
use rand::{rngs::ThreadRng, Rng};
use std::f32::consts::PI;

use super::Rules;

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
    pub fn get_force(&self, other: &Particle, rules: &Rules) -> Vec2 {
        if self as *const _ == other as *const _ {
            Vec2::ZERO
        } else {
            let rule = *rules.get(self.rule as usize, other.rule as usize);
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
    pub fn apply_force(&mut self, rng: &mut ThreadRng, force: Vec2) {
        let force: Vec2 = force.into();
        self.real_pos += force;
        let speed = force.length();
        if speed > 10. {
            let angle = rng.gen::<f32>() * PI * 2.;
            let dist = 120.;
            self.real_pos += Vec2::angled(angle) * dist;
            return;
        }
        self.visual_pos.x = self.visual_pos.x.lerp(self.real_pos.x, 0.2);
        self.visual_pos.y = self.visual_pos.y.lerp(self.real_pos.y, 0.2);
    }
}
