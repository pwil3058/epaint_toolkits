// Complete refactor of colour_math_egui_lib/src/widgets/attribute_deck.rs
// Copyright (c) 2026 Peter Williams <pwil3058@bigpond.net.au> <pwil3058@gmail.com>.

use crate::egui_drawer::EguiDrawer;
use colour_math::{
    HCV,
    beigui::attr_display::{ColourAttributeDisplay, ColourAttributeType},
};
use eframe::egui;

pub struct AttributeDeck;

impl AttributeDeck {
    pub fn show(
        ui: &mut egui::Ui,
        attributes: &[ColourAttributeType],
        current: Option<&HCV>,
        target: Option<&HCV>,
    ) {
        ui.vertical(|ui| {
            // 🌟 FIX: Read the layout bounds of the immediate parent frame region
            // so our widths map perfectly to the active column space rather than stretching infinitely.
            let deck_width = ui.available_width().clamp(200.0, 600.0);

            for attr_type in attributes {
                let mut cad = ColourAttributeDisplay::new(attr_type);
                cad.set_colour(current);
                cad.set_target_colour(target);

                let (resp, painter) =
                    ui.allocate_painter(egui::vec2(deck_width, 22.0), egui::Sense::hover());

                let drawer = EguiDrawer::new(&painter, resp.rect, resp.rect);
                cad.draw_all(&drawer);
                ui.add_space(4.0);
            }
        });
    }
}
