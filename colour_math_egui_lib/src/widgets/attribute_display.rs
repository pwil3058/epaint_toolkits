// Copyright (c) 2026 Peter Williams <pwil3058@bigpond.net.au> <pwil3058@gmail.com>.

use crate::egui_drawer::EguiDrawer;
use colour_math::ColourAttributeDisplay;
use eframe::egui;

/// A stateless immediate-mode widget that renders a single color attribute
/// indicator bar strip driven entirely by backend display traits.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct AttributeDisplay;

impl AttributeDisplay {
    /// Zero-parameter helper forwarding straight to default to preserve API ergonomics.
    pub fn new() -> Self {
        Self::default()
    }

    /// Renders a single attribute display bar on screen.
    ///
    /// * `ui` - The active layout window context.
    /// * `display_config` - The specific backend attribute display row context (e.g., HueCAD, ValueCAD).
    pub fn show(
        &self,
        ui: &mut egui::Ui,
        display_config: &ColourAttributeDisplay,
    ) -> egui::Response {
        let row_width = 320.0;
        let row_height = 24.0; // Clean thin dimension profile matching indicator bar strips

        // Allocate our explicit thin bar boundaries inside the current layout container tree
        let (rect, response) =
            ui.allocate_exact_size(egui::vec2(row_width, row_height), egui::Sense::hover());

        // Fetch a painter instance bound and hardware-clipped strictly to this row footprint
        let painter = ui.painter_at(rect);

        // Instantiate your production EguiDrawer bridge context
        let drawer = EguiDrawer::new(&painter, rect, rect);

        // 🌟 DELEGATE DRAWING: Hand total rendering control directly to your backend rules!
        // Your backend executes its own gradients, target lines, and text placement.
        display_config.draw_all(&drawer);

        response
    }
}
