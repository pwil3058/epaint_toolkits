// Copyright (c) 2026 Peter Williams <pwil3058@bigpond.net.au> <pwil3058@gmail.com>.

use eframe::egui;
use colour_math::{
    hue_wheel::HueWheel,
    beigui::attr_display::ColourAttributeDisplay,
    ScalarAttribute, HCV,
};
use crate::egui_drawer::EguiDrawer;

pub struct ColorMatrixDashboard {
    pub hue_wheel: HueWheel,
    pub active_attribute: ScalarAttribute,
    pub selected_item_id: Option<String>,

    // Core Colour Science Parameters
    pub current_colour: Option<HCV>,
    pub target_colour: Option<HCV>,

    // Viewport persistence transformations state variables
    pub pan_offset: egui::Vec2,
    pub zoom_scale: f32,

    // Digital readouts and Automation flags
    pub rgb_red: u16,
    pub rgb_green: u16,
    pub rgb_blue: u16,
    pub on_paste: bool,

    // Retained sample pixel memory structure
    pub sample_texture: Option<egui::TextureHandle>,
}

impl ColorMatrixDashboard {
    pub fn new() -> Self {
        Self {
            hue_wheel: HueWheel::new(),
            active_attribute: ScalarAttribute::Value,
            selected_item_id: None,
            current_colour: Some(HCV::default()), // Seed default tracking states
            target_colour: Some(HCV::default()),
            pan_offset: egui::Vec2::ZERO,
            zoom_scale: 1.0,
            rgb_red: 0x0000,
            rgb_green: 0x0000,
            rgb_blue: 0x0000,
            on_paste: false,
            sample_texture: None,
        }
    }

    /// Helper execution routine simulating screen pixel matrix extraction
    fn trigger_take_sample(&mut self, ctx: &egui::Context) {
        // In your production pipeline, this hooks straight into an external screen capture pass.
        // For our UI test harness, we seed a clean color matrix bitmap to verify layout tracking:
        let color_image = egui::ColorImage::new([128, 128], egui::Color32::from_rgb(110, 150, 200));
        let handle = ctx.load_texture("screen_sample", color_image, Default::default());
        self.sample_texture = Some(handle);

        if self.on_paste {
            self.execute_auto_match();
        }
    }

    /// Automatically sets the displayed color properties to the average of the sample
    fn execute_auto_match(&mut self) {
        // Trigger your colour_math matrix averaging calculations here!
        self.rgb_red = 0x4A00;
        self.rgb_green = 0x6100;
        self.rgb_blue = 0x8C00;
    }
}

