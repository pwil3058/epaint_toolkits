// Copyright (c) 2026 Peter Williams <pwil3058@bigpond.net.au> <pwil3058@gmail.com>.

use crate::widgets::hex_editor::HexEditor;
use colour_math::RGB;
use eframe::egui;

pub struct RgbHexEditor<'a> {
    // 🌟 FIXED: Consuming the full atomic RGB type directly!
    pub rgb: &'a mut RGB<u8>,
}

impl<'a> RgbHexEditor<'a> {
    pub fn new(rgb: &'a mut RGB<u8>) -> Self {
        Self { rgb }
    }

    pub fn show(self, ui: &mut egui::Ui) {
        ui.horizontal(|ui| {
            // Extract copies of current channels safely using your type-safe Index trait rules
            let mut r_val = self.rgb[0];
            let mut g_val = self.rgb[1];
            let mut blue_val = self.rgb[2];

            // Render independent HexEditor cells bound to our local frame buffers
            let r_resp = HexEditor::new(&mut r_val)
                .color(egui::Color32::from_rgb(255, 80, 80))
                .show(ui, "R: ");

            ui.add_space(10.0);

            let g_resp = HexEditor::new(&mut g_val)
                .color(egui::Color32::from_rgb(80, 255, 80))
                .show(ui, "G: ");

            ui.add_space(10.0);

            let b_resp = HexEditor::new(&mut blue_val)
                .color(egui::Color32::from_rgb(80, 80, 255))
                .show(ui, "B: ");

            // If any cell mutations are intercepted, re-assemble the atomic RGB container cleanly
            if r_resp.changed() || g_resp.changed() || b_resp.changed() {
                *self.rgb = RGB::from([r_val, g_val, blue_val]);
            }
        });
    }
}
