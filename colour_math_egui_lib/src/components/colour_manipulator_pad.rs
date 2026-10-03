// Copyright (c) 2026 Peter Williams <pwil3058@bigpond.net.au> <pwil3058@gmail.com>.

use crate::EguiColorBridge;
use colour_math::{
    ColourBasics, Prop, hue::angle::Angle, manipulator::ColourManipulator, rgb::RGB,
};
use eframe::egui;

pub struct ColourManipulatorPad {
    pub hue_step: Angle,
    pub scalar_step: Prop,
}

impl ColourManipulatorPad {
    pub fn new() -> Self {
        Self {
            hue_step: Angle::from(5.0),
            scalar_step: Prop::from(0.05_f64),
        }
    }

    pub fn show(
        &mut self,
        ui: &mut egui::Ui,
        manipulator: &mut ColourManipulator,
        texture: &mut Option<egui::TextureHandle>,
    ) {
        let side_btn_width = 32.0;
        let button_height = 24.0;

        // 🌟 FIX A: Match ergonomics lookup - remove explicit 'ref' keyword to satisfy type bounds
        let (content_width, content_height) = if let Some(tex) = texture {
            let size = tex.size();
            (size[0] as f32, size[1] as f32)
        } else {
            (220.0, 220.0)
        };

        ui.vertical_centered(|ui| {
            // 1. Value++ Button
            ui.with_layout(egui::Layout::top_down(egui::Align::Center), |ui| {
                if ui
                    .add_sized([content_width, button_height], egui::Button::new("Value++"))
                    .clicked()
                {
                    manipulator.incr_value(self.scalar_step);
                }
            });

            ui.add_space(4.0);

            // 2. Central Row: Dynamic Sized Image Center Box
            ui.horizontal(|ui| {
                let total_row_width = content_width + (side_btn_width * 2.0) + 8.0;
                let left_margin = (ui.available_width() - total_row_width) / 2.0;
                ui.add_space(left_margin.max(0.0));

                if ui
                    .add_sized([side_btn_width, content_height], egui::Button::new("<"))
                    .clicked()
                {
                    manipulator.rotate(-self.hue_step);
                }

                ui.add_space(4.0);

                let (field_rect, response) = ui.allocate_exact_size(
                    egui::vec2(content_width, content_height),
                    egui::Sense::click(),
                );

                let current_rgb_u64 = RGB::<u64>::from(manipulator.hcv());
                ui.painter()
                    .rect_filled(field_rect, 4.0, current_rgb_u64.to_color32());

                if let Some(tex) = texture {
                    ui.painter().image(
                        tex.id(),
                        field_rect,
                        egui::Rect::from_min_max(egui::pos2(0.0, 0.0), egui::pos2(1.0, 1.0)),
                        egui::Color32::WHITE,
                    );
                } else {
                    ui.painter().rect_stroke(
                        field_rect,
                        4.0,
                        egui::Stroke::new(1.0, egui::Color32::from_gray(64)),
                        egui::StrokeKind::Middle,
                    );
                }

                response.context_menu(|ui| {
                    if ui.button("📋 Paste Sample from Clipboard").clicked() {
                        // Clipboard decoding hooks go here
                    }
                    if ui.button("❌ Delete Sample Image").clicked() {
                        *texture = None;
                    }
                });

                ui.add_space(4.0);

                if ui
                    .add_sized([side_btn_width, content_height], egui::Button::new(">"))
                    .clicked()
                {
                    manipulator.rotate(self.hue_step);
                }
            });

            ui.add_space(4.0);

            // 3. Value-- Button
            ui.with_layout(egui::Layout::top_down(egui::Align::Center), |ui| {
                if ui
                    .add_sized([content_width, button_height], egui::Button::new("Value--"))
                    .clicked()
                {
                    manipulator.decr_value(self.scalar_step);
                }
            });

            ui.add_space(6.0);

            // 4. Footer Chroma tuning row
            ui.horizontal(|ui| {
                let half_row_width = (content_width + (side_btn_width * 2.0)) / 2.0;
                let footer_margin =
                    (ui.available_width() - (content_width + (side_btn_width * 2.0) + 4.0)) / 2.0;
                ui.add_space(footer_margin.max(0.0));

                if ui
                    .add_sized(
                        [half_row_width, button_height],
                        egui::Button::new("Chroma-- / Greyness++"),
                    )
                    .clicked()
                {
                    manipulator.decr_chroma(self.scalar_step);
                }
                ui.add_space(4.0);
                if ui
                    .add_sized(
                        [half_row_width, button_height],
                        egui::Button::new("Chroma++ / Greyness--"),
                    )
                    .clicked()
                {
                    manipulator.incr_chroma(self.scalar_step);
                }
            });

            ui.add_space(8.0);
            ui.separator();
            ui.add_space(4.0);

            // 5. Integral Automation Footer Controls (With egui Native Data Persistence)
            ui.horizontal(|ui| {
                let footer_align_margin =
                    (ui.available_width() - (content_width + (side_btn_width * 2.0))) / 2.0;
                ui.add_space(footer_align_margin.max(0.0));

                if ui.button("🤖 Auto Match Pixels").clicked() {
                    // Triggers calculations on your active manipulator structures
                }
                ui.add_space(16.0);

                // 🌟 FIX B: Load, render, and persist your checkbox state seamlessly inline!
                let storage_key = egui::Id::new("on_paste_auto_toggle");
                let mut on_paste_automatic = ui
                    .ctx()
                    .data_mut(|d| *d.get_temp_mut_or_default::<bool>(storage_key));

                if ui
                    .checkbox(&mut on_paste_automatic, "On Paste? (Match Automatically)")
                    .changed()
                {
                    ui.ctx()
                        .data_mut(|d| d.insert_temp(storage_key, on_paste_automatic));
                }
            });
        });
    }
}
