// Complete refactor of colour_math_egui_lib/src/components/colour_manipulator_view.rs
// Copyright (c) 2026 Peter Williams <pwil3058@bigpond.net.au> <pwil3058@gmail.com>.

use colour_math::manipulator::ColourManipulator;
use eframe::egui;

pub struct ColourManipulatorView {
    pub model: ColourManipulator,
    pub sample_patches: Vec<egui::ColorImage>,
}

impl ColourManipulatorView {
    pub fn new(initial_model: ColourManipulator) -> Self {
        Self {
            model: initial_model,
            sample_patches: Vec::new(),
        }
    }

    pub fn show(&mut self, ui: &mut egui::Ui, dimensions: egui::Vec2) {
        // Render the vector sample patches container row cleanly
        if !self.sample_patches.is_empty() {
            ui.allocate_ui_with_layout(
                dimensions,
                egui::Layout::left_to_right(egui::Align::Center),
                |ui| {
                    // Render any accumulated swatch patches here as needed...
                },
            );
        }
    }
}
