// Copyright (c) 2026 Peter Williams <pwil3058@bigpond.net.au> <pwil3058@gmail.com>.

use crate::components::colour_manipulator_pad::ColourManipulatorPad;
use crate::widgets::attribute_deck::AttributeDeck;
use colour_math::{ColourAttributeType, ColourManipulator};
use eframe::egui;

pub struct ColourEditor {
    pub active_rgb: colour_math::RGB<u8>,
    pub displayed_attributes: Vec<ColourAttributeType>,
    pub manipulator: ColourManipulator,
    pub manipulator_pad: ColourManipulatorPad,
}

impl ColourEditor {
    pub fn new(initial_model: ColourManipulator, sliders: &[ColourAttributeType]) -> Self {
        Self {
            active_rgb: initial_model.rgb(),
            displayed_attributes: sliders.to_vec(),
            manipulator: initial_model,
            manipulator_pad: ColourManipulatorPad::new(),
        }
    }

    /// Renders the unified painter's workspace console layout on screen.
    pub fn show(&mut self, ui: &mut egui::Ui) {
        ui.with_layout(egui::Layout::top_down(egui::Align::Min), |ui| {
            ui.add_space(4.0);

            // 1. REGION A (TOP): Ordered Attribute Display Deck Strips
            let display_deck =
                AttributeDeck::new(&self.displayed_attributes, egui::Direction::TopDown);
            display_deck.show(ui, &self.manipulator.hcv(), None::<&colour_math::HCV>);

            ui.add_space(14.0);
            ui.separator();
            ui.add_space(14.0);

            // 2. REGION B (MIDDLE): Digital R, G, B Hex Entry Fields
            ui.horizontal(|ui| {
                crate::widgets::digital_readout::DigitalReadout::show(ui, &mut self.manipulator);
            });

            ui.add_space(14.0);
            ui.separator();
            ui.add_space(14.0);

            // 3. REGION C (BOTTOM): Isolated Manipulator Panel
            let mut texture_bridge: Option<egui::TextureHandle> = None;
            ui.vertical(|ui| {
                ui.set_width(340.0);
                // 🌟 Dropped the duplicate right-side field!
                // This locks the layout into a single, beautifully aligned vertical block.
                self.manipulator_pad
                    .show(ui, &mut self.manipulator, &mut texture_bridge);
            });
        });
    }
}
