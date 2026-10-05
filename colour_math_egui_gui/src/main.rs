// Copyright (c) 2026 Peter Williams <pwil3058@bigpond.net.au> <pwil3058@gmail.com>.

use colour_math::ColourBasics;
use colour_math::beigui::attr_display::ColourAttributeType;
use eframe::egui;

mod app_shell;
use app_shell::AppShell;

// 🌟 FIX A: Correct the library module lookup path to pull straight out of your `colour` file module!
use colour_math_egui_lib::colour::{AverageColour, Dedans, Depuis};

fn main() -> eframe::Result<()> {
    let mut options = eframe::NativeOptions::default();
    options.persist_window = true;

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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum AppState {
    Normal,
    WaitingForScreenshot,
    Sampling,
}

pub struct GuiRunner {
    pub core_shell: AppShell,
    pub sidebar_width: f32,
    state: AppState,
    pub drag_start_pos: Option<egui::Pos2>,
    pub current_selection_rect: Option<egui::Rect>,
    pub background_snapshot: Option<std::sync::Arc<egui::ColorImage>>,
    pub background_texture: Option<egui::TextureHandle>,
}

impl GuiRunner {
    pub fn new(cc: &eframe::CreationContext<'_>, sliders: &[ColourAttributeType]) -> Self {
        Self {
            core_shell: AppShell::new(cc, sliders),
            sidebar_width: 380.0,
            state: AppState::Normal,
            drag_start_pos: None,
            current_selection_rect: None,
            background_snapshot: None,
            background_texture: None,
        }
    }
}

impl eframe::App for GuiRunner {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        ui.ctx().set_visuals(egui::Visuals::dark());

        // Deploy the single central panel canvas container across the full app footprint
        use egui::containers::panel::CentralPanel;
        CentralPanel::default().show_inside(ui, |ui| {
            ui.set_height(ui.available_height());
            ui.set_width(ui.available_width());

            // 🌟 FIX: Render the entire unified ColourEditor directly onto the workspace window!
            // This displays your sliders, readouts, and SampleField side-by-side cleanly.
            self.core_shell.colour_editor.show(ui);
        });
    }

    fn save(&mut self, _storage: &mut dyn eframe::Storage) {}
}
