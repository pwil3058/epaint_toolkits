// Copyright (c) 2026 Peter Williams <pwil3058@bigpond.net.au> <pwil3058@gmail.com>.
use eframe::egui;

struct ImageSamplerApp {
    start_pos: Option<egui::Pos2>,
    current_pos: Option<egui::Pos2>,
    screenshot: Option<egui::ColorImage>,
    sampled_image: Option<egui::ColorImage>,
    sampled_texture: Option<egui::TextureHandle>,
    waiting_for_screenshot: bool,
}

impl Default for ImageSamplerApp {
    fn default() -> Self {
        Self {
            start_pos: None,
            current_pos: None,
            screenshot: None,
            sampled_image: None,
            sampled_texture: None,
            waiting_for_screenshot: false,
        }
    }
}

impl eframe::App for ImageSamplerApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // 1. Capture the background screenshot
        if self.waiting_for_screenshot {
            ctx.input(|i| {
                for event in &i.raw.events {
                    if let egui::Event::Screenshot { image, .. } = event {
                        self.screenshot = Some(image.clone());
                        self.waiting_for_screenshot = false;
                    }
                }
            });
        }

        egui::SidePanel::left("control_panel").show(ctx, |ui| {
            ui.heading("Image Sampler Tool");
            if ui.button("📸 1. Capture Base Screen").clicked() {
                ctx.send_viewport_cmd(egui::ViewportCommand::Screenshot);
                self.waiting_for_screenshot = true;
            }
            ui.label("2. Drag a box over the canvas to crop.");

            // Display the cropped result if it exists
            if let Some(texture) = &self.sampled_texture {
                ui.separator();
                ui.label("Your Sample Image:");
                // Display the cropped image widget natively
                ui.image(texture);
            }
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            // Allocate the interactive zone
            let response = ui.allocate_response(ui.available_size(), egui::Sense::drag());

            if response.drag_started() {
                self.start_pos = response.interact_pointer_pos();
            }
            if response.dragged() {
                self.current_pos = response.interact_pointer_pos();
            }

            // 2. Crop the image once the user finishes dragging
            if response.drag_released() {
                if let (Some(start), Some(current), Some(img)) =
                    (self.start_pos, self.current_pos, &self.screenshot)
                {
                    let rect = egui::Rect::from_two_pos(start, current);

                    // Clamp selection bounds to ensure they remain inside the image boundary
                    let min_x = (rect.min.x as usize).clamp(0, img.width() - 1);
                    let max_x = (rect.max.x as usize).clamp(0, img.width() - 1);
                    let min_y = (rect.min.y as usize).clamp(0, img.height() - 1);
                    let max_y = (rect.max.y as usize).clamp(0, img.height() - 1);

                    let crop_width = max_x - min_x + 1;
                    let crop_height = max_y - min_y + 1;

                    if crop_width > 0 && crop_height > 0 {
                        // Extract pixel sequence for the cropped box area
                        let mut cropped_pixels = Vec::with_capacity(crop_width * crop_height);
                        for y in min_y..=max_y {
                            for x in min_x..=max_x {
                                cropped_pixels.push(img[(x, y)]);
                            }
                        }

                        // 3. Construct a brand new isolated ColorImage
                        let new_sample = egui::ColorImage {
                            size: [crop_width, crop_height],
                            pixels: cropped_pixels,
                        };

                        // Convert it to a texture handle so egui can draw it inside the UI
                        self.sampled_texture = Some(ctx.load_texture(
                            "cropped_sample",
                            new_sample.clone(),
                            Default::default(),
                        ));

                        self.sampled_image = Some(new_sample);
                    }
                }
                self.start_pos = None;
                self.current_pos = None;
            }

            // 4. Render visual box overlay during active drag
            if let (Some(start), Some(current)) = (self.start_pos, self.current_pos) {
                let preview_rect = egui::Rect::from_two_pos(start, current);
                ui.painter().rect(
                    preview_rect,
                    0.0,
                    egui::Color32::from_rgba_unmultiplied(255, 255, 255, 20),
                    egui::Stroke::new(1.5, egui::Color32::LIGHT_BLUE),
                );
            }
        });
    }
}
