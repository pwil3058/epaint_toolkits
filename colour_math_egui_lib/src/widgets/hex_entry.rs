// Copyright (c) 2026 Peter Williams <pwil3058@bigpond.net.au> <pwil3058@gmail.com>.

use colour_math_beigui::hex_entry;
use eframe::egui;

#[derive(Debug, Default)]
pub struct HexEntry {
    hex_entry: hex_entry::HexEntry<u8>,
    label: Option<(&'static str, egui::Color32)>,
}

impl HexEntry {
    pub fn show(&self, ui: &mut egui::Ui) -> bool {
        let mut changed = false;
        ui.horizontal(move |ui| {
            let buffer_key = ui.id().with("hex_buffer");
            let mut buffer = ui
                .ctx()
                .data_mut(|d| d.get_temp_mut_or_default::<String>(buffer_key).clone());

            if buffer.is_empty() {
                buffer = format!("0x{:02X}", self.hex_entry.value());
            }

            if let Some((label, colour)) = self.label {
                ui.colored_label(colour, label);
            }

            let edit = egui::TextEdit::singleline(&mut buffer)
                .id(ui.id().with("field"))
                .desired_width(45.0)
                .font(egui::FontId::monospace(14.0));
            let response = ui.add(edit);

            if response.changed() {
                changed = true;
                ui.ctx()
                    .data_mut(|d| d.insert_temp(buffer_key, buffer.clone()));
            }

            if response.has_focus() {
                if ui.input(|i| i.key_pressed(egui::Key::ArrowUp)) {
                    if let Some(value) = self.hex_entry.incr_value() {
                        buffer = format!("0x{:02X}", value);
                        ui.ctx()
                            .data_mut(|d| d.insert_temp(buffer_key, buffer.clone()));
                        changed = true;
                    } else {
                        print!("\x07");
                    }
                }
                if ui.input(|i| i.key_pressed(egui::Key::ArrowDown)) {
                    if let Some(value) = self.hex_entry.decr_value() {
                        buffer = format!("0x{:02X}", value);
                        ui.ctx()
                            .data_mut(|d| d.insert_temp(buffer_key, buffer.clone()));
                        changed = true;
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
                    changed = true;
                }
            }
        });
        changed
    }
}
