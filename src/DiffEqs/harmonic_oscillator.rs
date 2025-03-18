use nalgebra::{SVector};
use ode_solvers::System;

pub struct HarmonicOscillator;

impl System<f64, SVector<f64, 2>> for HarmonicOscillator {
    fn system(&self, _t: f64, y: &SVector<f64, 2>, dy: &mut SVector<f64, 2>) {
        dy[0] = y[1];
        dy[1] = -y[0];
    }
}
