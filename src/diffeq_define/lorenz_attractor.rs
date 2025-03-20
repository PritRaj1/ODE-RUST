use nalgebra::{SVector};

#[derive(Clone)]
pub struct LorenzAttractor {
    pub sigma: f64,
    pub rho: f64,
    pub beta: f64,
}

impl LorenzAttractor {
    pub fn new(sigma: f64, rho: f64, beta: f64) -> Self {
        Self { sigma, rho, beta }
    }

    pub fn dydx(&self, y: &SVector<f64, 4>, dy: &mut SVector<f64, 4>) {
        dy[0] = self.sigma * (y[1] - y[0]);
        dy[1] = y[0] * (self.rho - y[2]) - y[1];
        dy[2] = y[0] * y[1] - self.beta * y[2];
        dy[3] = 0.0;
    }
}
