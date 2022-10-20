use rand::rngs::ThreadRng;
use rand::Rng;

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
