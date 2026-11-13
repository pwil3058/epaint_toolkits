// Copyright (c) 2026 Peter Williams <pwil3058@bigpond.net.au> <pwil3058@gmail.com>.

use colour_math::{ColourManipulator, IntoRGB};
use eframe::egui;

#[derive(Default)]
pub struct HexEditor;

impl HexEditor {
    pub fn show(ui: &mut egui::Ui, field: &mut u8, label: &str, label_colour: egui::Color32) {
        ui.horizontal(|ui| {
            let buffer_key = ui.id().with(label.to_owned() + "hex_buffer");
            let mut buffer = ui
                .ctx()
                .data_mut(|d| d.get_temp_mut_or_default::<String>(buffer_key).clone());

            if buffer.is_empty() {
                buffer = format!("0x{:02X}", field);
            }

            ui.colored_label(label_colour, label);
            let edit = egui::TextEdit::singleline(&mut buffer)
                .id(ui.id().with(label.to_owned() + "r_field"))
                .desired_width(45.0)
                .font(egui::FontId::monospace(14.0));
            let response = ui.add(edit);

            if response.changed() {
                ui.ctx()
                    .data_mut(|d| d.insert_temp(buffer_key, buffer.clone()));
            }

            if response.has_focus() {
                if ui.input(|i| i.key_pressed(egui::Key::ArrowUp)) {
                    if *field < u8::MAX {
                        *field = field.saturating_add(1);
                        buffer = format!("0x{:02X}", field);
                        ui.ctx()
                            .data_mut(|d| d.insert_temp(buffer_key, buffer.clone()));
                    } else {
                        print!("\x07");
                    }
                }
                if ui.input(|i| i.key_pressed(egui::Key::ArrowDown)) {
                    if *field > 0 {
                        *field = field.saturating_sub(1);
                        buffer = format!("0x{:02X}", field);
                        ui.ctx()
                            .data_mut(|d| d.insert_temp(buffer_key, buffer.clone()));
                    } else {
                        print!("\x07");
                    }
                }
            }

            if response.lost_focus() || ui.input(|i| i.key_pressed(egui::Key::Enter)) {
                if let Ok(parsed_r) =
                    u8::from_str_radix(buffer.trim_start_matches("0x").trim_start_matches("0X"), 16)
                {
                    *field = parsed_r;
                }
                buffer = format!("0x{:02X}", field);
                ui.ctx()
                    .data_mut(|d| d.insert_temp(buffer_key, buffer.clone()));
            }
        });
    }
}

pub struct RgbHexEditor;

impl RgbHexEditor {
    pub fn show(ui: &mut egui::Ui, manipulator: &mut ColourManipulator) {
        let current_hcv = manipulator.hcv();
        let rgb_u8 = current_hcv.into_rgb::<u8>();

        let mut edited_rgb = current_hcv.into_rgb::<u8>();

        for (i, (label, label_colour)) in [
            ("R: ", egui::Color32::RED),
            ("G: ", egui::Color32::GREEN),
            ("B: ", egui::Color32::BLUE),
        ]
        .into_iter()
        .enumerate()
        {
            HexEditor::show(ui, &mut edited_rgb[i], label, label_colour);
        }

        if rgb_u8 != edited_rgb {
            manipulator.set_colour(&edited_rgb)
        }
    }
}
