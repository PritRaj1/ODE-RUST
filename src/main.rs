use crate::diffeq_evolve::solver::Solver;
use crate::diffeq_evolve::state_plot::StatePlot;
use crate::utils::conf_parse::parse_config;
use eframe::{NativeOptions, egui};

mod diffeq_evolve;
mod diffeq_define;
mod utils;

fn main() {
    let config = parse_config("config.ini").unwrap();

    let mut solver = Solver::new(&config);
    let state_plot = StatePlot::new(&mut solver);

    if config.simulation.show_plot {
        let mut native_options = NativeOptions::default();
        native_options.viewport.inner_size = Some(egui::vec2(1200.0, 800.0));
        eframe::run_native(
            "Differential Equation Solver",
            native_options,
                Box::new(|_| Ok(Box::new(state_plot)))
            ).unwrap();
    }
}