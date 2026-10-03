// Copyright (c) 2026 Peter Williams <pwil3058@bigpond.net.au> <pwil3058@gmail.com>.

use eframe::egui;

#[derive(Default, Debug, Clone, Copy, PartialEq, Eq)]
pub enum NudgeAction {
    #[default]
    None,
    ValueUp,
    ValueDown,
    Left,
    Right,
    ChromaDownGreynessUp,
    ChromaUpGreynessDown,
}

pub struct NudgePad;

impl NudgePad {
    pub fn show(ui: &mut egui::Ui, inner_content: impl FnOnce(&mut egui::Ui)) -> NudgeAction {
        let mut action = NudgeAction::None;

        ui.vertical_centered(|ui| {
            if ui.button("Value++").clicked() {
                action = NudgeAction::ValueUp;
            }

            ui.horizontal(|ui| {
                if ui.button("<").clicked() {
                    action = NudgeAction::Left;
                }

                // Allow the parent container to pass in your custom SampleField dynamically!
                inner_content(ui);

                if ui.button(">").clicked() {
                    action = NudgeAction::Right;
                }
            });

            if ui.button("Value--").clicked() {
                action = NudgeAction::ValueDown;
            }

            ui.add_space(4.0);
            ui.horizontal(|ui| {
                if ui.button("Chroma-- / Greyness++").clicked() {
                    action = NudgeAction::ChromaDownGreynessUp;
                }
                ui.add_space(12.0);
                if ui.button("Chroma++ / Greyness--").clicked() {
                    action = NudgeAction::ChromaUpGreynessDown;
                }
            });
        });

        action
    }
}
