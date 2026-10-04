// Copyright (c) 2026 Peter Williams <pwil3058@bigpond.net.au> <pwil3058@gmail.com>.
// Complete refactor of colour_math_egui_lib/src/widgets/take_sample_button.rs
// Copyright (c) 2026 Peter Williams <pwil3058@bigpond.net.au> <pwil3058@gmail.com>.

use eframe::egui;

pub struct TakeSampleButton;

impl TakeSampleButton {
    pub fn show(ui: &mut egui::Ui) -> bool {
        if ui.button("📸 Take Screen Sample").clicked() {
            // 🌟 FIX: Signal the main frame loop to activate the crosshair marquee selection overlay
            let trigger_key = egui::Id::new("take_sample_button_clicked_signal");
            ui.ctx().data_mut(|d| d.insert_temp(trigger_key, true));
            true
        } else {
            false
        }
    }
}
