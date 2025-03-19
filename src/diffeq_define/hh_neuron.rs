use nalgebra::{SVector};

#[derive(Clone)]
pub struct HodgkinHuxleyNeuron{
    pub g_na: f64,
    pub g_k: f64,
    pub g_l: f64,
    pub c: f64,
    pub v_na: f64,
    pub v_k: f64,
    pub v_l: f64,
}

fn alpha_m(v: f64) -> f64 {
    return 0.1 * (v + 40.0) / (1.0 - (-(v + 40.0) / 10.0).exp());
}

fn beta_m(v: f64) -> f64 {
    return 4.0 * (-(v + 65.0) / 18.0).exp();
}

fn alpha_h(v: f64) -> f64 {
    return 0.07 * (-(v + 65.0) / 20.0).exp();
}

fn beta_h(v: f64) -> f64 {
    return 1.0 / (1.0 + (-(v + 35.0) / 10.0).exp());
}

fn alpha_n(v: f64) -> f64 {
    return 0.01 * (v + 55.0) / (1.0 - (-(v + 55.0) / 10.0).exp());
}

fn beta_n(v: f64) -> f64 {
    return 0.125 * (-(v + 65.0) / 80.0).exp();
}

impl HodgkinHuxleyNeuron {
    fn new(g_na: f64, g_k: f64, g_l: f64, c: f64, v_na: f64, v_k: f64, v_l: f64) -> Self {
        Self { g_na, g_k, g_l, c, v_na, v_k, v_l }
    }

    pub fn dydx(&self, y: &SVector<f64, 4>, dy: &mut SVector<f64, 4>) {
        let v = y[0];
        let m = y[1];
        let n = y[2];
        let h = y[3];

        let alpha_m_v = alpha_m(v);
        let beta_m_v = beta_m(v);

        let alpha_h_v = alpha_h(v);
        let beta_h_v = beta_h(v);

        let alpha_n_v = alpha_n(v);
        let beta_n_v = beta_n(v);
    
        dy[0] = (self.g_na * m.powi(3) * h * (v - self.v_na) + self.g_k * n.powi(4) * (v - self.v_k) + self.g_l * (v - self.v_l)) / self.c;
        dy[1] = alpha_m_v * (1.0 - m) - beta_m_v * m;
        dy[2] = alpha_n_v * (1.0 - n) - beta_n_v * n;
        dy[3] = alpha_h_v * (1.0 - h) - beta_h_v * h;
    }
}
