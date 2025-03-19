use ode_solvers::{Dopri5, Rk4, System};
use nalgebra::SVector;
use crate::utils::conf_parse::Config;
use crate::diffeq_define::harmonic_oscillator::HarmonicOscillator;
use crate::diffeq_define::hh_neuron::HodgkinHuxleyNeuron;
use std::time::{Duration, Instant};

#[derive(Clone)]
pub enum SystemType {
    HarmonicOscillator(HarmonicOscillator),
    HodgkinHuxley(HodgkinHuxleyNeuron),
}

impl System<f64, SVector<f64, 4>> for SystemType {
    fn system(&self, _t: f64, y: &SVector<f64, 4>, dy: &mut SVector<f64, 4>) {
        match self {
            SystemType::HarmonicOscillator(ho) => ho.dydx(y, dy),
            SystemType::HodgkinHuxley(hh) => hh.dydx(y, dy),
        }
    }
}

pub struct Solver<'a> {
    system: SystemType,
    t0: f64,
    pub t_end: f64,
    pub dt: f64,
    y0: SVector<f64, 4>,
    rtol: f64,
    atol: f64,
    pub trajectory: Vec<SVector<f64, 4>>,
    pub times: Vec<f64>,
    pub solver_type: String,
    pub state_labels: Vec<String>,
    pub realtime_delay: Duration,
    config: &'a Config,
}

impl<'a> Solver<'a> {
    pub fn new(config: &'a Config) -> Self {
        let system = match config.simulation.diffeq_problem.as_str() {
            "harmonic_oscillator" => SystemType::HarmonicOscillator(
                HarmonicOscillator {
                    omega: config.harmonic_oscillator.omega,
                }
            ),
            "hodgkin_huxley" => SystemType::HodgkinHuxley(
                HodgkinHuxleyNeuron {
                    g_na: config.hodgkin_huxley.g_na,
                    g_k: config.hodgkin_huxley.g_k,
                    g_l: config.hodgkin_huxley.g_l,
                    c: config.hodgkin_huxley.c,
                    v_na: config.hodgkin_huxley.v_na,
                    v_k: config.hodgkin_huxley.v_k,
                    v_l: config.hodgkin_huxley.v_l,
                }
            ),
            _ => panic!("Unknown system type"),
        };

        let t0 = 0.0;
        let dt = config.simulation.dt;
        let t_end = t0 + (config.simulation.timesteps as f64) * dt;

        // Set initial conditions based on the system type
        let y0 = match &system {
            SystemType::HarmonicOscillator(_) => {
                // Initial displacement = 1.0, initial velocity = 0.0
                SVector::from_vec(vec![1.0, 0.0, 0.0, 0.0])
            },
            SystemType::HodgkinHuxley(_) => {
                // Initial voltage = -65mV, gates at steady state
                SVector::from_vec(vec![-65.0, 0.05, 0.32, 0.6])
            },
        };

        let state_labels = match &system {
            SystemType::HarmonicOscillator(_) => 
                vec!["x".to_string(), "v".to_string(), "unused".to_string(), "unused".to_string()],
            SystemType::HodgkinHuxley(_) => 
                vec!["V".to_string(), "m".to_string(), "n".to_string(), "h".to_string()],
        };

        Self {
            system,
            t0,
            t_end,
            dt,
            y0,
            rtol: config.simulation.rtol,
            atol: config.simulation.atol,
            trajectory: Vec::new(),
            times: Vec::new(),
            solver_type: config.simulation.solver.clone(),
            state_labels,
            realtime_delay: Duration::from_millis(config.simulation.realtime_delay as u64),
            config,
        }
    }

    pub fn solve(&mut self) {
        match self.solver_type.as_str() {
            "dopri5" => {
                let mut solver = Dopri5::new(
                    self.system.clone(),
                    self.t0, 
                    self.t_end, 
                    self.dt, 
                    self.y0, 
                    self.rtol, 
                    self.atol
                );
                solver.integrate().unwrap();
                self.trajectory = solver.y_out().iter().map(|v| v.clone()).collect();
                self.times = solver.x_out().iter().map(|&t| t).collect();
            },
            "runge_kutta_4" => {
                let mut solver = Rk4::new(
                    self.system.clone(),
                    self.t0, 
                    self.y0, 
                    self.t_end, 
                    self.dt
                );
                solver.integrate().unwrap();
                self.trajectory = solver.y_out().iter().map(|v| v.clone()).collect();
                self.times = solver.x_out().iter().map(|&t| t).collect();
            },
            _ => panic!("Unsupported solver"),
        }
    }

    pub fn step(&mut self) {
        let start_time = Instant::now();

        // If trajectory is empty, initialize with initial conditions
        if self.trajectory.is_empty() {
            self.trajectory.push(self.y0);
            self.times.push(self.t0);
        }

        let last_t = *self.times.last().unwrap();
        let last_y = *self.trajectory.last().unwrap();

        // Only continue if we haven't reached t_end
        if last_t < self.t_end {
            let next_t = (last_t + self.dt).min(self.t_end);

            match self.solver_type.as_str() {
                "dopri5" => {
                    let mut solver = Dopri5::new(
                        self.system.clone(),
                        last_t,
                        next_t,
                        self.dt,
                        last_y,
                        self.rtol,
                        self.atol
                    );
                    solver.integrate().unwrap();
                    if let Some(y) = solver.y_out().last() {
                        self.trajectory.push(y.clone());
                        self.times.push(next_t);
                    }
                },
                "runge_kutta_4" => {
                    let mut solver = Rk4::new(
                        self.system.clone(),
                        last_t,
                        last_y,
                        next_t,
                        self.dt
                    );
                    solver.integrate().unwrap();
                    if let Some(y) = solver.y_out().last() {
                        self.trajectory.push(y.clone());
                        self.times.push(next_t);
                    }
                },
                _ => panic!("Unsupported solver"),
            }
        }

        // Maintain real-time rate
        let elapsed = start_time.elapsed();
        if elapsed < self.realtime_delay {
            std::thread::sleep(self.realtime_delay - elapsed);
        }
    }
}


