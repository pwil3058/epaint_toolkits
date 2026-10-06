// Copyright (c) 2026 Peter Williams <pwil3058@bigpond.net.au> <pwil3058@gmail.com>.

use colour_math::{ColourAttributeType, ColouredShape, HCV, HueConstants, RGBConstants, Shape};
use eframe::egui;

mod app_shell;
use app_shell::AppShell;

fn main() -> eframe::Result<()> {
    let mut options = eframe::NativeOptions::default();
    options.persist_window = true;

    // 🌟 FIX: Initialize window sizing using egui's correct ViewportBuilder structure!
    options.viewport = egui::ViewportBuilder::default().with_inner_size(egui::vec2(740.0, 520.0));

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

pub struct GuiRunner {
    pub core_shell: AppShell,
    pub standalone_wheel: colour_math_egui_lib::components::colour_wheel_panel::ColourWheelPanel,
}

impl GuiRunner {
    pub fn new(cc: &eframe::CreationContext<'_>, sliders: &[ColourAttributeType]) -> Self {
        let mut panel_instance =
            colour_math_egui_lib::components::colour_wheel_panel::ColourWheelPanel::new();

        for (colour, name, tooltip, shape) in [
            (&HCV::RED, "hcv_red", "Primary: Red", Shape::Circle),
            (&HCV::GREEN, "hcv_green", "Primary: Green", Shape::Circle),
            (&HCV::BLUE, "hcv_blue", "Primary: Blue", Shape::Circle),
            (&HCV::CYAN, "hcv_cyan", "Secondary: Cyan", Shape::Square),
            (
                &HCV::MAGENTA,
                "hcv_magenta",
                "Secondary: Magenta",
                Shape::Square,
            ),
            (
                &HCV::YELLOW,
                "hcv_yellow",
                "Secondary: Yellow",
                Shape::Square,
            ),
            (&HCV::WHITE, "hcv_white", "The lightest grey", Shape::Circle),
            (&HCV::BLACK, "hcv_black", "The darkest grey", Shape::Circle),
        ] {
            let shape = ColouredShape::new(colour, name, tooltip, shape);
            panel_instance.hue_wheel.add_item(shape);
        }

        Self {
            core_shell: AppShell::new(cc, sliders),
            standalone_wheel: panel_instance,
        }
    }
}

impl eframe::App for GuiRunner {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        ui.ctx().set_visuals(egui::Visuals::dark());

        use egui::containers::panel::CentralPanel;
        CentralPanel::default().show_inside(ui, |ui| {
            ui.set_height(ui.available_height());
            ui.set_width(ui.available_width());

            // 🌟 STABLE FIXED LAYOUT: Enforce an explicit horizontal layout flow with strict widths
            ui.horizontal(|ui| {
                // COLUMN 1 (LEFT): The Workbench Console Layout
                ui.vertical(|ui| {
                    ui.set_width(360.0); // Protect the dashboard boundaries from collapsing
                    self.core_shell.colour_editor.show(ui);
                });

                ui.add_space(24.0);
                ui.separator();
                ui.add_space(24.0);

                // COLUMN 2 (RIGHT): The Independent Populated Constants Reference Wheel
                ui.vertical(|ui| {
                    ui.heading("☸️ Independent Reference Wheel");
                    ui.add_space(4.0);
                    ui.separator();
                    ui.add_space(12.0);

                    // Dynamically calculate the ideal size for the wheel panel square
                    let ideal_size = ui
                        .available_width()
                        .min(ui.available_height() - 60.0)
                        .max(220.0);
                    self.standalone_wheel.show(ui, ideal_size);
                });
            });
        });
    }

    fn save(&mut self, _storage: &mut dyn eframe::Storage) {}
}
