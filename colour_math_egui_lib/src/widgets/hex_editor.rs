// Copyright (c) 2026 Peter Williams <pwil3058@bigpond.net.au> <pwil3058@gmail.com>.

use std::mem;

use eframe::egui;

pub struct HexEditor<'a> {
    pub value: &'a mut u8,
    pub desired_width: f32,
    pub colour: Option<egui::Color32>,
}

impl<'a> HexEditor<'a> {
    pub fn new(value: &'a mut u8) -> Self {
        Self {
            value,
            desired_width: 45.0,
            colour: None,
        }
    }

    pub fn colour(mut self, color: egui::Color32) -> Self {
        self.colour = Some(color);
        self
    }

    pub fn show(self, ui: &mut egui::Ui, label: &str) -> egui::Response {
        ui.horizontal(|ui| {
            if let Some(color) = self.colour {
                ui.colored_label(color, label);
            } else {
                ui.label(label);
            }

            // Maintain u8 string value buffers dynamically on the stack frame
            let mut buffer = format!("0x{:02X}", self.value);

            let text_edit = egui::TextEdit::singleline(&mut buffer)
                .desired_width(self.desired_width)
                .margin(egui::Margin::symmetric(4, 2));

            let response = ui.add(text_edit);

            if response.hovered() {
                let scroll_delta = ui.ctx().input(|i| i.smooth_scroll_delta.y);
                if scroll_delta != 0.0 {
                    if scroll_delta > 0.0 {
                        *self.value = self.value.saturating_add(1);
                    } else {
                        *self.value = self.value.saturating_sub(1);
                    }
                    ui.ctx().request_repaint();
                }
            }

            if response.changed() {
                let clean_text = buffer
                    .trim()
                    .trim_start_matches("0x")
                    .trim_start_matches("0X");

                if let Ok(parsed) = u8::from_str_radix(clean_text, 16) {
                    *self.value = parsed;
                }
            }

            response
        })
        .inner
    }
}
