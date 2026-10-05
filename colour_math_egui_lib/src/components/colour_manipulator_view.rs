// Copyright (c) 2026 Peter Williams <pwil3058@bigpond.net.au> <pwil3058@gmail.com>.

use colour_math::{
    ColourBasics, ScalarAttribute, beigui::hue_wheel::HueWheel, manipulator::ColourManipulator,
};
use eframe::egui;
// 🌟 FIX: Pull in your native, production-ready drawing bridge struct directly!
use crate::egui_drawer::EguiDrawer;

pub struct ColourManipulatorView {
    pub model: ColourManipulator,
    pub hue_wheel: HueWheel,
    pub active_projection: ScalarAttribute,
    // Persistent zoom tracking state variable
    pub zoom_scale: f32,
    pub pan_offset: egui::Vec2,
}

impl ColourManipulatorView {
    pub fn new(initial_model: ColourManipulator) -> Self {
        Self {
            model: initial_model,
            hue_wheel: HueWheel::new(),
            active_projection: ScalarAttribute::Value,
            zoom_scale: 1.0,
            pan_offset: egui::Vec2::ZERO,
        }
    }

    /// Renders the visual workbench by delegating ALL layout calculations
    /// straight to your backend independent color rules using your EguiDrawer.
    pub fn show(&mut self, ui: &mut egui::Ui, dimensions: egui::Vec2) {
        ui.allocate_ui(dimensions, |ui| {
            ui.vertical(|ui| {
                // 1. Selector Row driving your projection attributes
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

                    if ui.button("⟲ Reset").clicked() {
                        self.zoom_scale = 1.0;
                        self.pan_offset = egui::Vec2::ZERO;
                    }
                });
                ui.add_space(4.0);

                // 2. Allocate an egui Painter canvas region matching your requested dimensions
                let wheel_side = dimensions.x.min(220.0);
                let (resp, painter) = ui.allocate_painter(
                    egui::vec2(wheel_side, wheel_side),
                    egui::Sense::click_and_drag(),
                );

                // Sync your backend wheel target context to match our active manipulator states
                self.hue_wheel.set_target_colour(Some(&self.model.hcv()));

                // 3. Handle Zoom wheel inputs natively via your backend methods
                if resp.hovered() {
                    let scroll_delta = ui.ctx().input(|i| i.smooth_scroll_delta.y);
                    if scroll_delta > 0.0 {
                        self.hue_wheel.incr_zoom();
                        self.zoom_scale = (self.zoom_scale + 0.05).clamp(1.0, 5.0);
                        ui.ctx().request_repaint();
                    } else if scroll_delta < 0.0 {
                        self.hue_wheel.decr_zoom();
                        self.zoom_scale = (self.zoom_scale - 0.05).clamp(1.0, 5.0);
                        ui.ctx().request_repaint();
                    }
                }

                if resp.dragged() {
                    self.pan_offset += resp.drag_delta();
                    ui.ctx().request_repaint();
                }

                // 4. Calculate spatial transformation matrices for the viewport drawing channel
                let mut transformed_rect = resp.rect;
                transformed_rect = transformed_rect.translate(self.pan_offset);
                let current_center = transformed_rect.center();
                transformed_rect = egui::Rect::from_center_size(
                    current_center,
                    transformed_rect.size() * self.zoom_scale,
                );

                // 🌟 5. DRAW: Instantiate your production EguiDrawer struct natively!
                // We pass your exact constructor arguments: painter, transformed_rect, and base canvas_rect.
                let drawer = EguiDrawer::new(&painter, transformed_rect, resp.rect);

                // Your backend executes its own graticule lines, spokes, and item layouts completely independently!
                self.hue_wheel.draw(self.active_projection, &drawer);
            });
        });
    }
}
