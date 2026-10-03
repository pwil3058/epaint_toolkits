// Copyright (c) 2026 Peter Williams <pwil3058@bigpond.net.au> <pwil3058@gmail.com>.

use eframe::egui;
// 🌟 FIX: Bring the App trait explicitly into code scope so the compiler can recognize its member methods!
use eframe::App;

use crate::components::colour_editor::ColourEditor;
use colour_math::manipulator::ColourManipulator;

pub struct AppShell {
    // Our stateful composite colour editing panel workspace
    pub colour_editor: ColourEditor,
}

impl AppShell {
    pub fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        // Initialize a safe, valid default baseline engine model state
        let baseline_manipulator = ColourManipulator::builder().clamped(false).build();

        Self {
            colour_editor: ColourEditor::new(baseline_manipulator),
        }
    }
}

// Now that `App` is explicitly in scope, the toolchain will cleanly map your parameters
impl App for AppShell {
    /// Renders the integrated console workspace on every single desktop frame pass tick.
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Enforce a clean, standard dark theme base layout for high colour accuracy
        ctx.set_visuals(egui::Visuals::dark());

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.vertical(|ui| {
                ui.heading("🔬 colour_math Egui Integration Test Bench");
                ui.add_space(4.0);
                ui.separator();
                ui.add_space(8.0);

                // Render the standalone composite editor panel workspace
                self.colour_editor.show(ui);
            });
        });
    }
}
