// Copyright (c) 2026 Peter Williams <pwil3058@bigpond.net.au> <pwil3058@gmail.com>.

use colour_math_beigui::HexEntry;
use eframe::egui;

use crate::colour::Dedans;

pub trait ShowHexEntry {
    fn show(&self, ui: &mut egui::Ui, initial_value: Option<u8>) -> Option<u8>;
}

impl ShowHexEntry for HexEntry<u8> {
    fn show(&self, ui: &mut egui::Ui, initial_value: Option<u8>) -> Option<u8> {
        let initial_value = if let Some(initial_value) = initial_value {
            self.set_value(initial_value);
            initial_value
        } else {
            self.value()
        };
        let prefix: &'static str = if let Some(coloured_label) = self.label {
            coloured_label.label
        } else {
            "whatever" //TODO: do something random here
        };
        ui.horizontal(move |ui| {
            let buffer_key = ui.id().with(prefix.to_owned() + "hex_buffer");
            let mut buffer = ui
                .ctx()
                .data_mut(|d| d.get_temp_mut_or_default::<String>(buffer_key).clone());

            if buffer.is_empty() {
                buffer = format!("0x{:02X}", self.value());
            }

            if let Some(coloured_label) = self.label {
                let (label, colour) = coloured_label.dedans();
                ui.colored_label(colour, label);
            }

            let edit = egui::TextEdit::singleline(&mut buffer)
                .id(ui.id().with(prefix.to_owned() + "field"))
                .desired_width(45.0)
                .font(egui::FontId::monospace(14.0));
            let response = ui.add(edit);

            if response.changed() {
                ui.ctx()
                    .data_mut(|d| d.insert_temp(buffer_key, buffer.clone()));
            }

            if response.has_focus() {
                if ui.input(|i| i.key_pressed(egui::Key::ArrowUp)) {
                    if let Some(value) = self.incr_value() {
                        buffer = format!("0x{:02X}", value);
                        ui.ctx()
                            .data_mut(|d| d.insert_temp(buffer_key, buffer.clone()));
                    } else {
                        print!("\x07");
                    }
                }
                if ui.input(|i| i.key_pressed(egui::Key::ArrowDown)) {
                    if let Some(value) = self.decr_value() {
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
                if let Ok(result) = self.set_value_from_text(trimmed)
                    && let Some(value) = result
                {
                    buffer = format!("0x{:02X}", value);
                    ui.ctx()
                        .data_mut(|d| d.insert_temp(buffer_key, buffer.clone()));
                }
            }
        });

        let new_value = self.value();
        if new_value == initial_value {
            None
        } else {
            Some(new_value)
        }
    }
}
