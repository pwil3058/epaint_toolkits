// Copyright (c) 2026 Peter Williams <pwil3058@bigpond.net.au> <pwil3058@gmail.com>.

use eframe::egui;

pub struct TakeSampleButton;

impl TakeSampleButton {
    /// Renders a standalone, reusable screen sample button block
    pub fn show(ui: &mut egui::Ui) -> bool {
        if ui.button("📸 Take Screen Sample").clicked() {
            // 🌟 FIX: Pass a default UserData argument inside the tuple parameters
            // to satisfy the constructor requirements of egui 0.36.2.
            ui.ctx()
                .send_viewport_cmd(egui::ViewportCommand::Screenshot(Default::default()));
            true
        } else {
            false
        }
    }
}
