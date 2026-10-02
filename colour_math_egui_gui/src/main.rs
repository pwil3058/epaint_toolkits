// Copyright (c) 2026 Peter Williams <pwil3058@bigpond.net.au> <pwil3058@gmail.com>.

use colour_math_egui_lib::colour_matrix_dashboard::ColorMatrixDashboard;
use colour_math_egui_lib::eframe;
use eframe::egui;

fn main() -> Result<(), eframe::Error> {
    // Basic runtime options matching your original GTK request specs
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_title("Colour Math Assistant (egui)")
            .with_inner_size([1100.0, 650.0]),
        ..Default::default()
    };

    eframe::run_native(
        "colour.math.assistant",
        options,
        Box::new(|_cc| {
            // Spin up your immediate-mode state container deck
            Ok(Box::new(ColorMatrixDashboard::new()))
        }),
    )
}
