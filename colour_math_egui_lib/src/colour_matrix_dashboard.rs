// Copyright (c) 2026 Peter Williams <pwil3058@bigpond.net.au> <pwil3058@gmail.com>.

use crate::egui_drawer::EguiDrawer;
use crate::widgets::attribute_deck::AttributeDeck;
use crate::widgets::digital_readout::DigitalReadout;
use crate::widgets::nudge_panel::NudgePanel;
use colour_math::{
    HCV, RGBConstants, ScalarAttribute, beigui::attr_display::ColourAttributeType,
    hue_wheel::HueWheel,
};
use eframe::egui;

pub struct ColorMatrixDashboard {
    pub hue_wheel: HueWheel,
    pub active_attribute: ScalarAttribute,
    pub selected_item_id: Option<String>,

    // Atomic Domain Models
    pub current_colour: Option<HCV>,
    pub target_colour: Option<HCV>,
    pub active_rgb: colour_math::rgb::RGB<u64>,

    // Layout configuration slice propagated down from the very top
    pub profile_attributes: Vec<ColourAttributeType>,

    // Viewport states and handles
    pub pan_offset: egui::Vec2,
    pub zoom_scale: f32,
    pub on_paste: bool,
    pub sample_texture: Option<egui::TextureHandle>,
}

impl ColorMatrixDashboard {
    pub fn new(attributes: &[ColourAttributeType]) -> Self {
        Self {
            hue_wheel: HueWheel::new(),
            active_attribute: ScalarAttribute::Value,
            selected_item_id: None,
            current_colour: Some(HCV::default()),
            target_colour: Some(HCV::default()),
            active_rgb: colour_math::rgb::RGB::<u64>::BLACK,
            profile_attributes: attributes.to_vec(),
            pan_offset: egui::Vec2::ZERO,
            zoom_scale: 1.0,
            on_paste: false,
            sample_texture: None,
        }
    }

    pub fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        // SECTION 1: Top Action Controls Bar
        ui.vertical_centered(|ui| {
            ui.horizontal_top(|ui| {
                if ui.button("Take Sample").clicked() {
                    let size = 128;
                    let pixels = vec![egui::Color32::from_gray(128); size * size];
                    let color_image = egui::ColorImage::new([size, size], pixels);
                    self.sample_texture = Some(ui.ctx().load_texture(
                        "sample",
                        color_image,
                        Default::default(),
                    ));
                }
            });
        });
        ui.add_space(4.0);

        // SECTION 2: Dynamic Modular Attribute Deck Widget
        AttributeDeck::show(
            ui,
            &self.profile_attributes,
            self.current_colour.as_ref(),
            self.target_colour.as_ref(),
        );
        ui.add_space(6.0);

        // SECTION 3: Main Side-by-Side Columns
        ui.horizontal(|ui| {
            // Left Column Layout: Wheel Geometry
            ui.vertical(|ui| {
                ui.horizontal(|ui| {
                    ui.radio_value(&mut self.active_attribute, ScalarAttribute::Value, "Value");
                    ui.radio_value(
                        &mut self.active_attribute,
                        ScalarAttribute::Chroma,
                        "Chroma",
                    );
                });
                ui.add_space(4.0);

                let (resp, painter) =
                    ui.allocate_painter(egui::vec2(500.0, 500.0), egui::Sense::click_and_drag());
                let drawer = EguiDrawer::new(&painter, resp.rect, resp.rect);
                self.hue_wheel.draw(self.active_attribute, &drawer);
            });

            ui.separator();

            // Right Column Layout: Atomic Metrics & Nudge Operations
            ui.vertical(|ui| {
                // Passes the solid u64 struct reference down atomically
                DigitalReadout::show(ui, &self.active_rgb);
                ui.add_space(6.0);

                NudgePanel::show(ui, self.sample_texture.as_ref());
            });
        });
    }
}
