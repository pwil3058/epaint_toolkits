// Copyright (c) 2026 Peter Williams <pwil3058@bigpond.net.au> <pwil3058@gmail.com>.

use colour_math::RGB;
use colour_math_beigui::RGBHexEntry;

use crate::widgets::hex_entry::*;

pub trait ShowRGBHexEntry {
    fn show(&self, ui: &mut egui::Ui, initial_rgb: Option<RGB<u8>>) -> Option<RGB<u8>>;
}

impl ShowRGBHexEntry for RGBHexEntry {
    fn show(&self, ui: &mut egui::Ui, initial_rgb: Option<RGB<u8>>) -> Option<RGB<u8>> {
        let mut changed = false;
        ui.horizontal(|ui| {
            if let Some(initial_rgb) = initial_rgb {
                for (entry, value) in self.entries.iter().zip(initial_rgb) {
                    changed |= entry.show(ui, Some(value)).is_some()
                }
            } else {
                for entry in self.entries.iter() {
                    changed |= entry.show(ui, None).is_some()
                }
            }
        });
        if changed { Some(self.rgb()) } else { None }
    }
}
