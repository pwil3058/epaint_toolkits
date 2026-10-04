// Copyright (c) 2026 Peter Williams <pwil3058@bigpond.net.au> <pwil3058@gmail.com>.

use colour_math::{ColourBasics, manipulator::ColourManipulator, rgb::RGB};
use eframe::egui;

pub struct ColourMatrixDashboard {
    pub active_manipulator: ColourManipulator,
    pub matrix_rows: usize,
    pub matrix_cols: usize,
    pub selected_cell: Option<(usize, usize)>,
}

impl Default for ColourMatrixDashboard {
    fn default() -> Self {
        Self {
            active_manipulator: ColourManipulator::builder().build(),
            matrix_rows: 5,
            matrix_cols: 5,
            selected_cell: None,
        }
    }
}

impl ColourMatrixDashboard {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn ui(&mut self, ui: &mut egui::Ui) {
        ui.vertical(|ui| {
            ui.heading("📋 Colour Matrix Dashboard Grid");
            ui.add_space(4.0);
            ui.separator();
            ui.add_space(4.0);

            // 1. Render Grid Layout
            egui::Grid::new("colour_matrix_display_grid")
                .spacing(egui::vec2(6.0, 6.0))
                .show(ui, |ui| {
                    for row in 0..self.matrix_rows {
                        for col in 0..self.matrix_cols {
                            let is_selected = self.selected_cell == Some((row, col));

                            let (rect, resp) = ui
                                .allocate_exact_size(egui::vec2(48.0, 48.0), egui::Sense::click());

                            ui.painter()
                                .rect_filled(rect, 4.0, egui::Color32::from_gray(96));

                            if is_selected {
                                ui.painter().rect_stroke(
                                    rect,
                                    4.0,
                                    egui::Stroke::new(2.0, egui::Color32::WHITE),
                                    egui::StrokeKind::Middle,
                                );
                            } else if resp.hovered() {
                                ui.painter().rect_stroke(
                                    rect,
                                    4.0,
                                    egui::Stroke::new(1.0, egui::Color32::LIGHT_GRAY),
                                    egui::StrokeKind::Middle,
                                );
                            }

                            if resp.clicked() {
                                self.selected_cell = Some((row, col));
                            }
                        }
                        ui.end_row();
                    }
                });
            // Inside colour_math_egui_gui/src/colour_matrix_dashboard.rs UI loop update:

            ui.add_space(8.0);
            ui.separator();
            ui.add_space(6.0);

            // 2. Render our type-safe, separate R, G, B hex inputs side-by-side
            ui.horizontal(|ui| {
                ui.label("⚙️ Selected Matrix Cell Channels:");
                ui.add_space(8.0);

                // Feeds straight into our updated triple-field layout!
                colour_math_egui_lib::widgets::digital_readout::DigitalReadout::show(
                    ui,
                    &mut self.active_manipulator,
                );
            });
        });
    }
}
