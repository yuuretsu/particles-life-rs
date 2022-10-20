pub trait Lerp {
    fn lerp(self, b: Self, t: f64) -> Self;
}

impl Lerp for f64 {
    fn lerp(self, b: Self, t: f64) -> Self {
        self * (1. - t) + b * t
    }
}
