// Copyright (c) 2026 Peter Williams <pwil3058@bigpond.net.au> <pwil3058@gmail.com>.

use crate::drawer::Drawer;
use colour_math::{HueWheel, ScalarAttributeType};
use eframe::egui;

pub struct ColourWheelPanel {
    pub hue_wheel: HueWheel,
    pub active_projection: ScalarAttributeType,
    pub pan_offset: egui::Vec2,
}

impl ColourWheelPanel {
    pub fn new() -> Self {
        Self {
            hue_wheel: HueWheel::new(),
            active_projection: ScalarAttributeType::Value,
            pan_offset: egui::Vec2::ZERO,
        }
    }

    /// Renders the color wheel canvas and maps spatial mouse tooltips driven entirely by backend metrics.
    pub fn show(&mut self, ui: &mut egui::Ui, side_dimension: f32) {
        ui.vertical(|ui| {
            // Radio Selector Group forcing projection updates across coordinate layers
            ui.horizontal(|ui| {
                ui.radio_value(
                    &mut self.active_projection,
                    ScalarAttributeType::Value,
                    "Value",
                );
                ui.radio_value(
                    &mut self.active_projection,
                    ScalarAttributeType::Chroma,
                    "Chroma",
                );
                ui.radio_value(
                    &mut self.active_projection,
                    ScalarAttributeType::Warmth,
                    "Warmth",
                );

                ui.add_space(160.0);

                if ui.button("⟲ Recentre").clicked() {
                    self.pan_offset = egui::Vec2::ZERO;
                }
            });
            ui.add_space(4.0);

            // Allocate our square canvas interaction bounding footprint box
            let (resp, painter) = ui.allocate_painter(
                egui::vec2(side_dimension, side_dimension),
                egui::Sense::click_and_drag(),
            );

            // Handle Zoom wheel inputs natively via your backend methods
            if resp.hovered() {
                let scroll_delta = ui.ctx().input(|i| i.smooth_scroll_delta.y);
                if scroll_delta > 0.0 {
                    self.hue_wheel.incr_zoom();
                    ui.ctx().request_repaint();
                } else if scroll_delta < 0.0 {
                    self.hue_wheel.decr_zoom();
                    ui.ctx().request_repaint();
                }
            }

            if resp.dragged() {
                self.pan_offset += resp.drag_delta();
                ui.ctx().request_repaint();
            }

            let drawer = Drawer::new(painter, Some(self.pan_offset));

            self.hue_wheel.draw(self.active_projection, &drawer);

            if let Some(mouse_pixel_pos) = ui.ctx().input(|i| i.pointer.hover_pos())
                && resp.rect.contains(mouse_pixel_pos)
            {
                let backend_point = drawer.transform_pos2(mouse_pixel_pos);

                if let Some(tooltip_text) = self
                    .hue_wheel
                    .tooltip_for_point(backend_point, self.active_projection)
                {
                    let unique_id = egui::Id::new("drawing_canvas_tooltip");

                    egui::containers::Tooltip::always_open(
                        ui.ctx().clone(),
                        ui.layer_id(),
                        unique_id,
                        egui::PopupAnchor::Pointer,
                    )
                    .show(|ui| {
                        ui.label(tooltip_text);
                    });
                }
            }
        });
    }
}

impl Default for ColourWheelPanel {
    fn default() -> Self {
        Self::new()
    }
}
