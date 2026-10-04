// Copyright (c) 2026 Peter Williams <pwil3058@bigpond.net.au> <pwil3058@gmail.com>.

use colour_math::beigui::attr_display::ColourAttributeType;
use eframe::egui;

mod app_shell;
use app_shell::AppShell;

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

// 🌟 Clear enum managing our user interaction states cleanly
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
        }
    }
}

impl eframe::App for GuiRunner {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        ui.ctx().set_visuals(egui::Visuals::dark());

        // 🌟 STEP A: Drain screenshot events cleanly to avoid layout re-draw stutters
        let mut fresh_screenshot = None;
        ui.ctx().input_mut(|i| {
            let mut remaining_events = vec![];
            for event in i.events.drain(..) {
                if let egui::Event::Screenshot { image, .. } = event {
                    fresh_screenshot = Some(image);
                } else {
                    remaining_events.push(event);
                }
            }
            i.events = remaining_events;
        });

        // 🌟 STEP B: Handle State Changes smoothly
        if let Some(image) = fresh_screenshot {
            if self.state == AppState::WaitingForScreenshot {
                self.background_snapshot = Some(image);
                self.state = AppState::Sampling;
            }
        }

        // 📊 STANDARD INTERFACE RENDERING (Only active during normal mode)
        if self.state == AppState::Normal {
            // Load window width state persistence
            let storage_key = egui::Id::new("paint_assistant_sidebar_width");
            self.sidebar_width = ui
                .ctx()
                .data_mut(|d| *d.get_temp_mut_or_default::<f32>(storage_key));
            if self.sidebar_width < 100.0 {
                self.sidebar_width = 380.0;
            }

            use egui::containers::panel::CentralPanel;
            CentralPanel::default().show_inside(ui, |ui| {
                ui.set_height(ui.available_height());

                ui.columns(2, |columns| {
                    let col_left = &mut columns[0];
                    col_left.set_height(col_left.available_height());
                    col_left.heading("📊 Paint Series Manager");
                    col_left.separator();
                    col_left.add_space(4.0);
                    self.core_shell.table_view.ui(col_left);

                    let col_right = &mut columns[1];
                    col_right.set_height(col_right.available_height());
                    col_right.set_width(col_right.available_width());
                    col_right.vertical(|ui| {
                        self.core_shell.show(ui);
                    });
                });
            });

            // Check if our library app shell button clicked state triggered an overlay request
            let trigger_key = egui::Id::new("take_sample_button_clicked_signal");
            if ui
                .ctx()
                .data_mut(|d| *d.get_temp_mut_or_default::<bool>(trigger_key))
            {
                ui.ctx().data_mut(|d| d.insert_temp(trigger_key, false));
                self.state = AppState::WaitingForScreenshot;
                ui.ctx()
                    .send_viewport_cmd(egui::ViewportCommand::Screenshot(Default::default()));
            }
        }
        // 🌟 STEP C: ISOLATED SAMPLING MODE (Renders on a frozen clean background copy)
        else if self.state == AppState::Sampling {
            if let Some(ref image) = self.background_snapshot {
                let overlay_rect = ui.max_rect();
                let overlay_response =
                    ui.allocate_rect(overlay_rect, egui::Sense::click_and_drag());
                ui.ctx().set_cursor_icon(egui::CursorIcon::Crosshair);

                // Draw the clean base screenshot directly as our background layer first
                let temp_tex = ui.ctx().load_texture(
                    "sampling_backdrop",
                    (*image).clone(),
                    Default::default(),
                );
                ui.painter().image(
                    temp_tex.id(),
                    overlay_rect,
                    egui::Rect::from_min_max(egui::pos2(0.0, 0.0), egui::pos2(1.0, 1.0)),
                    egui::Color32::WHITE,
                );

                // Add a subtle dim overlay to make the marquee stand out sharply
                ui.painter()
                    .rect_filled(overlay_rect, 0.0, egui::Color32::from_black_alpha(60));

                if overlay_response.drag_started() {
                    self.drag_start_pos = overlay_response.interact_pointer_pos();
                }

                if overlay_response.dragged() {
                    if let (Some(start), Some(current)) =
                        (self.drag_start_pos, overlay_response.interact_pointer_pos())
                    {
                        let rect = egui::Rect::from_two_pos(start, current);
                        self.current_selection_rect = Some(rect);

                        // Draw selection box outline
                        ui.painter().rect_stroke(
                            rect,
                            0.0,
                            egui::Stroke::new(2.0, egui::Color32::WHITE),
                            egui::StrokeKind::Outside,
                        );
                    }
                }

                // 🌟 STEP D: Crop the exact coordinates instantly when drag stops
                if overlay_response.drag_stopped() {
                    if let Some(selection) = self.current_selection_rect {
                        // Because we are capturing on a static canvas copy, coordinates match 1:1 perfectly!
                        let x_min = (selection.min.x as usize).clamp(0, image.size[0]);
                        let y_min = (selection.min.y as usize).clamp(0, image.size[1]);
                        let x_max = (selection.max.x as usize).clamp(x_min, image.size[0]);
                        let y_max = (selection.max.y as usize).clamp(y_min, image.size[1]);

                        let crop_width = x_max - x_min;
                        let crop_height = y_max - y_min;

                        if crop_width > 0 && crop_height > 0 {
                            let mut cropped_pixels = Vec::with_capacity(crop_width * crop_height);
                            for y in y_min..y_max {
                                for x in x_min..x_max {
                                    let idx = y * image.size[0] + x;
                                    cropped_pixels.push(image.pixels[idx]);
                                }
                            }

                            let cropped_img = egui::ColorImage {
                                size: [crop_width, crop_height],
                                pixels: cropped_pixels,
                                source_size: egui::vec2(crop_width as f32, crop_height as f32),
                            };

                            // Run your pixel averaging algorithms instantly on the cropped selection patch
                            let mut total_r: u64 = 0;
                            let mut total_g: u64 = 0;
                            let mut total_b: u64 = 0;
                            let count = cropped_img.pixels.len() as u64;

                            for pixel in &cropped_img.pixels {
                                let ch = pixel.to_array(); // 🌟 Using your unmodified array discovery!
                                total_r += ch[0] as u64;
                                total_g += ch[1] as u64;
                                total_b += ch[2] as u64;
                            }

                            let avg_rgb = colour_math::rgb::RGB::<u8>::from([
                                (total_r / count) as u8,
                                (total_g / count) as u8,
                                (total_b / count) as u8,
                            ]);

                            // Automatically drive our workbench manipulator state model values
                            self.core_shell
                                .colour_editor
                                .manipulator_view
                                .model
                                .set_colour(&avg_rgb);

                            // Pipe the completed actual-size texture handle down into the pad preview container
                            self.core_shell.sample_texture = Some(ui.ctx().load_texture(
                                "live_screen_capture",
                                cropped_img,
                                Default::default(),
                            ));
                        }
                    }

                    // Reset state back to normal cleanly
                    self.state = AppState::Normal;
                    self.drag_start_pos = None;
                    self.current_selection_rect = None;
                    self.background_snapshot = None;
                }
            }
        }
    }

    fn save(&mut self, _storage: &mut dyn eframe::Storage) {}
}
