// Copyright (c) 2026 Peter Williams <pwil3058@bigpond.net.au> <pwil3058@gmail.com>.

use colour_math::{ColourBasics, HCV, Prop};
use eframe::egui;

pub struct DirectionalNudgeButton<'a> {
    pub label: &'a str,
    pub target_direction_colour: &'a HCV,
}

impl<'a> DirectionalNudgeButton<'a> {
    pub fn new(label: &'a str, target_direction_colour: &'a HCV) -> Self {
        Self {
            label,
            target_direction_colour,
        }
    }

    /// Renders the colour-tinted nudge button natively.
    pub fn show(self, ui: &mut egui::Ui) -> egui::Response {
        // 1. Convert your type-safe internal Prop array triplet straight to egui colour bytes
        let rgb_array = <[Prop; 3]>::from(*self.target_direction_colour);
        let button_colour = egui::Color32::from_rgb(
            u8::from(rgb_array[0]),
            u8::from(rgb_array[1]),
            u8::from(rgb_array[2]),
        );

        // 2. Extract your custom best_foreground() calculation to guarantee high text contrast
        let fg_array = <[Prop; 3]>::from(self.target_direction_colour.best_foreground());
        let text_colour = egui::Color32::from_rgb(
            u8::from(fg_array[0]),
            u8::from(fg_array[1]),
            u8::from(fg_array[2]),
        );

        // 3. Override background styling immediately on this frame loop sweep
        let widgets = &mut ui.style_mut().visuals.widgets;
        widgets.inactive.bg_fill = button_colour;
        widgets.inactive.fg_stroke.color = text_colour;

        // Brighten background slightly on mouse hover sweeps
        widgets.hovered.bg_fill = button_colour.linear_multiply(1.15);
        widgets.hovered.fg_stroke.color = text_colour;

        // Render the raw button widget onto the layout canvas area
        let response = ui.button(self.label);

        // 4. Reset style immediately so we do not pollute parent or neighbouring containers
        ui.reset_style();

        response
    }
}
