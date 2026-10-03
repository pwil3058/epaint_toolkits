// Copyright (c) 2026 Peter Williams <pwil3058@bigpond.net.au> <pwil3058@gmail.com>.

use colour_math_egui_lib::app_shell::ColorMathAppShell;
use eframe::egui;

fn main() -> Result<(), eframe::Error> {
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
            // 🌟 FIX: Change this to build the AppShell instead of the standalone wheel!
            Ok(Box::new(ColorMathAppShell::new()))
        }),
    )
}
