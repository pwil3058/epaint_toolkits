// Copyright (c) 2026 Peter Williams <pwil3058@bigpond.net.au> <pwil3058@gmail.com>.

use crate::colour::{Dedans, SampleImage, SampleImages};
use eframe::egui;
use std::str::FromStr;

#[derive(Default)]
pub struct SampleField {
    pub samples: SampleImages,
    // Parallel GPU texture rendering cache mirror matching our underlying samples vector 1:1
    texture_render_cache: Vec<egui::TextureHandle>,
}

impl SampleField {
    pub fn new() -> Self {
        Self::default()
    }

    /// Spatially registers a new image patch where the cursor pointer was released,
    /// compiles its texture mirror asset, and returns the re-computed color average.
    pub fn add_spatial_sample(
        &mut self,
        ctx: &egui::Context,
        image: egui::ColorImage,
        dropped_pos: egui::Pos2,
    ) -> colour_math::rgb::RGB<u8> {
        let unique_id = format!("sample_patch_idx_{}", self.samples.0.len());
        let texture_handle = ctx.load_texture(&unique_id, image.clone(), Default::default());

        // Compute the complete mathematical bounding rectangle box instantly on paste
        let size_vec = texture_handle.size_vec2();
        let bound_rect = egui::Rect::from_min_size(dropped_pos, size_vec);

        self.texture_render_cache.push(texture_handle);
        self.samples.0.push(SampleImage {
            image,
            rect: bound_rect,
        });

        self.samples.average_colour()
    }

    pub fn clear(&mut self) {
        self.samples.0.clear();
        self.texture_render_cache.clear();
    }

    /// Renders the spatial canvas container on screen.
    /// Returns `Some(RGB<u8>)` if a paste or removal recalculation occurs during this frame pass.
    pub fn show(
        &mut self,
        ui: &mut egui::Ui,
        bg_color: egui::Color32,
    ) -> (egui::Response, Option<colour_math::rgb::RGB<u8>>) {
        let content_side = 220.0;

        let (field_rect, response) = ui.allocate_exact_size(
            egui::vec2(content_side, content_side),
            egui::Sense::click_and_drag(),
        );

        let painter = ui.painter_at(field_rect);

        // 1. Paint the customizable base solid background panel color cleanly
        painter.rect_filled(field_rect, 4.0, bg_color);

        let mut hovered_patch_origin = None;
        let mut color_update_signal = None;
        let pointer_pos = ui.ctx().input(|i| i.pointer.hover_pos());

        // 2. Render all sample assets from oldest to newest layer passes
        let total_samples = self.samples.0.len();
        for idx in 0..total_samples {
            let sample = &self.samples.0[idx];
            let texture = &self.texture_render_cache[idx];

            painter.image(
                texture.id(),
                sample.rect,
                egui::Rect::from_min_max(egui::pos2(0.0, 0.0), egui::pos2(1.0, 1.0)),
                egui::Color32::WHITE,
            );

            painter.rect_stroke(
                sample.rect,
                0.0,
                egui::Stroke::new(1.0, egui::Color32::from_gray(180)),
                egui::StrokeKind::Outside,
            );

            // Highlight hovered layer bounding boxes
            if let Some(pos) = pointer_pos {
                if sample.rect.contains(pos) {
                    painter.rect_stroke(
                        sample.rect,
                        0.0,
                        egui::Stroke::new(1.5, egui::Color32::WHITE),
                        egui::StrokeKind::Outside,
                    );

                    // Keep track of the actual min position of the hovered sample for hit testing
                    hovered_patch_origin = Some((sample.rect.min, idx));
                }
            }
        }

        // Outer border outline
        painter.rect_stroke(
            field_rect,
            4.0,
            egui::Stroke::new(1.0, egui::Color32::from_gray(64)),
            egui::StrokeKind::Middle,
        );

        // 🌟 3. POPUP CONTEXT MENU: Bulletproof inline layout text parsing!
        response.context_menu(|ui| {
            ui.label("📋 Paste Hex Color String:");

            // Persist an isolated string edit buffer inside egui's temporary storage map
            let buffer_key = ui.id().with("menu_paste_buffer");
            let mut text_buffer = ui
                .ctx()
                .data_mut(|d| d.get_temp_mut_or_default::<String>(buffer_key).clone());

            // Render an active input text widget line where standard Ctrl+V key pasting works natively
            let text_edit = egui::TextEdit::singleline(&mut text_buffer)
                .hint_text("Type or Paste 0xHex here")
                .desired_width(140.0);

            if ui.add(text_edit).changed() {
                ui.ctx()
                    .data_mut(|d| d.insert_temp(buffer_key, text_buffer.clone()));
            }

            // Clicking Submit parses the string through your custom colour_math modules seamlessly
            if ui.button("🆗 Inject Color Sample").clicked() && !text_buffer.is_empty() {
                if let Ok(parsed_rgb) = colour_math::rgb::RGB::<u8>::from_str(&text_buffer) {
                    // 🌟 FIX: Wrap your clean, un-premultiplied Color32 inside a vec! macro container
                    // to perfectly satisfy epaint's image vector signature requirements!
                    let color_image = egui::ColorImage::new([1, 1], vec![parsed_rgb.dedans()]);

                    // Drop it straight onto the canvas exactly where the user right-clicked their mouse
                    let drop_spot = ui
                        .ctx()
                        .input(|i| i.pointer.interact_pos().unwrap_or(field_rect.center()));
                    let new_avg = self.add_spatial_sample(ui.ctx(), color_image, drop_spot);

                    color_update_signal = Some(new_avg);

                    // Flush the working string buffer on a successful match
                    text_buffer = String::new();
                    ui.ctx()
                        .data_mut(|d| d.insert_temp(buffer_key, text_buffer.clone()));
                }
                ui.close();
            }

            if let Some((target_pos, cache_idx)) = hovered_patch_origin {
                ui.separator();
                if ui.button("❌ Remove Hovered Sample").clicked() {
                    if self.samples.remove_image_at(target_pos).is_some() {
                        _ = self.texture_render_cache.remove(cache_idx);
                        color_update_signal = Some(self.samples.average_colour());
                    }
                    ui.close();
                }
            }

            if !self.samples.0.is_empty() {
                ui.separator();
                if ui.button("🗑️ Remove All Samples").clicked() {
                    self.clear();
                    color_update_signal = Some(colour_math::rgb::RGB::<u8>::default());
                    ui.close();
                }
            }
        });

        (response, color_update_signal)
    }
}
