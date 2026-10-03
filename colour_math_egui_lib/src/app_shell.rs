// Copyright (c) 2026 Peter Williams <pwil3058@bigpond.net.au> <pwil3058@gmail.com>.

use crate::components::colour_editor::ColourEditor;
use crate::egui_drawer::EguiDrawer;
use crate::paint_table_view::PaintTableView;
use crate::widgets::attribute_deck::AttributeDeck;
use crate::widgets::digital_readout::DigitalReadout;
use crate::widgets::nudge_panel::NudgePanel;
use colour_math::{
    HCV, ScalarAttribute, beigui::attr_display::ColourAttributeType, hue_wheel::HueWheel,
    manipulator::ColourManipulator,
};
use eframe::egui;

pub struct AppShell {
    pub table_view: PaintTableView,
    pub colour_editor: ColourEditor,
    pub hue_wheel: HueWheel,
    pub active_wheel_attribute: ScalarAttribute,

    // 🌟 FIX: Declare the missing persistent transformation state properties
    pub pan_offset: egui::Vec2,
    pub zoom_scale: f32,

    // Automation States matching your GTK Test specifications
    pub on_paste_automatic: bool,
    pub sample_texture: Option<egui::TextureHandle>,
    pub text_editor_buffer: String,
}

impl AppShell {
    pub fn new(_cc: &eframe::CreationContext<'_>, sliders: &[ColourAttributeType]) -> Self {
        let baseline_manipulator = ColourManipulator::builder().clamped(false).build();

        Self {
            table_view: PaintTableView::new(),
            colour_editor: ColourEditor::new(baseline_manipulator, sliders),
            hue_wheel: HueWheel::new(),
            active_wheel_attribute: ScalarAttribute::Value,

            // 🌟 FIX: Initialize default matrix transformation trackers on startup loop
            pan_offset: egui::Vec2::ZERO,
            zoom_scale: 1.0,

            on_paste_automatic: false,
            sample_texture: None,
            text_editor_buffer: "📝 Blending log book initialized...".to_string(),
        }
    }

    /// Captures a target region coordinates pass and saves it as a texture handle memory token
    fn execute_take_sample(&mut self, ctx: &egui::Context) {
        let size = 128;
        let sample_color = egui::Color32::from_gray(160);
        let pixels = vec![sample_color; size * size];
        let color_image = egui::ColorImage::new([size, size], pixels);

        self.sample_texture =
            Some(ctx.load_texture("captured_sample", color_image, Default::default()));

        if self.on_paste_automatic {
            self.execute_auto_match_routine();
        }
    }

    fn execute_auto_match_routine(&mut self) {
        // Query your colour_math engine mechanisms here to calculate average pixel states
    }

