// Copyright (c) 2026 Peter Williams <pwil3058@bigpond.net.au> <pwil3058@gmail.com>.

use colour_math::beigui::attr_display::ColourAttributeType;
use colour_math_egui_lib::app_shell::AppShell;
use eframe::egui;

fn main() -> eframe::Result<()> {
    let mut options = eframe::NativeOptions::default();
    options.persist_window = true; // Automatically serializes main window bounds across sessions

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
    pub sidebar_width: f32,
}

impl GuiRunner {
    pub fn new(cc: &eframe::CreationContext<'_>, sliders: &[ColourAttributeType]) -> Self {
        Self {
            core_shell: AppShell::new(cc, sliders),
            sidebar_width: 380.0,
        }
    }
}
// Replace your eframe::App implementation block inside colour_math_egui_gui/src/main.rs:

impl eframe::App for GuiRunner {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        use egui::containers::panel::CentralPanel;

        let ctx = ui.ctx();
        ctx.set_visuals(egui::Visuals::dark());

        // Intercept asynchronous platform screenshot response events natively inside input queue
        ctx.input(|i| {
            for event in &i.events {
                if let egui::Event::Screenshot { image, .. } = event {
                    let color_image =
                        egui::ColorImage::from_rgba_unmultiplied(image.size, image.as_raw());

                    self.core_shell.sample_texture = Some(ctx.load_texture(
                        "live_screen_capture",
                        color_image,
                        Default::default(),
                    ));
                }
            }
        });

        // Deploy the single central panel canvas container across the full app footprint
        CentralPanel::default().show_inside(ui, |ui| {
            ui.set_height(ui.available_height());

            // 🌟 FIX: Split the root space into two dedicated vertical layout columns natively!
            // egui handles the width calculations, forcing Column 2 to occupy all leftover space.
            ui.columns(2, |columns| {
                // 📊 COLUMN 1: The Paint Series Ledger Table View
                let col_left = &mut columns[0];
                col_left.set_height(col_left.available_height());

                col_left.heading("📊 Paint Series Manager");
                col_left.separator();
                col_left.add_space(4.0);

                self.core_shell.table_view.ui(col_left);

                // 🔬 COLUMN 2: The Full-Width Colour Workspace Console Viewport
                let col_right = &mut columns[1];
                col_right.set_height(col_right.available_height());

                // Force child layouts inside to stretch out to match this explicit width constraint
                col_right.set_width(col_right.available_width());

                // Render our library workspace shell components right into the central space
                self.core_shell.show(col_right);
            });
        });
    }

    fn save(&mut self, _storage: &mut dyn eframe::Storage) {
        // Persistence parameters are serialized here seamlessly
    }
}
