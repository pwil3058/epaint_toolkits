// Copyright (c) 2026 Peter Williams <pwil3058@bigpond.net.au> <pwil3058@gmail.com>.

use colour_math::{
    HCV, ScalarAttribute, beigui::attr_display::ColourAttributeType, hue_wheel::HueWheel,
    manipulator::ColourManipulator,
};
use colour_math_egui_lib::components::colour_editor::ColourEditor;
use colour_math_egui_lib::components::colour_table_view::ColourTableView;
use colour_math_egui_lib::egui_drawer::EguiDrawer;
use colour_math_egui_lib::widgets::attribute_deck::AttributeDeck;
use colour_math_egui_lib::widgets::digital_readout::DigitalReadout;
use colour_math_egui_lib::widgets::nudge_panel::NudgePanel;
use eframe::egui;

pub struct AppShell {
    pub table_view: ColourTableView,
    pub colour_editor: ColourEditor,
    pub hue_wheel: HueWheel,
    pub active_wheel_attribute: ScalarAttribute,

    // Persistent transformation trackers
    pub pan_offset: egui::Vec2,
    pub zoom_scale: f32,

    pub on_paste_automatic: bool,
    pub sample_texture: Option<egui::TextureHandle>,
    pub text_editor_buffer: String,
}

impl AppShell {
    pub fn new(_cc: &eframe::CreationContext<'_>, sliders: &[ColourAttributeType]) -> Self {
        let baseline_manipulator = ColourManipulator::builder().clamped(false).build();

        Self {
            table_view: ColourTableView::new(),
            colour_editor: ColourEditor::new(baseline_manipulator, sliders),
            hue_wheel: HueWheel::new(),
            active_wheel_attribute: ScalarAttribute::Value,
            pan_offset: egui::Vec2::ZERO,
            zoom_scale: 1.0,
            on_paste_automatic: false,
            sample_texture: None,
            text_editor_buffer: "🔬 Blending log book initialized...".to_string(),
        }
    }

    fn execute_take_sample(&mut self, ctx: &egui::Context) {
        let size = 128;
        let sample_color = egui::Color32::from_gray(160);
        let pixels = vec![sample_color; size * size];
        let color_image = egui::ColorImage::new([size, size], pixels);

        self.sample_texture =
            Some(ctx.load_texture("captured_sample", color_image, Default::default()));
    }
}

// 🌟 FIX: We switch AppShell back to a standard component layout pattern.
// This allows the top-level binary runner (main.rs) to handle the App trait panels natively!
impl AppShell {
    pub fn show(&mut self, ui: &mut egui::Ui) {
        let current_hcv = self.colour_editor.manipulator_view.model.hcv();

        egui::ScrollArea::vertical()
            .id_salt("console_scroll_viewport")
            .show(ui, |ui| {
                ui.add_space(4.0);
                ui.heading("🔬 Colour Workspace Console");
                ui.add_space(4.0);
                ui.separator();
                ui.add_space(4.0);

                // 1. Independent Take Sample Button Call
                ui.vertical_centered(|ui| {
                    if colour_math_egui_lib::widgets::take_sample_button::TakeSampleButton::show(ui)
                    {
                        self.execute_take_sample(ui.ctx());
                    }
                });
                ui.add_space(4.0);

                // 2. Attribute Sliders Deck
                AttributeDeck::show(
                    ui,
                    &self.colour_editor.displayed_attributes,
                    Some(&current_hcv),
                    None,
                );
                ui.add_space(8.0);

                // 3. Radio Selector & Interactive HueWheel Canvas
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

                    if resp.hovered() {
                        let scroll_delta = ui.ctx().input(|i| i.smooth_scroll_delta.y);
                        if scroll_delta != 0.0 {
                            self.zoom_scale =
                                (self.zoom_scale + scroll_delta * 0.005).clamp(1.0, 5.0);
                        }
                    }

                    if resp.dragged() {
                        self.pan_offset += resp.drag_delta();
                    }

                    if self.zoom_scale == 1.0 {
                        self.pan_offset = egui::Vec2::ZERO;
                    } else {
                        let max_pan = (self.zoom_scale - 1.0) * (wheel_side / 2.0);
                        self.pan_offset.x = self.pan_offset.x.clamp(-max_pan, max_pan);
                        self.pan_offset.y = self.pan_offset.y.clamp(-max_pan, max_pan);
                    }

                    let mut transformed_rect = resp.rect;
                    transformed_rect = transformed_rect.translate(self.pan_offset);
                    let current_center = transformed_rect.center();
                    transformed_rect = egui::Rect::from_center_size(
                        current_center,
                        transformed_rect.size() * self.zoom_scale,
                    );

                    let drawer = EguiDrawer::new(&painter, transformed_rect, resp.rect);
                    self.hue_wheel.draw(self.active_wheel_attribute, &drawer);
                });
                ui.add_space(8.0);

                // -----------------------------------------------------------------
                // CONSOLE BLOCK 4: Atomic Digital Readouts & Unified Manipulator Console
                // -----------------------------------------------------------------
                ui.group(|ui| {
                    // 🌟 FIX: Supply a mutable reference to the model engine controller
                    // instead of a read-only snapshot reference to active_rgb.
                    DigitalReadout::show(ui, &mut self.colour_editor.manipulator_view.model);
                    ui.add_space(6.0);

                    self.colour_editor.pad.show(
                        ui,
                        &mut self.colour_editor.manipulator_view.model,
                        &mut self.sample_texture,
                    );
                });
            });
    }
}