    /// Unified layout compositor organizing your simple widget building-blocks
    pub fn show(&mut self, ui: &mut egui::Ui) {
        let current_hcv = self.colour_editor.manipulator_view.model.hcv();

        // -----------------------------------------------------------------
        // CONSOLE BLOCK 1: Top Take Sample Actions Bar
        // -----------------------------------------------------------------
        ui.vertical_centered(|ui| {
            if ui.button("📸 Take Screen Sample").clicked() {
                self.execute_take_sample(ui.ctx());
            }
        });
        ui.add_space(4.0);

        // -----------------------------------------------------------------
        // CONSOLE BLOCK 2: Compile-Time Configured Attribute Sliders Deck
        // -----------------------------------------------------------------
        AttributeDeck::show(
            ui,
            &self.colour_editor.displayed_attributes,
            Some(&current_hcv),
            None,
        );
        ui.add_space(8.0);

        // -----------------------------------------------------------------
        // CONSOLE BLOCK 3: Radio Selector & Interactive HueWheel Color Canvas
        // -----------------------------------------------------------------
        ui.group(|ui| {
            ui.horizontal(|ui| {
                ui.label("🎯 Wheel Projection Attribute:");
                ui.radio_value(
                    &mut self.active_wheel_attribute,
                    ScalarAttribute::Value,
                    "Value",
                );
                ui.radio_value(
                    &mut self.active_wheel_attribute,
                    ScalarAttribute::Chroma,
                    "Chroma",
                );
                ui.radio_value(
                    &mut self.active_wheel_attribute,
                    ScalarAttribute::Greyness,
                    "Greyness",
                );
                ui.radio_value(
                    &mut self.active_wheel_attribute,
                    ScalarAttribute::Warmth,
                    "Warmth",
                );

                ui.add_space(16.0);
                if ui.button("⟲ Reset Pan/Zoom").clicked() {
                    self.pan_offset = egui::Vec2::ZERO;
                    self.zoom_scale = 1.0;
                }
            });
            ui.add_space(4.0);

            let wheel_side = ui.available_width().min(320.0);
            let (resp, painter) = ui.allocate_painter(
                egui::vec2(wheel_side, wheel_side),
                egui::Sense::click_and_drag(),
            );
            // 🌟 1. CAPTURE SCROLL WHEEL DELTAS (Zoom Interaction Loop)
            if resp.hovered() {
                let scroll_delta = ui.ctx().input(|i| i.smooth_scroll_delta.y);
                if scroll_delta != 0.0 {
                    // 🌟 FIX: Locked lower bound constraint to 1.0 instead of 0.2.
                    // This prevents the colour wheel from ever shrinking smaller than a perfect fit inside its frame box!
                    self.zoom_scale = (self.zoom_scale + scroll_delta * 0.005).clamp(1.0, 5.0);
                }
            }

            // 🌟 2. CAPTURE MOUSE DRAG DELTAS (Pan Interaction Loop)
            if resp.dragged() {
                self.pan_offset += resp.drag_delta();
            }

            // 🌟 3. ENFORCE INNER BOUNDARY CLAMPING WHEN ZOOM_SCALE IS AT MINIMUM
            if self.zoom_scale == 1.0 {
                // If the user isn't zoomed in, snap the pan offset back to dead center
                self.pan_offset = egui::Vec2::ZERO;
            } else {
                // Otherwise, clamp the pan vector to stop the user from dragging the wheel completely off the canvas view bounds
                let max_pan = (self.zoom_scale - 1.0) * (wheel_side / 2.0);
                self.pan_offset.x = self.pan_offset.x.clamp(-max_pan, max_pan);
                self.pan_offset.y = self.pan_offset.y.clamp(-max_pan, max_pan);
            }

            // Apply view persistent matrix transformations...
            let mut transformed_rect = resp.rect;
            transformed_rect = transformed_rect.translate(self.pan_offset);

            let current_center = transformed_rect.center();
            transformed_rect = egui::Rect::from_center_size(
                current_center,
                transformed_rect.size() * self.zoom_scale,
            );

            // Hand our calculated target region bounds securely down into the drawing engine
            let drawer = EguiDrawer::new(&painter, transformed_rect, resp.rect);
            self.hue_wheel.draw(self.active_wheel_attribute, &drawer);
        });
        ui.add_space(8.0);

        // -----------------------------------------------------------------
        // CONSOLE BLOCK 4: Atomic Digital Readouts, Nudge Pads & Workspace
        // -----------------------------------------------------------------
        ui.group(|ui| {
            DigitalReadout::show(ui, &self.colour_editor.active_rgb);
            ui.add_space(6.0);

            // 🌟 FIX: Supply a direct mutable reference handle instead of .as_ref()
            self.colour_editor.pad.show(
                ui,
                &mut self.colour_editor.manipulator_view.model,
                &mut self.sample_texture,
            );
        });
        ui.add_space(8.0);

        // -----------------------------------------------------------------
        // CONSOLE BLOCK 5: Automation Footer Controls
        // -----------------------------------------------------------------
        ui.horizontal(|ui| {
            if ui.button("🤖 Auto Match Pixels").clicked() {
                self.execute_auto_match_routine();
            }
            ui.add_space(24.0);
            ui.checkbox(
                &mut self.on_paste_automatic,
                "On Paste? (Match Automatically)",
            );
        });
    }
}
