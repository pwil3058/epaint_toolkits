// Copyright (c) 2026 Peter Williams <pwil3058@bigpond.net.au> <pwil3058@gmail.com>.

use crate::egui_drawer::EguiDrawer;
use colour_math::{HueWheel, ScalarAttribute};
use eframe::egui;

pub struct ColourWheelPanel {
    pub hue_wheel: HueWheel,
    pub active_projection: ScalarAttribute,
    pub pan_offset: egui::Vec2,
}

impl ColourWheelPanel {
    pub fn new() -> Self {
        Self {
            hue_wheel: HueWheel::new(),
            active_projection: ScalarAttribute::Value,
            pan_offset: egui::Vec2::ZERO,
        }
    }

    /// Renders the color wheel canvas and maps spatial mouse tooltips driven entirely by backend metrics.
    pub fn show(&mut self, ui: &mut egui::Ui, side_dimension: f32) {
        ui.vertical(|ui| {
            // Radio Selector Group forcing projection updates across coordinate layers
            ui.horizontal(|ui| {
                ui.radio_value(&mut self.active_projection, ScalarAttribute::Value, "Value");
                ui.radio_value(
                    &mut self.active_projection,
                    ScalarAttribute::Chroma,
                    "Chroma",
                );
                ui.radio_value(
                    &mut self.active_projection,
                    ScalarAttribute::Warmth,
                    "Warmth",
                );

                if ui.button("⟲ Reset Pan").clicked() {
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

            let transformed_rect = resp.rect.translate(self.pan_offset);

            // 1. Instantiate your production EguiDrawer bridge context
            let drawer = EguiDrawer::new(&painter, transformed_rect, resp.rect);

            // 2. DRAW BASE WHEEL: Hand rendering control directly to your backend HueWheel state engine loop!
            self.hue_wheel.draw(self.active_projection, &drawer);

            // 🌟 3. REAL-TIME HOVER TOOLTIPS WITH EXPLICIT ACCURATE TRANSFORMS
            if let Some(mouse_pixel_pos) = ui.ctx().input(|i| i.pointer.hover_pos()) {
                if resp.rect.contains(mouse_pixel_pos) {
                    // Transpose screen pixels into your backend's expected centered Cartesian fixed-point grid space
                    let scale_factor = resp.rect.width() / 2.0;
                    let center = transformed_rect.center();

                    let abstract_x = (mouse_pixel_pos.x - center.x) / scale_factor;
                    let abstract_y = (center.y - mouse_pixel_pos.y) / scale_factor; // Invert Y axis natively

                    let backend_point = colour_math::Point {
                        x: (abstract_x as f64).into(),
                        y: (abstract_y as f64).into(),
                    };

                    // Execute hit testing with the correct backend argument sequence pass!
                    if let Some(tooltip_text) = self
                        .hue_wheel
                        .tooltip_for_point(backend_point, self.active_projection)
                    {
                        let unique_id = egui::Id::new("drawing_canvas_tooltip");

                        // 🌟 FIX: Use the publicly re-exported PopupAnchor path!
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
            }
        });
    }
}
