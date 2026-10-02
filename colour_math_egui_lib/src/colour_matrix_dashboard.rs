// Copyright (c) 2026 Peter Williams <pwil3058@bigpond.net.au> <pwil3058@gmail.com>.

use crate::egui_drawer::EguiDrawer;
use colour_math::{
    HCV, ScalarAttribute, beigui::attr_display::ColourAttributeDisplay, hue_wheel::HueWheel,
};
use eframe::egui;

pub struct ColorMatrixDashboard {
    pub hue_wheel: HueWheel,
    pub active_attribute: ScalarAttribute,
    pub selected_item_id: Option<String>,
    pub current_colour: Option<HCV>,
    pub target_colour: Option<HCV>,

    // 🆕 1. Local viewport pan & zoom state properties
    pub pan_offset: egui::Vec2,
    pub zoom_scale: f32,
}

impl ColorMatrixDashboard {
    pub fn new() -> Self {
        Self {
            hue_wheel: HueWheel::new(),
            active_attribute: ScalarAttribute::Value,
            selected_item_id: None,
            current_colour: None,
            target_colour: None,
            pan_offset: egui::Vec2::ZERO,
            zoom_scale: 1.0,
        }
    }
}

impl eframe::App for ColorMatrixDashboard {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        // 🌟 FIX: Wrap your layout inside the CentralPanel container so the engine can drive 'ui' references!
        ui.heading("Color Science Analysis Control Deck");
        ui.separator();

        ui.horizontal(|ui| {
            // =========================================================================
            // LEFT COLUMN: Interactive Vector Wheel Viewport Canvas Space
            // =========================================================================
            ui.vertical(|ui| {
                ui.horizontal(|ui| {
                    ui.label("Active Matrix Dimension:");
                    ui.selectable_value(
                        &mut self.active_attribute,
                        ScalarAttribute::Value,
                        "Value",
                    );
                    ui.selectable_value(
                        &mut self.active_attribute,
                        ScalarAttribute::Chroma,
                        "Chroma",
                    );
                    ui.selectable_value(
                        &mut self.active_attribute,
                        ScalarAttribute::Warmth,
                        "Warmth",
                    );
                    ui.selectable_value(
                        &mut self.active_attribute,
                        ScalarAttribute::Greyness,
                        "Greyness",
                    );
                });

                ui.add_space(5.0);

                // Allocate our drawing layout canvas square
                let (response, painter) =
                    ui.allocate_painter(egui::vec2(500.0, 500.0), egui::Sense::click_and_drag());

                // Initialize our custom EguiDrawer mapping coordinate layers
                let drawer = EguiDrawer::new(&painter, response.rect);

                // Paint our vector rings and primary primaries/secondaries spokes
                self.hue_wheel.draw(self.active_attribute, &drawer);

                // Evaluate mouse pointer coordinates when hovering over the wheel canvas
                if let Some(hover_pos) = response.hover_pos() {
                    let center = response.rect.center();
                    let scale = drawer.scale;

                    // Convert screen absolute pixels back to Cartesian domain coordinates [-1.0..=1.0]
                    let user_x = (hover_pos.x - center.x) as f64 / scale;
                    let user_y = -(hover_pos.y - center.y) as f64 / scale; // Invert Y to match Cartesian
                    let target_point = colour_math::beigui::Point {
                        x: user_x.into(),
                        y: user_y.into(),
                    };

                    // Extract tooltips if hovering precisely over a plotted pigment shape
                    if let Some(tooltip_text) = self
                        .hue_wheel
                        .tooltip_for_point(target_point, self.active_attribute)
                    {
                        ui.output_mut(|o| {
                            o.commands
                                .push(egui::output::OutputCommand::CopyText(tooltip_text.clone()));
                        });

                        // Show modern, type-safe floating tooltip frame card
                        egui::containers::Tooltip::for_widget(&response).show(|ui| {
                            ui.label(tooltip_text);
                        });
                    }

                    // Cache the intersected shape ID on hover so the context menu knows what it's targeting
                    if let Some(shape) = self
                        .hue_wheel
                        .item_at_point(target_point, self.active_attribute)
                    {
                        self.selected_item_id = Some(shape.id().to_string());
                    }
                }

                // Native context menu attachment handler (Triggers automatically on Right-Click!)
                response.context_menu(|ui| {
                    ui.set_min_width(160.0);
                    if let Some(ref id) = self.selected_item_id {
                        ui.label(format!("Shape Target: {}", id));
                        ui.separator();
                        if ui.button("✨ Select as Target Color").clicked() {
                            // Close the dropdown overlay window instantly
                            ui.close();
                        }
                    } else {
                        ui.label("No vector element targeted");
                    }
                });
            });

            ui.separator();

            // =========================================================================
            // RIGHT COLUMN: Linear Gradients Control Stack Panel
            // =========================================================================
            ui.vertical(|ui| {
                ui.heading("Dimension Gradients View");
                ui.separator();

                // Dynamically iterate and draw all ColourAttributeDisplay components back-to-back
                for attr_type in &[
                    colour_math::beigui::attr_display::ColourAttributeType::Hue,
                    colour_math::beigui::attr_display::ColourAttributeType::Chroma,
                    colour_math::beigui::attr_display::ColourAttributeType::Value,
                    colour_math::beigui::attr_display::ColourAttributeType::Warmth,
                    colour_math::beigui::attr_display::ColourAttributeType::Greyness,
                ] {
                    let mut cad = ColourAttributeDisplay::new(attr_type);
                    cad.set_colour(self.current_colour.as_ref());
                    cad.set_target_colour(self.target_colour.as_ref());

                    // Allocate horizontal strip sections matching original requests
                    let (cad_resp, cad_painter) = ui.allocate_painter(
                        egui::vec2(ui.available_width(), 35.0),
                        egui::Sense::hover(),
                    );

                    let cad_drawer = EguiDrawer::new(&cad_painter, cad_resp.rect);
                    cad.draw_all(&cad_drawer);
                    ui.add_space(2.0);
                }
            });
        });
    }
}
