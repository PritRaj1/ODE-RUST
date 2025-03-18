use std::error::Error;
use serde::Deserialize;
use std::fs;
use toml;

#[derive(Deserialize, Debug)]
struct Config {
    simulation: Simulation,
    noise: Noise,
}

#[derive(Deserialize, Debug)]
struct Simulation {
    timesteps: usize,
    dt: f64,
}

#[derive(Deserialize, Debug)]
struct Noise {
    enable: bool,
    stddev: f64,
}

fn parse_config(path: &str) -> Result<Config, Box<dyn std::error::Error>> {
    let contents = fs::read_to_string(path)?;
    let config: Config = toml::from_str(&contents)?;
    Ok(config)
}

// Test the config parser
fn main() -> Result<(), Box<dyn Error>> {
    let config = parse_config("config.ini")?;
    println!("{:?}", config);
    Ok(())
}
