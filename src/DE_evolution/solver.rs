use ode_solvers::{Dopri5, Rk4};
use crate::DiffEqs::harmonic_oscillator::HarmonicOscillator;
use crate::utils::conf_parse::Simulation;
use nalgebra::SVector;
use std::time::{Duration, Instant};
use eframe::egui;

pub struct Solver {
    pub trajectory: Vec<[f64; 2]>,
    pub solver_type: String,
    pub t0: f64,
    pub y0: SVector<f64, 2>,
    pub t_end: f64,
    pub dt: f64,
    pub rtol: f64,
    pub atol: f64,
    pub state_labels: Vec<String>,
    pub realtime_delay: Duration,
}

impl Solver {
    pub fn new(config: &Simulation) -> Self {
        let y0 = SVector::<f64, 2>::new(1.0, 0.0);

        let dt = config.dt;
        let t0 = 0.0;
        let t_end = t0 + (config.timesteps as f64) * dt;
        let rtol = config.rtol;
        let atol = config.atol;

        let state_labels = vec!["x".to_string(), "v".to_string()];
        let t_delay = Duration::from_millis(config.realtime_delay as u64);

        Solver {
            trajectory: Vec::new(),
            solver_type: config.solver.clone(),
            t0,
            y0,
            t_end,
            dt,
            rtol,
            atol,
            state_labels,
            realtime_delay: t_delay,
        }
    }

    pub fn solve(&mut self) {
        match self.solver_type.as_str() {
            "dopri5" => {
                let mut solver = Dopri5::new(HarmonicOscillator, self.t0, self.t_end, self.dt, self.y0, self.rtol, self.atol);
                solver.integrate().unwrap();
                self.trajectory = solver.y_out().iter().map(|v| [v[0], v[1]]).collect();
            },
            "runge_kutta_4" => {
                let mut solver = Rk4::new(HarmonicOscillator, self.t0, self.y0, self.t_end, self.dt);
                solver.integrate().unwrap();
                self.trajectory = solver.y_out().iter().map(|v| [v[0], v[1]]).collect();
            },
            _ => panic!("Unsupported solver"),
        }
    }

    pub fn step(&mut self) {
        let start_time = Instant::now();

        // If we already have points, use the last point as initial condition
        let current_t = if self.trajectory.is_empty() {
            self.t0
        } else {
            self.t0 + (self.trajectory.len() as f64) * self.dt
        };

        let current_y = if self.trajectory.is_empty() {
            self.y0
        } else {
            let last_point = self.trajectory.last().unwrap();
            SVector::<f64, 2>::new(last_point[0], last_point[1])
        };

        let next_t = current_t + self.dt;

        match self.solver_type.as_str() {
            "dopri5" => {
                let mut solver = Dopri5::new(
                    HarmonicOscillator,
                    current_t,
                    next_t,
                    self.dt,
                    current_y,
                    self.rtol,
                    self.atol
                );
                solver.integrate().unwrap();
                if let Some(point) = solver.y_out().last() {
                    self.trajectory.push([point[0], point[1]]);
                }
            },
            "runge_kutta_4" => {
                let mut solver = Rk4::new(
                    HarmonicOscillator,
                    current_t,
                    current_y,
                    next_t,
                    self.dt
                );
                solver.integrate().unwrap();
                if let Some(point) = solver.y_out().last() {
                    self.trajectory.push([point[0], point[1]]);
                }
            },
            _ => panic!("Unsupported solver"),
        }

        std::thread::sleep(self.realtime_delay);
        let elapsed = start_time.elapsed();
        if elapsed < self.realtime_delay {
            std::thread::sleep(self.realtime_delay - elapsed);
        }
    }
}


