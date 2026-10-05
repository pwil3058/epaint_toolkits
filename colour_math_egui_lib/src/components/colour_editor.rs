// Copyright (c) 2026 Peter Williams <pwil3058@bigpond.net.au> <pwil3058@gmail.com>.

use crate::colour::Dedans;
use crate::components::colour_manipulator_view::ColourManipulatorView;
use crate::widgets::sample_field::SampleField;
use colour_math::{
    ColourBasics, beigui::attr_display::ColourAttributeType, manipulator::ColourManipulator,
};
use eframe::egui; // Uses your clean, un-premultiplied trait conversion

pub struct ColourEditor {
    pub active_rgb: colour_math::rgb::RGB<u8>,
    pub displayed_attributes: Vec<ColourAttributeType>,
    pub manipulator_view: ColourManipulatorView,
    pub sample_field: SampleField,
}

impl ColourEditor {
    pub fn new(initial_model: ColourManipulator, sliders: &[ColourAttributeType]) -> Self {
        Self {
            active_rgb: initial_model.rgb::<u8>(),
            displayed_attributes: sliders.to_vec(),
            manipulator_view: ColourManipulatorView::new(initial_model),
            sample_field: SampleField::new(),
        }
    }

    /// Renders the entire standalone colour workbench console layout on screen.
    pub fn show(&mut self, ui: &mut egui::Ui) {
        ui.vertical(|ui| {
            ui.heading("🔬 Colour Workbench Console");
            ui.add_space(4.0);
            ui.separator();
            ui.add_space(8.0);

            // Lay components out side-by-side horizontally: Sliders on left, Field on right
            ui.horizontal(|ui| {
                // 📊 REGION 1: Colour Attribute Sliders & Interactive Wheels
                ui.vertical(|ui| {
                    ui.set_width(320.0);
                    // Pass the expected bounding size to drive your hue wheel rendering calculations
                    let view_dimensions = egui::vec2(320.0, ui.available_height() - 20.0);
                    self.manipulator_view.show(ui, view_dimensions);
                });

                ui.add_space(16.0);
                ui.separator();
                ui.add_space(16.0);

                // 🎨 REGION 2: Reusable Multi-Sample Drawing Area Field Panel
                ui.vertical(|ui| {
                    // Render our digital R, G, B text field hex editors inside the group panel
                    crate::widgets::digital_readout::DigitalReadout::show(
                        ui,
                        &mut self.manipulator_view.model,
                    );
                    ui.add_space(8.0);

                    // Fetch the true primitive background color via your un-premultiplied trait conversion
                    let base_bg: egui::Color32 = self.manipulator_view.model.hcv().dedans();

                    // Render our self-contained, multi-sample spatial drawing field widget natively!
                    let (_field_resp, color_update_signal) = self.sample_field.show(ui, base_bg);

                    // If a right-click paste or deletion event triggered a color recalculation this frame:
                    if let Some(new_avg_color) = color_update_signal {
                        let storage_key = egui::Id::new("on_paste_auto_toggle");
                        let on_paste_automatic = ui
                            .ctx()
                            .data_mut(|d| *d.get_temp_mut_or_default::<bool>(storage_key));

                        if on_paste_automatic {
                            // Automatically align our sliders to match the newly pasted average color!
                            self.manipulator_view.model.set_colour(&new_avg_color);
                        }
                    }
                });
            });
        });
    }
}
