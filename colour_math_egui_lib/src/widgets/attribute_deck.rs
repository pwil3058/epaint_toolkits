// Copyright (c) 2026 Peter Williams <pwil3058@bigpond.net.au> <pwil3058@gmail.com>.

use crate::widgets::attribute_display::AttributeDisplay;
use colour_math::{ColourAttributeDisplay, ColourAttributeType, ColourBasics};
use eframe::egui;

/// A stateless layout container widget that handles rendering an ordered array
/// stack of attribute indicator bars configured in either a vertical or horizontal layout.
pub struct AttributeDeck {
    /// The structural array profile defining exactly which attributes to render on screen.
    pub profile: Vec<ColourAttributeType>,
    /// The active immediate-mode orientation layout constraint (egui::Direction::TopDown or LeftToRight).
    pub direction: egui::Direction,
}

impl AttributeDeck {
    /// Creates a definitive multi-directional attribute display layout deck.
    ///
    /// # Arguments
    /// * `profile` - The ordered sequence of attributes to display.
    /// * `direction` - The fixed rendering direction layout constraint (e.g., `egui::Direction::TopDown`).
    pub fn new(profile: &[ColourAttributeType], direction: egui::Direction) -> Self {
        Self {
            profile: profile.to_vec(),
            direction,
        }
    }

    /// Renders the complete, ordered layout deck structure on screen.
    ///
    /// * `ui` - The active drawing interface window context frame handle.
    /// * `current_colour` - The primary color value driving the main indicator needles.
    /// * `target_colour` - An optional comparison target color to map background lines.
    // Inside colour_math_egui_lib/src/widgets/attribute_deck.rs (around line 46)

    /// Renders the complete, ordered layout deck structure on screen.
    ///
    /// * `ui` - The active drawing interface window context frame handle.
    /// * `current_colour` - The primary color value driving the main indicator needles.
    /// * `target_colour` - An optional comparison target color to map background lines.
    pub fn show(
        &self,
        ui: &mut egui::Ui,
        current_colour: &impl ColourBasics,
        target_colour: Option<&impl ColourBasics>,
    ) {
        let single_display = AttributeDisplay::new();
        let gap_spacer = 4.0;

        // 🌟 FIX: Use egui's correct, explicit `from_main_dir_and_cross_align` constructor function name!
        ui.with_layout(
            egui::Layout::from_main_dir_and_cross_align(self.direction, egui::Align::Min),
            |ui| {
                let total_items = self.profile.len();

                for (idx, attr_type) in self.profile.iter().enumerate() {
                    let mut display_config = ColourAttributeDisplay::new(attr_type);

                    display_config.set_colour(Some(current_colour));
                    if let Some(target) = target_colour {
                        display_config.set_target_colour(Some(target));
                    }

                    single_display.show(ui, &display_config);

                    if idx < total_items - 1 {
                        ui.add_space(gap_spacer);
                    }
                }
            },
        );
    }
}
