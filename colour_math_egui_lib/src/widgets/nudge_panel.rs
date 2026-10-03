// Copyright (c) 2026 Peter Williams <pwil3058@bigpond.net.au> <pwil3058@gmail.com>.

use eframe::egui;

pub struct NudgePanel;

impl NudgePanel {
    pub fn show(ui: &mut egui::Ui, texture: Option<&egui::TextureHandle>) {
        ui.vertical_centered(|ui| {
            if ui.button("Value++").clicked() {
                // Hook to core colour_math loops
            }

            ui.add_space(4.0);
            ui.horizontal(|ui| {
                if ui.button("<").clicked() {}

                // 🌟 FIX: Named tuple elements accurately so response and painter are distinct types
                let (response, sample_painter) = ui.allocate_painter(
                    egui::vec2(ui.available_width() - 60.0, 240.0),
                    egui::Sense::hover(),
                );

                // Pass the real geometric rect region down into the filling method pass
                sample_painter.rect_filled(response.rect, 4.0, egui::Color32::from_gray(96));

                if let Some(ref tex) = texture {
                    sample_painter.image(
                        tex.id(),
                        egui::Rect::from_center_size(
                            response.rect.center(),
                            egui::vec2(128.0, 128.0),
                        ),
                        egui::Rect::from_min_max(egui::pos2(0.0, 0.0), egui::pos2(1.0, 1.0)),
                        egui::Color32::WHITE,
                    );
                }

                if ui.button(">").clicked() {}
            });
            ui.add_space(4.0);

            if ui.button("Value--").clicked() {}

            ui.add_space(6.0);
            ui.horizontal(|ui| {
                if ui.button("Chroma-- / Greyness++").clicked() {}
                ui.add_space(10.0);
                if ui.button("Chroma++ / Greyness--").clicked() {}
            });
        });
    }
}
