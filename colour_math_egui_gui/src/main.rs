// Copyright (c) 2026 Peter Williams <pwil3058@bigpond.net.au> <pwil3058@gmail.com>.

use eframe::egui;

use colour_math::beigui::attr_display::ColourAttributeType;
use colour_math_egui_lib::app_shell::AppShell;

fn main() -> eframe::Result<()> {
    let mut options = eframe::NativeOptions::default();
    options.persist_window = true; // Safely serializes main window bounds across sessions

    // Configure your compile-time profile options at the very top
    let watercolour_profile = [
        ColourAttributeType::Hue,
        ColourAttributeType::Value,
        ColourAttributeType::Chroma,
        ColourAttributeType::Warmth,
    ];

    eframe::run_native(
        "Watercolour Painter's Colour Assistant",
        options,
        Box::new(move |cc| Ok(Box::new(GuiRunner::new(cc, &watercolour_profile)))),
    )
}

struct GuiRunner {
    pub core_shell: AppShell,
}

impl GuiRunner {
    pub fn new(cc: &eframe::CreationContext<'_>, sliders: &[ColourAttributeType]) -> Self {
        Self {
            core_shell: AppShell::new(cc, sliders),
        }
    }
}

// 🌟 THE SYSTEM LAYER APPLIES THE TRAIT APP BOUNDS HERE WITH THE CORRECT METHOD:
impl eframe::App for GuiRunner {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        ui.ctx().set_visuals(egui::Visuals::dark());

        // 🌟 FIX A: Use Panel::left but anchor it using `.show_inside(ui, ...)`
        // to pass the mutable layout handle that egui 0.36.2 expects.
        egui::Panel::left("paint_series_manager_panel")
            .resizable(true)
            .default_size(380.0)
            .show_inside(ui, |ui| {
                ui.add_space(4.0);
                self.core_shell.table_view.ui(ui);
            });

        // 🌟 FIX B: Anchor the Central Panel straight to the parent `ui` using `.show_inside(ui, ...)`
        // to prevent any 0-pixel height collapse and enable full workspace scrolling!
        egui::CentralPanel::default().show_inside(ui, |ui| {
            egui::ScrollArea::vertical()
                .id_salt("console_scroll_viewport")
                .show(ui, |ui| {
                    ui.add_space(4.0);
                    ui.heading("🔬 Colour Workspace Console");
                    ui.add_space(4.0);
                    ui.separator();
                    ui.add_space(4.0);

                    // Pipe our core library workspace shell straight onto the screen area
                    self.core_shell.show(ui);
                });
        });
    }
}
