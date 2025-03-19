use nalgebra::{SVector};

#[derive(Clone)]
pub struct HodgkinHuxleyNeuron{
    pub g_na: f64,
    pub g_k: f64,
    pub g_l: f64,   
    pub e_na: f64,
    pub e_k: f64,
    pub e_l: f64,
    pub i_ext_amplitude: f64,
    pub i_ext_start: usize,
    pub i_ext_end: usize,
}

fn alpha_m(v: f64) -> f64 {
    return (2.5 - 0.1 * v) / ((2.5 - 0.1 * v).exp() - 1.0);
}

fn beta_m(v: f64) -> f64 {
    return 4.0 * (-v / 18.0).exp()
}

fn alpha_h(v: f64) -> f64 {
    return 0.07 * (-v / 20.0).exp()
}

fn beta_h(v: f64) -> f64 {
    return 1.0 / (1.0 + (3.0 - 0.1 * v).exp());
}

fn alpha_n(v: f64) -> f64 {
    return (0.1 - 0.01 * v) / ((1.0 - 0.1 * v).exp() - 1.0);
}

fn beta_n(v: f64) -> f64 {
    return 0.125 * (-v / 80.0).exp()
}

impl HodgkinHuxleyNeuron {
    fn new(g_na: f64, g_k: f64, g_l: f64, e_na: f64, e_k: f64, e_l: f64, i_ext_amplitude: f64, i_ext_start: usize, i_ext_end: usize) -> Self {
        Self { g_na, g_k, g_l, e_na, e_k, e_l, i_ext_amplitude, i_ext_start, i_ext_end }
    }

    pub fn dydx(&self, y: &SVector<f64, 4>, dy: &mut SVector<f64, 4>, t: f64) {
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

        let i_ext = if t >= self.i_ext_start as f64 && t <= self.i_ext_end as f64 {
            self.i_ext_amplitude
        } else {
            0.0
        };
    
        dy[0] = - self.g_na * m.powi(3) * h * (v - self.e_na) - self.g_k * n.powi(4) * (v - self.e_k) - self.g_l * (v - self.e_l) + i_ext;
        dy[1] = alpha_m_v * (1.0 - m) - beta_m_v * m;
        dy[2] = alpha_n_v * (1.0 - n) - beta_n_v * n;
        dy[3] = alpha_h_v * (1.0 - h) - beta_h_v * h;
    }
}
