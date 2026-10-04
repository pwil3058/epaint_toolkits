// Copyright (c) 2026 Peter Williams <pwil3058@bigpond.net.au> <pwil3058@gmail.com>.

use crate::{
    components::colour_manipulator_pad::ColourManipulatorPad,
    components::colour_manipulator_view::ColourManipulatorView, egui_drawer::EguiDrawer,
    widgets::rgb_hex_editor::RgbHexEditor,
};
use colour_math::{
    ColourBasics, Prop, RGB, UFDRNumber,
    beigui::attr_display::{ColourAttributeDisplay, ColourAttributeType},
    manipulator::ColourManipulator,
};
use eframe::egui;

pub struct ColourEditor {
    pub manipulator_view: ColourManipulatorView,
    pub pad: ColourManipulatorPad,
    pub active_rgb: RGB<u8>,
    pub displayed_attributes: Vec<ColourAttributeType>,
}

impl ColourEditor {
    /// 🆕 Accept a customizable array slice of attributes upon dashboard creation
    pub fn new(initial_model: ColourManipulator, sliders: &[ColourAttributeType]) -> Self {
        Self {
            active_rgb: initial_model.rgb::<u8>(),
            // 🌟 FIX: Instantiated using the zero-argument constructor matching our updated Pad struct
            pad: ColourManipulatorPad::new(),
            manipulator_view: ColourManipulatorView::new(initial_model),
            displayed_attributes: sliders.to_vec(),
        }
    }

    fn is_settable(attr_type: &ColourAttributeType) -> bool {
        match attr_type {
            ColourAttributeType::Warmth => false,
            _ => true,
        }
    }

    pub fn show(&mut self, ui: &mut egui::Ui) {
        ui.vertical(|ui| {
            // -----------------------------------------------------------------
            // STACK 1: The Configured Dynamic Attribute Sliders Trackers
            // -----------------------------------------------------------------
            let current_hcv = self.manipulator_view.model.hcv();

            // Iterate natively over your customized collection constraints
            for attr_type in &self.displayed_attributes {
                let mut cad = ColourAttributeDisplay::new(attr_type);
                cad.set_colour(Some(&current_hcv));

                ui.horizontal(|ui| {
                    ui.add_space(2.0);

                    let (cad_rect, cad_resp) = ui.allocate_exact_size(
                        egui::vec2(ui.available_width() - 40.0, 24.0),
                        if Self::is_settable(attr_type) {
                            egui::Sense::click_and_drag()
                        } else {
                            egui::Sense::hover()
                        },
                    );

                    let cad_drawer = EguiDrawer::new(ui.painter(), cad_rect, cad_rect);
                    cad.draw_all(&cad_drawer);

                    if Self::is_settable(attr_type) && cad_resp.dragged() {
                        if let Some(pointer_pos) = ui.ctx().input(|i| i.pointer.interact_pos()) {
                            let click_fraction = ((pointer_pos.x - cad_rect.left())
                                / cad_rect.width())
                            .clamp(0.0, 1.0);
                            let new_prop = Prop::from(click_fraction as f64);

                            match attr_type {
                                ColourAttributeType::Value => {
                                    let ufdr_val = UFDRNumber::from(click_fraction as f64) * 3;
                                    self.manipulator_view.model.set_sum(
                                        ufdr_val,
                                        colour_math::manipulator::SetScalar::Accommodate,
                                    );
                                }
                                ColourAttributeType::Chroma => {
                                    self.manipulator_view.model.set_chroma(
                                        new_prop,
                                        colour_math::manipulator::SetScalar::Accommodate,
                                    );
                                }
                                _ => {}
                            }
                        }
                    }
                });
                ui.add_space(4.0);
            }

            ui.separator();
            ui.add_space(4.0);

            // -----------------------------------------------------------------
            // STACK 2: Precise Numeric RgbHexEditor Line Strip
            // -----------------------------------------------------------------
            let prev_rgb = self.active_rgb;

            RgbHexEditor::new(&mut self.active_rgb).show(ui);

            if self.active_rgb != prev_rgb {
                // Pipeline the structural active_rgb state using your engine's native From trait configurations
                let prop_array = <[Prop; 3]>::from(self.active_rgb);
                let target_rgb_u64 = RGB::<u64>::from(prop_array);

                self.manipulator_view.model.set_colour(&target_rgb_u64);
            }

            ui.add_space(6.0);

            // -----------------------------------------------------------------
            // STACK 3: The Standalone Directional Nudge Pad Ring & Backdrop Area
            // -----------------------------------------------------------------
            self.pad
                .show(ui, &mut self.manipulator_view.model, &mut None);

            ui.add_space(6.0);

            self.active_rgb = RGB::<u8>::from(self.manipulator_view.model.hcv());

            // -----------------------------------------------------------------
            // STACK 4: The Live Workspace Sample Patches Canvas View
            // -----------------------------------------------------------------
            self.manipulator_view
                .show(ui, egui::vec2(ui.available_width(), 120.0));
        });
    }
}
