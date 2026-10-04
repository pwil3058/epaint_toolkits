// Complete refactor of colour_math_egui_lib/src/widgets/digital_readout.rs
// Copyright (c) 2026 Peter Williams <pwil3058@bigpond.net.au> <pwil3058@gmail.com>.

use colour_math::{ColourBasics, manipulator::ColourManipulator, rgb::RGB};
use eframe::egui;

pub struct DigitalReadout;

impl DigitalReadout {
    pub fn show(ui: &mut egui::Ui, manipulator: &mut ColourManipulator) {
        let current_hcv = manipulator.hcv();
        let rgb_u8 = current_hcv.rgb::<u8>();

        let mut r_val = rgb_u8[0];
        let mut g_val = rgb_u8[1];
        let mut b_val = rgb_u8[2];

        ui.horizontal(|ui| {
            // -----------------------------------------------------------------
            // 🟥 FIELD 1: RED CHANNEL HEX EDITOR
            // -----------------------------------------------------------------
            let r_buffer_key = ui.id().with("r_hex_buffer");
            let mut r_buffer = ui
                .ctx()
                .data_mut(|d| d.get_temp_mut_or_default::<String>(r_buffer_key).clone());

            // 🌟 FIX: Only populate the buffer if it is uninitialized or out of sync
            if r_buffer.is_empty() {
                r_buffer = format!("0x{:02X}", r_val);
            }

            ui.colored_label(egui::Color32::from_rgb(255, 80, 80), "R: ");
            let r_edit = egui::TextEdit::singleline(&mut r_buffer)
                .id(ui.id().with("r_field"))
                .desired_width(45.0)
                .font(egui::FontId::monospace(14.0));
            let r_resp = ui.add(r_edit);

            if r_resp.changed() {
                ui.ctx()
                    .data_mut(|d| d.insert_temp(r_buffer_key, r_buffer.clone()));
            }

            if r_resp.has_focus() {
                if ui.input(|i| i.key_pressed(egui::Key::ArrowUp)) {
                    if r_val < u8::MAX {
                        r_val = r_val.saturating_add(1);
                        r_buffer = format!("0x{:02X}", r_val);
                        ui.ctx()
                            .data_mut(|d| d.insert_temp(r_buffer_key, r_buffer.clone()));
                    } else {
                        print!("\x07");
                    }
                }
                if ui.input(|i| i.key_pressed(egui::Key::ArrowDown)) {
                    if r_val > 0 {
                        r_val = r_val.saturating_sub(1);
                        r_buffer = format!("0x{:02X}", r_val);
                        ui.ctx()
                            .data_mut(|d| d.insert_temp(r_buffer_key, r_buffer.clone()));
                    } else {
                        print!("\x07");
                    }
                }
            }

            if r_resp.lost_focus() || ui.input(|i| i.key_pressed(egui::Key::Enter)) {
                if let Ok(parsed_r) = u8::from_str_radix(
                    r_buffer.trim_start_matches("0x").trim_start_matches("0X"),
                    16,
                ) {
                    r_val = parsed_r;
                }
                r_buffer = format!("0x{:02X}", r_val);
                ui.ctx()
                    .data_mut(|d| d.insert_temp(r_buffer_key, r_buffer.clone()));
            }

            ui.add_space(10.0);

            // -----------------------------------------------------------------
            // 🟩 FIELD 2: GREEN CHANNEL HEX EDITOR
            // -----------------------------------------------------------------
            let g_buffer_key = ui.id().with("g_hex_buffer");
            let mut g_buffer = ui
                .ctx()
                .data_mut(|d| d.get_temp_mut_or_default::<String>(g_buffer_key).clone());
            if g_buffer.is_empty() {
                g_buffer = format!("0x{:02X}", g_val);
            }

            ui.colored_label(egui::Color32::from_rgb(80, 255, 80), "G: ");
            let g_edit = egui::TextEdit::singleline(&mut g_buffer)
                .id(ui.id().with("g_field"))
                .desired_width(45.0)
                .font(egui::FontId::monospace(14.0));
            let g_resp = ui.add(g_edit);

            if g_resp.changed() {
                ui.ctx()
                    .data_mut(|d| d.insert_temp(g_buffer_key, g_buffer.clone()));
            }

            if g_resp.has_focus() {
                if ui.input(|i| i.key_pressed(egui::Key::ArrowUp)) {
                    if g_val < u8::MAX {
                        g_val = g_val.saturating_add(1);
                        g_buffer = format!("0x{:02X}", g_val);
                        ui.ctx()
                            .data_mut(|d| d.insert_temp(g_buffer_key, g_buffer.clone()));
                    } else {
                        print!("\x07");
                    }
                }
                if ui.input(|i| i.key_pressed(egui::Key::ArrowDown)) {
                    if g_val > 0 {
                        g_val = g_val.saturating_sub(1);
                        g_buffer = format!("0x{:02X}", g_val);
                        ui.ctx()
                            .data_mut(|d| d.insert_temp(g_buffer_key, g_buffer.clone()));
                    } else {
                        print!("\x07");
                    }
                }
            }

            if g_resp.lost_focus() || ui.input(|i| i.key_pressed(egui::Key::Enter)) {
                if let Ok(parsed_g) = u8::from_str_radix(
                    g_buffer.trim_start_matches("0x").trim_start_matches("0X"),
                    16,
                ) {
                    g_val = parsed_g;
                }
                g_buffer = format!("0x{:02X}", g_val);
                ui.ctx()
                    .data_mut(|d| d.insert_temp(g_buffer_key, g_buffer.clone()));
            }

            ui.add_space(10.0);

            // -----------------------------------------------------------------
            // 🟦 FIELD 3: BLUE CHANNEL HEX EDITOR
            // -----------------------------------------------------------------
            let b_buffer_key = ui.id().with("b_hex_buffer");
            let mut b_buffer = ui
                .ctx()
                .data_mut(|d| d.get_temp_mut_or_default::<String>(b_buffer_key).clone());
            if b_buffer.is_empty() {
                b_buffer = format!("0x{:02X}", b_val);
            }

            ui.colored_label(egui::Color32::from_rgb(80, 80, 255), "B: ");
            let b_edit = egui::TextEdit::singleline(&mut b_buffer)
                .id(ui.id().with("b_field"))
                .desired_width(45.0)
                .font(egui::FontId::monospace(14.0));
            let b_resp = ui.add(b_edit);

            if b_resp.changed() {
                ui.ctx()
                    .data_mut(|d| d.insert_temp(b_buffer_key, b_buffer.clone()));
            }

            if b_resp.has_focus() {
                if ui.input(|i| i.key_pressed(egui::Key::ArrowUp)) {
                    if b_val < u8::MAX {
                        b_val = b_val.saturating_add(1);
                        b_buffer = format!("0x{:02X}", b_val);
                        ui.ctx()
                            .data_mut(|d| d.insert_temp(b_buffer_key, b_buffer.clone()));
                    } else {
                        print!("\x07");
                    }
                }
                if ui.input(|i| i.key_pressed(egui::Key::ArrowDown)) {
                    if b_val > 0 {
                        b_val = b_val.saturating_sub(1);
                        b_buffer = format!("0x{:02X}", b_val);
                        ui.ctx()
                            .data_mut(|d| d.insert_temp(b_buffer_key, b_buffer.clone()));
                    } else {
                        print!("\x07");
                    }
                }
            }

            if b_resp.lost_focus() || ui.input(|i| i.key_pressed(egui::Key::Enter)) {
                if let Ok(parsed_b) = u8::from_str_radix(
                    b_buffer.trim_start_matches("0x").trim_start_matches("0X"),
                    16,
                ) {
                    b_val = parsed_b;
                }
                b_buffer = format!("0x{:02X}", b_val);
                ui.ctx()
                    .data_mut(|d| d.insert_temp(b_buffer_key, b_buffer.clone()));
            }

            // 🌟 RE-ASSEMBLE REFACTOR VIA ARRAY FROM POLICY
            if rgb_u8[0] != r_val || rgb_u8[1] != g_val || rgb_u8[2] != b_val {
                let updated_array = [r_val, g_val, b_val];
                let target_rgb = RGB::<u8>::from(updated_array);
                manipulator.set_colour(&target_rgb);
            }
        });
    }
}
