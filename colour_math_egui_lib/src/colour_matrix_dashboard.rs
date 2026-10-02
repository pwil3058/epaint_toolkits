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

    // Viewport persistence transformations state variables
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

                    ui.separator();
                    if ui.button("⟲ Reset Pan/Zoom").clicked() {
                        self.pan_offset = egui::Vec2::ZERO;
                        self.zoom_scale = 1.0;
                        self.selected_item_id = None;
                    }
                });

                ui.add_space(5.0);

                // Allocate our drawing layout canvas square
                let (response, painter) =
                    ui.allocate_painter(egui::vec2(500.0, 500.0), egui::Sense::click_and_drag());

                // 1. CAPTURE VIEWPORT ZOOM GESTURES
                if response.hovered() {
                    let scroll_delta = ui.ctx().input(|i| i.smooth_scroll_delta.y);
                    if scroll_delta != 0.0 {
                        self.zoom_scale = (self.zoom_scale + scroll_delta * 0.005).clamp(0.2, 5.0);
                    }
                }

                // 2. CAPTURE CANVAS DRAG PANNING
                if response.dragged() {
                    self.pan_offset += response.drag_delta();
                }

                // 3. COMPUTE MATRIX TRANSFORMATIONS
                let mut transformed_rect = response.rect;

                // 🌟 FIX: Capture the returned translated rect layout variable natively!
                transformed_rect = transformed_rect.translate(self.pan_offset);

                let center_point = transformed_rect.center();
                transformed_rect = egui::Rect::from_center_size(
                    center_point,
                    transformed_rect.size() * self.zoom_scale,
                );

                // Initialize our custom EguiDrawer passing the moving transformed_rect AND stable container boundary
                let drawer = EguiDrawer::new(&painter, transformed_rect, response.rect);

                // Paint fixed square background and panned/zoomed graticule tracks
                self.hue_wheel.draw(self.active_attribute, &drawer);

                // 4. EXECUTE MOUSE INTERSECTIONS & TOOLTIPS (Only when NOT actively dragging)
                if !response.dragged() {
                    if let Some(pointer_pos) = ui.ctx().input(|i| i.pointer.interact_pos()) {
                        if response.rect.contains(pointer_pos) {
                            let adjusted_scale = drawer.scale;

                            // Convert screen absolute pixels back to Cartesian domain coordinates [-1.0..=1.0]
                            let user_x = (pointer_pos.x - center_point.x) as f64 / adjusted_scale;
                            let user_y = -(pointer_pos.y - center_point.y) as f64 / adjusted_scale;
                            let target_point = colour_math::beigui::Point {
                                x: user_x.into(),
                                y: user_y.into(),
                            };

                            // Extract tooltips seamlessly on hover
                            if let Some(tooltip_text) = self
                                .hue_wheel
                                .tooltip_for_point(target_point, self.active_attribute)
                            {
                                ui.ctx().copy_text(tooltip_text.clone());

                                egui::containers::Tooltip::for_widget(&response).show(|ui| {
                                    ui.label(tooltip_text);
                                });
                            }

                            // Intercept interactive left-click selections
                            if response.clicked() {
                                if let Some(shape) = self
                                    .hue_wheel
                                    .item_at_point(target_point, self.active_attribute)
                                {
                                    self.selected_item_id = Some(shape.id().to_string());
                                } else {
                                    self.selected_item_id = None;
                                }
                            }
                        }
                    }
                }

                // 5. STABLE PANEL VIEW
                let active_selection = self.selected_item_id.clone();
                if let Some(id) = active_selection {
                    egui::Window::new("✨ Pigment Selection Action")
                        .id(egui::Id::new("paint_selection_window"))
                        .movable(true)
                        .collapsible(false)
                        .resizable(false)
                        .default_pos(response.rect.left_bottom() + egui::vec2(10.0, -120.0))
                        .show(ui.ctx(), |ui| {
                            ui.label(format!("Target Identifier: {}", id));
                            ui.separator();
                            if ui.button("➕ Add Paint to Palette").clicked() {
                                self.selected_item_id = None;
                            }
                            if ui.button("❌ Clear Selection").clicked() {
                                self.selected_item_id = None;
                            }
                        });
                }
            });

            ui.separator();

            // =========================================================================
            // RIGHT COLUMN: Linear Gradients Control Stack Panel
            // =========================================================================
            ui.vertical(|ui| {
                ui.heading("Dimension Gradients View");
                ui.separator();

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

                    let (cad_resp, cad_painter) = ui.allocate_painter(
                        egui::vec2(ui.available_width(), 35.0),
                        egui::Sense::hover(),
                    );

                    let cad_drawer = EguiDrawer::new(&cad_painter, cad_resp.rect, cad_resp.rect);
                    cad.draw_all(&cad_drawer);
                    ui.add_space(2.0);
                }
            });
        });
    }
}
