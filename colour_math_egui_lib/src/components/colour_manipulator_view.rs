// Copyright (c) 2026 Peter Williams <pwil3058@bigpond.net.au> <pwil3058@gmail.com>.

use colour_math::{ColourBasics, Prop, manipulator::ColourManipulator};
use eframe::egui;

pub struct ColourManipulatorView {
    // The core GUI-free model engine doing the heavy math validation lifting
    pub model: ColourManipulator,
    // List of rect colour samples captured or dropped into this workspace area
    pub sample_patches: Vec<[Prop; 3]>,
}

impl ColourManipulatorView {
    pub fn new(initial_model: ColourManipulator) -> Self {
        Self {
            model: initial_model,
            sample_patches: Vec::new(),
        }
    }

    /// Renders the integrated sample workspace backdrop and patch lists.
    pub fn show(&mut self, ui: &mut egui::Ui, dimensions: egui::Vec2) {
        // 1. Extract the current validated colour from your model brain
        let current_hcv = self.model.hcv();
        let current_rgb_prop = <[Prop; 3]>::from(current_hcv);

        let background_colour = egui::Color32::from_rgb(
            u8::from(current_rgb_prop[0]),
            u8::from(current_rgb_prop[1]),
            u8::from(current_rgb_prop[2]),
        );

        // 2. Allocate the precise workspace frame canvas box up front
        let (rect, _response) = ui.allocate_exact_size(dimensions, egui::Sense::hover());

        // Fill the frame background container with your active current colour natively
        ui.painter().rect_filled(rect, 0.0, background_colour);

        // 3. Render your list of rectangular colour samples layout grouping axis
        // 🌟 FIX 1 & 3: Pass plain dimensions Vec2 directly to map the toolchain environment accurately
        ui.allocate_ui(dimensions, |ui| {
            // 🌟 FIX 2: Swapped back to horizontal_wrapped matching egui 0.36.2!
            ui.horizontal_wrapped(|ui| {
                let mut patch_to_remove: Option<usize> = None;

                for (idx, patch) in self.sample_patches.iter().enumerate() {
                    let patch_colour = egui::Color32::from_rgb(
                        u8::from(patch[0]),
                        u8::from(patch[1]),
                        u8::from(patch[2]),
                    );

                    // Allocate an independent rectangle box for each separate sample patch
                    let (patch_rect, patch_resp) =
                        ui.allocate_exact_size(egui::vec2(64.0, 48.0), egui::Sense::click());

                    ui.painter().rect_filled(patch_rect, 4.0, patch_colour);

                    ui.painter().rect_stroke(
                        patch_rect,
                        4.0,
                        egui::Stroke::new(1.0, egui::Color32::BLACK),
                        egui::StrokeKind::Middle,
                    );

                    // 4. THE INTERACTIVE POPUP CONTEXT MENU
                    patch_resp.context_menu(|ui| {
                        if ui.button("📋 Paste to Workspace").clicked() {
                            // Link this patch array smoothly back to update your engine properties
                        }
                        if ui.button("❌ Remove Sample").clicked() {
                            patch_to_remove = Some(idx);
                        }
                    });
                }

                // Cleanly mutate your array memory outside the rendering iteration pass loop
                if let Some(remove_idx) = patch_to_remove {
                    self.sample_patches.remove(remove_idx);
                }
            });
        });
    }
}
