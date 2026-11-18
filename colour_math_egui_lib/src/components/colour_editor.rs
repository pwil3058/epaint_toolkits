// Copyright (c) 2026 Peter Williams <pwil3058@bigpond.net.au> <pwil3058@gmail.com>.

use crate::components::colour_manipulator_pad::ColourManipulatorPad;
use crate::widgets::attribute_deck::AttributesDisplay;
use crate::widgets::rgb_hex_entry::EguiRGBHexEntry;
use colour_math_beigui::{ColourAttributeType, ColourManipulator};
use eframe::egui;

pub struct ColourEditor {
    pub active_rgb: colour_math::RGB<u8>,
    pub displayed_attributes: Vec<ColourAttributeType>,
    pub manipulator: ColourManipulator,
    pub rgb_hex_entry: EguiRGBHexEntry,
    pub manipulator_pad: ColourManipulatorPad,
    pub attribute_display_size: egui::Vec2,
}

impl ColourEditor {
    pub fn new(
        initial_model: ColourManipulator,
        attr_types: &[ColourAttributeType],
        attribute_display_size: egui::Vec2,
    ) -> Self {
        Self {
            active_rgb: initial_model.rgb(),
            displayed_attributes: attr_types.to_vec(),
            manipulator: initial_model,
            rgb_hex_entry: EguiRGBHexEntry::builder().build(),
            manipulator_pad: ColourManipulatorPad::new(),
            attribute_display_size,
        }
    }

    /// Renders the unified painter's workspace console layout on screen.
    pub fn show(&mut self, ui: &mut egui::Ui) {
        ui.with_layout(egui::Layout::top_down(egui::Align::Min), |ui| {
            ui.add_space(4.0);

            let display_deck =
                AttributesDisplay::new(&self.displayed_attributes, egui::Direction::TopDown);
            display_deck.show(
                ui,
                &self.manipulator.hcv(),
                None::<&colour_math::HCV>,
                self.attribute_display_size,
            );

            ui.add_space(14.0);
            ui.separator();
            ui.add_space(14.0);

            // ui.horizontal(|ui| {
            //     crate::widgets::rgb_hex_editor::RgbHexEditor::show(ui, &mut self.manipulator);
            // });
            if let Some(new_rgb) = self.rgb_hex_entry.show(ui, Some(self.manipulator.rgb())) {
                self.manipulator.set_colour(&new_rgb);
            }

            ui.add_space(14.0);
            ui.separator();
            ui.add_space(14.0);

            let mut texture_bridge: Option<egui::TextureHandle> = None;
            ui.vertical(|ui| {
                ui.set_width(340.0);
                self.manipulator_pad
                    .show(ui, &mut self.manipulator, &mut texture_bridge);
            });
        });
    }
}
