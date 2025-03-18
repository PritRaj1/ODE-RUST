use crate::DE_evolution::solver::Solver;
use crate::DE_evolution::state_plot::StatePlot;
use crate::utils::conf_parse::parse_config;
use eframe::NativeOptions;

mod DE_evolution;
mod DiffEqs;
mod utils;

fn main() {
    let config = parse_config("config.ini").unwrap();

    let solver = Solver::new(&config.simulation);
    let state_plot = StatePlot::new(solver);

    let native_options = NativeOptions::default();      
    eframe::run_native(
        "State Plot",
        native_options,
        Box::new(|_| Ok(Box::new(state_plot)))
    ).unwrap();
}