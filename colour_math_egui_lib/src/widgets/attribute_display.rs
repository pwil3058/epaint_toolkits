// Copyright (c) 2026 Peter Williams <pwil3058@bigpond.net.au> <pwil3058@gmail.com>.

use crate::drawer::Drawer;
use colour_math::Colour;
use colour_math_beigui::{
    ColourAttributeDisplay, ColourAttributeType, ColourAttributesDisplay, DrawAll,
};
use eframe::egui;

/// A stateless immediate-mode widget that renders a single color attribute
/// indicator bar strip driven entirely by backend display traits.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct AttributeDisplay;

impl AttributeDisplay {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn show(
        &self,
        ui: &mut egui::Ui,
        display_config: &ColourAttributeDisplay,
        size: egui::Vec2,
    ) -> egui::Response {
        let (response, painter) = ui.allocate_painter(size, egui::Sense::empty());
        let drawer = Drawer::new(painter, None);

        display_config.draw_all(&drawer);

        response
    }
}

pub trait ShowAttributeDisplay: DrawAll {
    fn show(&self, ui: &mut egui::Ui, size: egui::Vec2) -> egui::Response {
        let (response, painter) = ui.allocate_painter(size, egui::Sense::empty());
        let drawer = Drawer::new(painter, None);

        self.draw_all(&drawer);

        response
    }
}

impl ShowAttributeDisplay for ColourAttributeDisplay {}

pub struct EguiColourAttributesDisplay {
    pub cads: ColourAttributesDisplay,
}

impl EguiColourAttributesDisplay {
    pub fn new(cats: &[ColourAttributeType]) -> Self {
        Self {
            cads: ColourAttributesDisplay::new(cats),
        }
    }

    pub fn show(
        &mut self,
        ui: &mut egui::Ui,
        current_colour: Option<&impl Colour>,
        target_colour: Option<&impl Colour>,
        row_size: egui::Vec2,
    ) {
        self.cads.set_colour(current_colour);
        self.cads.set_target_colour(target_colour);
        ui.vertical(|ui| {
            let total_items = self.cads.len();

            for (count, cad) in self.cads.iter().enumerate() {
                cad.show(ui, row_size);
                if count < total_items - 1 {
                    ui.add_space(4.0);
                }
            }
        });
    }
}
