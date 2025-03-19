use serde::Deserialize;
use std::fs;
use toml;

#[derive(Deserialize, Debug)]
pub struct Config {
    pub simulation: Simulation,
    pub harmonic_oscillator: HarmonicOscillatorSystem,
    pub hodgkin_huxley: HodgkinHuxleySystem,
}

#[derive(Deserialize, Debug)]
pub struct Simulation {
    pub timesteps: usize,
    pub dt: f64,
    pub solver: String,
    pub rtol: f64,
    pub atol: f64,
    pub realtime_delay: f64,
    pub diffeq_problem: String,
}

#[derive(Deserialize, Debug)]
pub struct HarmonicOscillatorSystem {
    pub omega: f64,
}

#[derive(Deserialize, Debug)]
pub struct HodgkinHuxleySystem {
    pub g_na: f64,
    pub g_k: f64,
    pub g_l: f64,
    pub C: f64,
}

pub fn parse_config(path: &str) -> Result<Config, Box<dyn std::error::Error>> {
    let contents = fs::read_to_string(path)?;
    let config: Config = toml::from_str(&contents)?;

    Ok(config)
}
