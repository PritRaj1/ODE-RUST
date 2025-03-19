use nalgebra::{SVector};

#[derive(Clone)]
pub struct HarmonicOscillator{
    pub omega: f64,
}

impl HarmonicOscillator {
    fn new(omega: f64) -> Self {
        Self { omega }
    }

    pub fn dydx(&self, y: &SVector<f64, 4>, dy: &mut SVector<f64, 4>) {
        dy[0] = y[1];
        dy[1] = -self.omega * y[0];
        dy[2] = 0.0;
        dy[3] = 0.0;
    }
}
