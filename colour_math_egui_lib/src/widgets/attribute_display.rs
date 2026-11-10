// Copyright (c) 2026 Peter Williams <pwil3058@bigpond.net.au> <pwil3058@gmail.com>.

use crate::drawer::Drawer;
use colour_math::ColourAttributeDisplay;
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
