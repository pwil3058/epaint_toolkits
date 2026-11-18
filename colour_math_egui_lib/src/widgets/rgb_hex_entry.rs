// Copyright (c) 2026 Peter Williams <pwil3058@bigpond.net.au> <pwil3058@gmail.com>.

use std::rc::Rc;

use colour_math::RGB;
use colour_math_beigui::RGBHexEntry;

use crate::widgets::hex_entry::*;

pub struct EguiRGBHexEntry {
    pub rgb_hex_entry: RGBHexEntry,
    pub egui_hex_entries: [EguiHexEntry; 3],
}

impl EguiRGBHexEntry {
    pub fn new(rgb_hex_entry: RGBHexEntry) -> Self {
        let egui_hex_entries = rgb_hex_entry
            .entries
            .iter()
            .map(|e| EguiHexEntry::new(Rc::clone(e)))
            .collect::<Vec<_>>()
            .try_into()
            .unwrap();
        Self {
            rgb_hex_entry,
            egui_hex_entries,
        }
    }

    pub fn builder() -> EguiRGBHexEntryBuilder {
        EguiRGBHexEntryBuilder::default()
    }

    pub fn show(&self, ui: &mut egui::Ui, initial_rgb: Option<RGB<u8>>) -> Option<RGB<u8>> {
        let mut changed = false;
        ui.horizontal(|ui| {
            if let Some(initial_rgb) = initial_rgb {
                for (entry, value) in self.egui_hex_entries.iter().zip(initial_rgb) {
                    changed |= entry.show(ui, Some(value)).is_some()
                }
            } else {
                for entry in self.egui_hex_entries.iter() {
                    changed |= entry.show(ui, None).is_some()
                }
            }
        });
        if changed {
            Some(self.rgb_hex_entry.rgb())
        } else {
            None
        }
    }
}

#[derive(Debug, Default)]
pub struct EguiRGBHexEntryBuilder {
    initial_rgb: Option<RGB<u8>>,
    max_step: Option<u8>,
}

impl EguiRGBHexEntryBuilder {
    pub fn build(&self) -> EguiRGBHexEntry {
        let mut rhe_builder = RGBHexEntry::builder();
        if let Some(max_step) = self.max_step {
            let _ = rhe_builder.max_step(max_step);
        }
        if let Some(initial_rgb) = self.initial_rgb {
            let _ = rhe_builder.initial_rgb(initial_rgb);
        }
        EguiRGBHexEntry::new(rhe_builder.build())
    }
}
