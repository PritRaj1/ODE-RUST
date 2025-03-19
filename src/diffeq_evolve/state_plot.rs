use eframe::egui;
use egui_plot::{Plot, Line};

pub struct StatePlot<'a> {
    solver: &'a mut super::solver::Solver<'a>,
}

impl<'a> StatePlot<'a> {
    pub fn new(solver: &'a mut super::solver::Solver<'a>) -> Self {
        Self { solver }
    }
}

impl<'a> eframe::App for StatePlot<'a> {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            // Time series plot
            egui::Grid::new("plots_grid")
                .spacing([20.0, 20.0]) // Increased spacing
                .min_col_width(400.0) // Set minimum column width
                .show(ui, |ui| {
                    // First row: Time series plots
                    for i in 0..2 {
                        Plot::new(format!("State {}", self.solver.state_labels[i]))
                            .height(250.0) // Increased height
                            .width(500.0) // Fixed width
                            .x_axis_label("t")
                            .y_axis_label(&self.solver.state_labels[i])
                            .show(ui, |plot_ui| {
                                if !self.solver.trajectory.is_empty() {
                                    let points: Vec<[f64; 2]> = self.solver.trajectory.iter()
                                        .zip(self.solver.times.iter())
                                        .map(|(y, &t)| [t, y[i]])
                                        .collect();

                                    let line = Line::new(points)
                                        .name(&self.solver.state_labels[i])
                                        .color(match i {
                                            0 => egui::Color32::RED,
                                            1 => egui::Color32::GREEN,
                                            _ => egui::Color32::WHITE,
                                        });

                                    plot_ui.line(line);
                                }
                            });
                    }
                    ui.end_row();

                    // Second row: Time series plots
                    for i in 2..4 {
                        Plot::new(format!("State {}", self.solver.state_labels[i]))
                            .height(250.0) // Increased height
                            .width(500.0) // Fixed width
                            .x_axis_label("t")
                            .y_axis_label(&self.solver.state_labels[i])
                            .show(ui, |plot_ui| {
                                if !self.solver.trajectory.is_empty() {
                                    let points: Vec<[f64; 2]> = self.solver.trajectory.iter()
                                        .zip(self.solver.times.iter())
                                        .map(|(y, &t)| [t, y[i]])
                                        .collect();

                                    let line = Line::new(points)
                                        .name(&self.solver.state_labels[i])
                                        .color(match i {
                                            2 => egui::Color32::BLUE,
                                            3 => egui::Color32::YELLOW,
                                            _ => egui::Color32::WHITE,
                                        });

                                    plot_ui.line(line);
                                }
                            });
                    }
                    ui.end_row();

                    // Third row: Phase space plots
                    for i in 1..3 {
                        Plot::new(format!("Phase Portrait {}", i))
                            .height(250.0) // Increased height
                            .width(500.0) // Fixed width
                            .x_axis_label(&self.solver.state_labels[0])
                            .y_axis_label(&self.solver.state_labels[i])
                            .show(ui, |phase_plot| {
                                if !self.solver.trajectory.is_empty() {
                                    let phase_points: Vec<[f64; 2]> = self.solver.trajectory.iter()
                                        .map(|y| [y[0], y[i]])
                                        .collect();

                                    let line = Line::new(phase_points)
                                        .color(match i {
                                            1 => egui::Color32::GREEN,
                                            2 => egui::Color32::BLUE,
                                            _ => egui::Color32::WHITE,
                                        });

                                    phase_plot.line(line);
                                }
                            });
                    }
                    ui.end_row();

                    // Fourth row: Last phase space plot
                    Plot::new(format!("Phase Portrait {}", 3))
                        .height(250.0) // Increased height
                        .width(500.0) // Fixed width
                        .x_axis_label(&self.solver.state_labels[0])
                        .y_axis_label(&self.solver.state_labels[3])
                        .show(ui, |phase_plot| {
                            if !self.solver.trajectory.is_empty() {
                                let phase_points: Vec<[f64; 2]> = self.solver.trajectory.iter()
                                    .map(|y| [y[0], y[3]])
                                    .collect();

                                let line = Line::new(phase_points)
                                    .color(egui::Color32::YELLOW);

                                phase_plot.line(line);
                            }
                        });
                });

            // Take a simulation step and request repaint
            self.solver.step();
            ctx.request_repaint();
        });
    }
}


