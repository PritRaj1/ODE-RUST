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
        
        // Tight spacing and margins
        let mut style = (*ctx.style()).clone();
        style.spacing.item_spacing = egui::Vec2::ZERO;
        style.spacing.window_margin = egui::Margin::same(2 as i8); 
        ctx.set_style(style);

        egui::CentralPanel::default().show(ctx, |ui| {
            egui::Grid::new("plots_grid")
                .spacing([0.0, 0.0]) 
                .num_columns(2)
                .min_col_width(200.0)
                .show(ui, |ui| {
                    // First row: First state time series and Poincare plot
                    Plot::new(format!("State {}", self.solver.state_labels[0]))
                        .height(250.0)
                        .width(250.0)
                        .x_axis_label("t (ms)")
                        .y_axis_label(&self.solver.state_labels[0])
                        .show(ui, |plot_ui| {
                            if !self.solver.trajectory.is_empty() {
                                let points: Vec<[f64; 2]> = self.solver.trajectory.iter()
                                    .zip(self.solver.times.iter())
                                    .map(|(y, &t)| [t, y[0]])
                                    .collect();

                                let line = Line::new(points)
                                    .name(&self.solver.state_labels[0])
                                    .color(egui::Color32::WHITE);

                                plot_ui.line(line);
                            }
                        });

                    Plot::new(format!("Poincare {}", self.solver.state_labels[0]))
                        .height(250.0)
                        .width(250.0)
                        .x_axis_label(&format!("{} (t)", self.solver.state_labels[0]))
                        .y_axis_label(&format!("{} (t+1)", self.solver.state_labels[0]))
                        .show(ui, |plot_ui| {
                            if self.solver.trajectory.len() > 1 {
                                let poincare_points: Vec<[f64; 2]> = self.solver.trajectory.windows(2)
                                    .map(|w| [w[0][0], w[1][0]])
                                    .collect();

                                let line = Line::new(poincare_points)
                                    .name(&format!("Poincare {}", self.solver.state_labels[0]))
                                    .color(egui::Color32::YELLOW);

                                plot_ui.line(line);
                            }
                        });

                    ui.end_row();

                    // Subsequent rows: Phase portraits and Poincare plots
                    for i in 1..4 {
                        Plot::new(format!("Phase Portrait {}", i))
                            .height(250.0)
                            .width(250.0)
                            .x_axis_label(&self.solver.state_labels[0])
                            .y_axis_label(&self.solver.state_labels[i])
                            .show(ui, |phase_plot| {
                                if !self.solver.trajectory.is_empty() {
                                    let phase_points: Vec<[f64; 2]> = self.solver.trajectory.iter()
                                        .map(|y| [y[0], y[i]])
                                        .collect();

                                    let line = Line::new(phase_points)
                                        .color(match i {
                                            0 => egui::Color32::YELLOW,
                                            1 => egui::Color32::GREEN,
                                            2 => egui::Color32::YELLOW,
                                            3 => egui::Color32::RED,
                                            _ => egui::Color32::WHITE,
                                        });

                                    phase_plot.line(line);
                                }
                            });

                        Plot::new(format!("Poincare {}", self.solver.state_labels[i]))
                            .height(250.0)
                            .width(250.0)
                            .x_axis_label(&format!("{} (t)", self.solver.state_labels[i]))
                            .y_axis_label(&format!("{} (t+1)", self.solver.state_labels[i]))
                            .show(ui, |plot_ui| {
                                if self.solver.trajectory.len() > 1 {
                                    let poincare_points: Vec<[f64; 2]> = self.solver.trajectory.windows(2)
                                        .map(|w| [w[0][i], w[1][i]])
                                        .collect();

                                    let line = Line::new(poincare_points)
                                        .color(match i {
                                            0 => egui::Color32::YELLOW,
                                            1 => egui::Color32::GREEN,
                                            2 => egui::Color32::YELLOW,
                                            3 => egui::Color32::RED,
                                            _ => egui::Color32::WHITE,
                                        });

                                    plot_ui.line(line);
                                }
                            });

                        ui.end_row();
                    }
                });

            // Take a simulation step and request repaint
            self.solver.step();
            ctx.request_repaint();
        });
    }
}
