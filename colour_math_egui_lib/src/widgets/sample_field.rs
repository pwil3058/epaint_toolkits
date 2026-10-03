// Copyright (c) 2026 Peter Williams <pwil3058@bigpond.net.au> <pwil3058@gmail.com>.

use eframe::egui;

pub struct SampleField<'a> {
    pub texture: Option<&'a egui::TextureHandle>,
    pub dimensions: egui::Vec2,
}

impl<'a> SampleField<'a> {
    pub fn new(texture: Option<&'a egui::TextureHandle>, dimensions: egui::Vec2) -> Self {
        Self {
            texture,
            dimensions,
        }
    }

    pub fn show(self, ui: &mut egui::Ui) -> egui::Response {
        let (rect, response) = ui.allocate_exact_size(self.dimensions, egui::Sense::hover());

        // Paint the baseline gray backdrop box panel container natively
        ui.painter()
            .rect_filled(rect, 0.0, egui::Color32::from_gray(96));

        if let Some(texture) = self.texture {
            ui.painter().image(
                texture.id(),
                egui::Rect::from_center_size(rect.center(), egui::vec2(128.0, 128.0)),
                egui::Rect::from_min_max(egui::pos2(0.0, 0.0), egui::pos2(1.0, 1.0)),
                egui::Color32::WHITE,
            );
        }

        response
    }
}
