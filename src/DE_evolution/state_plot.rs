use eframe::egui;
use egui_plot::{Plot, Line, PlotPoints};
use crate::DE_evolution::solver::Solver;
use eframe::App;

pub struct StatePlot {
    solver: Solver,
}

impl StatePlot {
    pub fn new(solver: Solver) -> Self {
        Self { solver }
    }
}

impl eframe::App for StatePlot {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            let plot = Plot::new("time_series")
                .view_aspect(3.0)
                .show_axes(true)
                .x_axis_label("Time, t")
                .y_axis_label("State Values");
            
            // Trajectories as time series
            plot.show(ui, |plot_ui| {

                if !self.solver.trajectory.is_empty() {
                    let pos_points: Vec<[f64; 2]> = self.solver.trajectory.iter()
                        .enumerate()
                        .map(|(i, point)| [i as f64 * self.solver.dt, point[0]])
                        .collect();
                    
                    plot_ui.line(Line::new(pos_points)
                        .name("Position"));
                }

                if !self.solver.trajectory.is_empty() {
                    let vel_points: Vec<[f64; 2]> = self.solver.trajectory.iter()
                        .enumerate()
                        .map(|(i, point)| [i as f64 * self.solver.dt, point[1]])
                        .collect();
                    
                    plot_ui.line(Line::new(vel_points)
                        .name("Velocity"));
                }
            });

            // Phase portrait; state 2 vs state 1
            let phase_plot = Plot::new("phase_plot")
                .view_aspect(1.0)
                .show_axes(true)
                .x_axis_label("Position, x")
                .y_axis_label("Velocity, dx/dt")
                .auto_bounds_x()
                .auto_bounds_y();

            phase_plot.show(ui, |plot_ui| {

                if !self.solver.trajectory.is_empty() {
                    let phase_points: Vec<[f64; 2]> = self.solver.trajectory.iter()
                        .map(|point| [point[0], point[1]])
                        .collect();
                    
                    plot_ui.line(Line::new(phase_points)
                        .name("Phase Portrait")
                        .color(egui::Color32::YELLOW));
                }
            });

            // Take a simulation step
            if self.solver.trajectory.len() as f64 * self.solver.dt < self.solver.t_end {
                self.solver.step();
                ctx.request_repaint();
            }
        });
    }
}


