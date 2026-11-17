// Copyright (c) 2026 Peter Williams <pwil3058@bigpond.net.au> <pwil3058@gmail.com>.

use std::cell::Cell;

use colour_math_beigui::HexEntry;
use eframe::egui;

use crate::colour::Dedans;

#[derive(Debug, Default)]
pub struct EguiHexEntry {
    pub changed_value: Cell<Option<u8>>,
    pub hex_entry: HexEntry<u8>,
}

impl EguiHexEntry {
    pub fn show(&self, ui: &mut egui::Ui) {
        let old_value = self.hex_entry.value();
        ui.horizontal(move |ui| {
            let buffer_key = ui.id().with("hex_buffer");
            let mut buffer = ui
                .ctx()
                .data_mut(|d| d.get_temp_mut_or_default::<String>(buffer_key).clone());

            if buffer.is_empty() {
                buffer = format!("0x{:02X}", self.hex_entry.value());
            }

            if let Some(coloured_label) = self.hex_entry.label {
                let (label, colour) = coloured_label.dedans();
                ui.colored_label(colour, label);
            }

            let edit = egui::TextEdit::singleline(&mut buffer)
                .id(ui.id().with("field"))
                .desired_width(45.0)
                .font(egui::FontId::monospace(14.0));
            let response = ui.add(edit);

            if response.changed() {
                ui.ctx()
                    .data_mut(|d| d.insert_temp(buffer_key, buffer.clone()));
            }

            if response.has_focus() {
                if ui.input(|i| i.key_pressed(egui::Key::ArrowUp)) {
                    if let Some(value) = self.hex_entry.incr_value() {
                        buffer = format!("0x{:02X}", value);
                        ui.ctx()
                            .data_mut(|d| d.insert_temp(buffer_key, buffer.clone()));
                    } else {
                        print!("\x07");
                    }
                }
                if ui.input(|i| i.key_pressed(egui::Key::ArrowDown)) {
                    if let Some(value) = self.hex_entry.decr_value() {
                        buffer = format!("0x{:02X}", value);
                        ui.ctx()
                            .data_mut(|d| d.insert_temp(buffer_key, buffer.clone()));
                    } else {
                        print!("\x07");
                    }
                }
            }

            if response.lost_focus() || ui.input(|i| i.key_pressed(egui::Key::Enter)) {
                let trimmed = buffer.trim_start_matches("0x").trim_start_matches("0X");
                if let Ok(result) = self.hex_entry.set_value_from_text(trimmed)
                    && let Some(value) = result
                {
                    buffer = format!("0x{:02X}", value);
                    ui.ctx()
                        .data_mut(|d| d.insert_temp(buffer_key, buffer.clone()));
                }
            }
        });
        let new_value = self.hex_entry.value();
        self.changed_value.set(if new_value == old_value {
            None
        } else {
            Some(new_value)
        });
    }
}
