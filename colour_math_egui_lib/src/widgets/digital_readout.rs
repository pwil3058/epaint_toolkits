// Copyright (c) 2026 Peter Williams <pwil3058@bigpond.net.au> <pwil3058@gmail.com>.

use colour_math::ColourBasics;
use eframe::egui;

pub struct DigitalReadout;

impl DigitalReadout {
    pub fn show(ui: &mut egui::Ui, colour: &impl ColourBasics) {
        ui.horizontal(|ui| {
            let rgb_u16 = colour.rgb::<u16>();

            // 🌟 FIX: Access the elements via index notation [] instead of accessing the private inner array field .0
            ui.colored_label(egui::Color32::from_rgb(255, 80, 80), "Red:");
            ui.monospace(format!("0x{:04X}", rgb_u16[0]));
            ui.add_space(15.0);

            ui.colored_label(egui::Color32::from_rgb(80, 255, 80), "Green:");
            ui.monospace(format!("0x{:04X}", rgb_u16[1]));
            ui.add_space(15.0);

            ui.colored_label(egui::Color32::from_rgb(80, 80, 255), "Blue:");
            ui.monospace(format!("0x{:04X}", rgb_u16[2]));
        });
    }
}
