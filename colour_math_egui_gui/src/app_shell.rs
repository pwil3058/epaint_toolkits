// Copyright (c) 2026 Peter Williams <pwil3058@bigpond.net.au> <pwil3058@gmail.com>.

use colour_math::ColourAttributeType;
use colour_math_egui_lib::components::colour_editor::ColourEditor;
use eframe::egui;

pub struct AppShell {
    pub colour_editor: ColourEditor,
}

impl AppShell {
    pub fn new(_cc: &eframe::CreationContext<'_>, sliders: &[ColourAttributeType]) -> Self {
        let core_manipulator = colour_math::manipulator::ColourManipulator::builder().build();

        Self {
            colour_editor: ColourEditor::new(core_manipulator, sliders),
        }
    }

    pub fn show(&mut self, ui: &mut egui::Ui) {
        // Delegate the entire workbench presentation directly down into our unified ColourEditor component container
        self.colour_editor.show(ui);
    }
}
