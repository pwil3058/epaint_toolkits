// Copyright (c) 2026 Peter Williams <pwil3058@bigpond.net.au> <pwil3058@gmail.com>.

use crate::widgets::{colour_button::DirectionalNudgeButton, sample_field::SampleField};
use colour_math::{ColourBasics, HCV, Prop, hue::angle::Angle, manipulator::ColourManipulator};
use eframe::egui;

pub struct ColourManipulatorPad {
    /// Fine-tuning angle increment step used when nudging the hue buttons left or right
    pub hue_step: Angle,
    /// Fine-tuning proportion step used when nudging chroma or value scalars up or down
    pub scalar_step: Prop,
}

impl ColourManipulatorPad {
    pub fn new() -> Self {
        Self {
            // Default to a crisp 5-degree arc step and a standard fractional prop nudge increment
            hue_step: Angle::from(5.0),
            scalar_step: Prop::from(0.05_f64),
        }
    }

    /// Renders the standalone color manipulator ring, piping clicks right into your engine model.
    pub fn show(
        &self,
        ui: &mut egui::Ui,
        manipulator: &mut ColourManipulator,
        texture: Option<&egui::TextureHandle>,
    ) {
        ui.vertical_centered(|ui| {
            // -----------------------------------------------------------------
            // 1. TOP AXIS: Value++ Nudge Button
            // -----------------------------------------------------------------
            let mut val_up_model = ColourManipulator::builder()
                .init_hcv(&manipulator.hcv())
                .build();
            val_up_model.incr_value(self.scalar_step);
            let val_up_tint = val_up_model.hcv();

            if DirectionalNudgeButton::new("Value++", &val_up_tint)
                .show(ui)
                .clicked()
            {
                manipulator.incr_value(self.scalar_step);
            }

            ui.add_space(4.0);

            // -----------------------------------------------------------------
            // 2. CENTRAL ROW: Left Hue, Central Grey Sampling Canvas, Right Hue
            // -----------------------------------------------------------------
            ui.horizontal(|ui| {
                // Left Hue Nudge Button ("<")
                let mut hue_left_model = ColourManipulator::builder()
                    .init_hcv(&manipulator.hcv())
                    .build();
                hue_left_model.rotate(-self.hue_step);
                let hue_left_tint = hue_left_model.hcv();

                if DirectionalNudgeButton::new(" < ", &hue_left_tint)
                    .show(ui)
                    .clicked()
                {
                    manipulator.rotate(-self.hue_step);
                }

                // Central Gray backdrop matrix canvas image box
                SampleField::new(texture, egui::vec2(220.0, 220.0)).show(ui);

                // Right Hue Nudge Button (">")
                let mut hue_right_model = ColourManipulator::builder()
                    .init_hcv(&manipulator.hcv())
                    .build();
                hue_right_model.rotate(self.hue_step);
                let hue_right_tint = hue_right_model.hcv();

                if DirectionalNudgeButton::new(" > ", &hue_right_tint)
                    .show(ui)
                    .clicked()
                {
                    manipulator.rotate(self.hue_step);
                }
            });

            ui.add_space(4.0);

            // -----------------------------------------------------------------
            // 3. BOTTOM AXIS: Value-- Nudge Button
            // -----------------------------------------------------------------
            let mut val_down_model = ColourManipulator::builder()
                .init_hcv(&manipulator.hcv())
                .build();
            val_down_model.decr_value(self.scalar_step);
            let val_down_tint = val_down_model.hcv();

            if DirectionalNudgeButton::new("Value--", &val_down_tint)
                .show(ui)
                .clicked()
            {
                manipulator.decr_value(self.scalar_step);
            }

            ui.add_space(8.0);

            // -----------------------------------------------------------------
            // 4. FOOTER ROW: Chroma / Greyness Tuning Shifts
            // -----------------------------------------------------------------
            ui.horizontal(|ui| {
                // Chroma-- / Greyness++ Nudge Button
                let mut chroma_down_model = ColourManipulator::builder()
                    .init_hcv(&manipulator.hcv())
                    .build();
                chroma_down_model.decr_chroma(self.scalar_step);
                let chroma_down_tint = chroma_down_model.hcv();

                if DirectionalNudgeButton::new("Chroma-- / Greyness++", &chroma_down_tint)
                    .show(ui)
                    .clicked()
                {
                    manipulator.decr_chroma(self.scalar_step);
                }

                ui.add_space(12.0);

                // Chroma++ / Greyness-- Nudge Button
                let mut chroma_up_model = ColourManipulator::builder()
                    .init_hcv(&manipulator.hcv())
                    .build();
                chroma_up_model.incr_chroma(self.scalar_step);
                let chroma_up_tint = chroma_up_model.hcv();

                if DirectionalNudgeButton::new("Chroma++ / Greyness--", &chroma_up_tint)
                    .show(ui)
                    .clicked()
                {
                    manipulator.incr_chroma(self.scalar_step);
                }
            });
        });
    }
}
