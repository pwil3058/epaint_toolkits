// Copyright (c) 2026 Peter Williams <pwil3058@bigpond.net.au> <pwil3058@gmail.com>.

use eframe::egui;

pub struct HexEditor<'a> {
    pub value: &'a mut u8, // 🌟 Downscaled to u8 channel depth
    pub min: u8,
    pub max: u8,
    pub desired_width: f32,
    pub label_color: Option<egui::Color32>,
}

impl<'a> HexEditor<'a> {
    pub fn new(value: &'a mut u8) -> Self {
        Self {
            value,
            min: u8::MIN,
            max: u8::MAX,
            desired_width: 45.0,
            label_color: None,
        }
    }

    pub fn range(mut self, min: u8, max: u8) -> Self {
        self.min = min;
        self.max = max;
        self
    }

    pub fn color(mut self, color: egui::Color32) -> Self {
        self.label_color = Some(color);
        self
    }

    pub fn show(self, ui: &mut egui::Ui, label: &str) -> egui::Response {
        ui.horizontal(|ui| {
            if let Some(color) = self.label_color {
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
                        if *self.value < self.max {
                            *self.value = self.value.saturating_add(1);
                        }
                    } else if *self.value > self.min {
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
                    if parsed >= self.min && parsed <= self.max {
                        *self.value = parsed;
                    }
                }
            }

            response
        })
        .inner
    }
}