impl ColorMatrixDashboard {
    /// Renders the complete, integrated manipulator console interface.
    pub fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {

        // ---------------------------------------------------------------------
        // SECTION 1: Top Control & Sample Execution Block
        // ---------------------------------------------------------------------
        ui.vertical_centered_horizontal(|ui| {
            if ui.button("Take Sample").clicked() {
                self.trigger_take_sample(ui.ctx());
            }
        });
        ui.add_space(4.0);

        // Top Gradient Indicators Stack (Displays Current Colour + Target Triangle Pointers)
        ui.vertical(|ui| {
            for attr_type in &[
                colour_math::beigui::attr_display::ColourAttributeType::Hue,
                colour_math::beigui::attr_display::ColourAttributeType::Value,
                colour_math::beigui::attr_display::ColourAttributeType::Chroma,
                colour_math::beigui::attr_display::ColourAttributeType::Greyness,
                colour_math::beigui::attr_display::ColourAttributeType::Warmth,
            ] {
                let mut cad = ColourAttributeDisplay::new(attr_type);
                cad.set_colour(self.current_colour.as_ref());
                cad.set_target_colour(self.target_colour.as_ref());

                let (cad_resp, cad_painter) = ui.allocate_painter(
                    egui::vec2(ui.available_width(), 22.0),
                    egui::Sense::hover()
                );
                let cad_drawer = EguiDrawer::new(&cad_painter, cad_resp.rect, cad_resp.rect);
                cad.draw_all(&cad_drawer);
                ui.add_space(2.0);
            }
        });

        ui.add_space(6.0);

        ui.horizontal(|ui| {
            // =========================================================================
            // LEFT INNER COLUMN: Core Vector Geometry Wheel Canvas
            // =========================================================================
            ui.vertical(|ui| {
                ui.horizontal(|ui| {
                    ui.label("Attribute:");
                    ui.radio_value(&mut self.active_attribute, ScalarAttribute::Value, "Value");
                    ui.radio_value(&mut self.active_attribute, ScalarAttribute::Chroma, "Chroma");
                    ui.radio_value(&mut self.active_attribute, ScalarAttribute::Greyness, "Greyness");
                    ui.radio_value(&mut self.active_attribute, ScalarAttribute::Warmth, "Warmth");

                    ui.add_space(10.0);
                    if ui.button("⟲ Reset Pan/Zoom").clicked() {
                        self.pan_offset = egui::Vec2::ZERO;
                        self.zoom_scale = 1.0;
                    }
                });

                ui.add_space(5.0);

                let (response, painter) = ui.allocate_painter(
                    egui::vec2(500.0, 500.0),
                    egui::Sense::click_and_drag()
                );

                if response.hovered() {
                    let scroll_delta = ui.ctx().input(|i| i.smooth_scroll_delta.y);
                    if scroll_delta != 0.0 {
                        self.zoom_scale = (self.zoom_scale + scroll_delta * 0.005).clamp(0.2, 5.0);
                    }
                }

                if response.dragged() {
                    self.pan_offset += response.drag_delta();
                }

                let mut transformed_rect = response.rect;
                transformed_rect = transformed_rect.translate(self.pan_offset);
                let center_point = transformed_rect.center();
                transformed_rect = egui::Rect::from_center_size(
                    center_point,
                    transformed_rect.size() * self.zoom_scale
                );

                let drawer = EguiDrawer::new(&painter, transformed_rect, response.rect);
                self.hue_wheel.draw(self.active_attribute, &drawer);

                if !response.dragged() {
                    if let Some(pointer_pos) = ui.ctx().input(|i| i.pointer.interact_pos()) {
                        if response.rect.contains(pointer_pos) {
                            let adjusted_scale = drawer.scale;
                            let user_x = (pointer_pos.x - center_point.x) as f64 / adjusted_scale;
                            let user_y = -(pointer_pos.y - center_point.y) as f64 / adjusted_scale;
                            let target_point = colour_math::beigui::Point { x: user_x.into(), y: user_y.into() };

                            if let Some(tooltip_text) = self
                                .hue_wheel
                                .tooltip_for_point(target_point, self.active_attribute)
                            {
                                ui.ctx().copy_text(tooltip_text.clone());
                                egui::containers::Tooltip::for_widget(&response).show(|ui| {
                                    ui.label(tooltip_text);
                                });
                            }

                            if response.clicked() {
                                if let Some(shape) = self.hue_wheel.item_at_point(target_point, self.active_attribute) {
                                    self.selected_item_id = Some(shape.id().to_string());
                                } else {
                                    self.selected_item_id = None;
                                }
                            }
                        }
                    }
                }

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
            // RIGHT INNER COLUMN: Secondary Verification Trackers
            // =========================================================================
            ui.vertical(|ui| {
                // Duplicate gradient set sitting directly below the active attributes choice
                for attr_type in &[
                    colour_math::beigui::attr_display::ColourAttributeType::Hue,
                    colour_math::beigui::attr_display::ColourAttributeType::Value,
                    colour_math::beigui::attr_display::ColourAttributeType::Chroma,
                    colour_math::beigui::attr_display::ColourAttributeType::Greyness,
                    colour_math::beigui::attr_display::ColourAttributeType::Warmth,
                ] {
                    ui.add_space(4.0);

                    // -----------------------------------------------------------------
                    // SECTION 3: Digital RGB Hex Readout Strip
                    // -----------------------------------------------------------------
                    ui.horizontal(|ui| {
                        ui.colored_label(egui::Color32::from_rgb(255, 80, 80), "Red:");
                        ui.label(format!("0x{:04X}", self.rgb_red));
                        ui.add_space(15.0);

                        ui.colored_label(egui::Color32::from_rgb(80, 255, 80), "Green:");
                        ui.label(format!("0x{:04X}", self.rgb_green));
                        ui.add_space(15.0);

                        ui.colored_label(egui::Color32::from_rgb(80, 80, 255), "Blue:");
                        ui.label(format!("0x{:04X}", self.rgb_blue));
                    });

                    ui.add_space(4.0);

                    // -----------------------------------------------------------------
                    // SECTION 4: Fine-Tuning Directional Workspace Console
                    // -----------------------------------------------------------------
                    ui.vertical_centered(|ui| {
                        if ui.button("Value++").clicked() {
                            // Hook your direct colour_math increment loops here
                        }

                        ui.horizontal(|ui| {
                            if ui.button("<").clicked() {}

                            // Central Canvas Field area rendering the captured workspace sample image
                            let (sample_rect, sample_painter) = ui.allocate_painter(
                                egui::vec2(ui.available_width() - 60.0, 240.0),
                                egui::Sense::hover()
                            );

                            // Fill a neutral gray comparison field background container matching your screenshot
                            sample_painter.rect_filled(sample_rect, 0.0, egui::Color32::from_gray(96));

                            // If a screen matrix texture exists, paint it centered at real scale
                            if let Some(ref texture) = self.sample_texture {
                                sample_painter.image(
                                    texture.id(),
                                    egui::Rect::from_center_size(sample_rect.center(), egui::vec2(128.0, 128.0)),
                                    egui::Rect::from_min_max(egui::pos2(0.0, 0.0), egui::pos2(1.0, 1.0)),
                                    egui::Color32::WHITE
                                );
                            }

                            if ui.button(">").clicked() {}
                        });

                        if ui.button("Value--").clicked() {}

                        ui.add_space(4.0);
                        ui.horizontal(|ui| {
                            if ui.button("Chroma-- / Greyness++").clicked() {}
                            ui.add_space(10.0);
                            if ui.button("Chroma++ / Greyness--").clicked() {}
                        });
                    });

                    ui.add_space(8.0);
                    ui.separator();

                    // -----------------------------------------------------------------
                    // SECTION 5: Footer Actions and Interaction Controls
                    // -----------------------------------------------------------------
                    ui.horizontal(|ui| {
                        if ui.button("Auto Match").clicked() {
                            self.execute_auto_match();
                        }
                        ui.add_space(20.0);
                        ui.checkbox(&mut self.on_paste, "On Paste?");
                    });
                });
            });
        }
    }
