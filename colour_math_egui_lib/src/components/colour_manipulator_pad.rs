// Complete refactor of colour_math_egui_lib/src/components/colour_manipulator_pad.rs
// Copyright (c) 2026 Peter Williams <pwil3058@bigpond.net.au> <pwil3058@gmail.com>.

use colour_math::{
    ColourBasics,
    manipulator::{ColourManipulator, DeltaSize},
    rgb::RGB,
};
use eframe::egui;

#[derive(Clone, Copy, Debug, Default)]
pub struct ColourManipulatorPad;

impl ColourManipulatorPad {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn show(
        &mut self,
        ui: &mut egui::Ui,
        manipulator: &mut ColourManipulator,
        texture: &mut Option<egui::TextureHandle>,
    ) {
        let content_width = 220.0;
        let content_height = 220.0;
        let side_btn_width = 32.0;
        let button_height = 24.0;

        let delta_size = ui.input(|i| {
            if i.modifiers.ctrl {
                DeltaSize::Small
            } else if i.modifiers.shift {
                DeltaSize::Large
            } else {
                DeltaSize::Normal
            }
        });

        ui.vertical_centered(|ui| {
            // 1. Value++ Button
            ui.with_layout(egui::Layout::top_down(egui::Align::Center), |ui| {
                if ui
                    .add_sized([content_width, button_height], egui::Button::new("Value++"))
                    .clicked()
                {
                    manipulator.incr_value(delta_size.for_value());
                }
            });

            ui.add_space(4.0);

            // 2. Central Row: Left Hue, Central Field with Context Menus, Right Hue
            ui.horizontal(|ui| {
                let total_row_width = content_width + (side_btn_width * 2.0) + 8.0;
                let left_margin = (ui.available_width() - total_row_width) / 2.0;
                ui.add_space(left_margin.max(0.0));

                if ui
                    .add_sized([side_btn_width, content_height], egui::Button::new("<"))
                    .clicked()
                {
                    manipulator.rotate(delta_size.for_hue_clockwise());
                }

                ui.add_space(4.0);

                let (field_rect, response) = ui.allocate_exact_size(
                    egui::vec2(content_width, content_height),
                    egui::Sense::click(),
                );

                // Draw ground-truth base solid color block behind
                let current_rgb_u64 = RGB::<u64>::from(manipulator.hcv());
                let rgb_channels = current_rgb_u64.rgb::<u8>();
                ui.painter().rect_filled(
                    field_rect,
                    4.0,
                    egui::Color32::from_rgb(rgb_channels[0], rgb_channels[1], rgb_channels[2]),
                );

                if let Some(tex) = texture {
                    // Enforce actual-size rendering centered perfectly inside our field box footprint
                    let actual_texture_size = tex.size_vec2();
                    let centered_image_rect = egui::Rect::from_center_size(
                        field_rect.center(),
                        egui::vec2(
                            actual_texture_size.x.min(content_width),
                            actual_texture_size.y.min(content_height),
                        ),
                    );

                    ui.painter().image(
                        tex.id(),
                        centered_image_rect,
                        egui::Rect::from_min_max(egui::pos2(0.0, 0.0), egui::pos2(1.0, 1.0)),
                        egui::Color32::WHITE,
                    );

                    ui.painter().rect_stroke(
                        centered_image_rect,
                        0.0,
                        egui::Stroke::new(1.0, egui::Color32::from_gray(180)),
                        egui::StrokeKind::Outside,
                    );
                } else {
                    // Clean stroke border layout outline when no sample is active
                    ui.painter().rect_stroke(
                        field_rect,
                        4.0,
                        egui::Stroke::new(1.0, egui::Color32::from_gray(64)),
                        egui::StrokeKind::Middle,
                    );
                }

                response.context_menu(|ui| {
                    if ui.button("❌ Delete Active Sample").clicked() {
                        *texture = None;
                    }
                });

                ui.add_space(4.0);

                if ui
                    .add_sized([side_btn_width, content_height], egui::Button::new(">"))
                    .clicked()
                {
                    manipulator.rotate(delta_size.for_hue_anticlockwise());
                }
            });

            ui.add_space(4.0);

            // 3. Value-- Button
            ui.with_layout(egui::Layout::top_down(egui::Align::Center), |ui| {
                if ui
                    .add_sized([content_width, button_height], egui::Button::new("Value--"))
                    .clicked()
                {
                    manipulator.decr_value(delta_size.for_value());
                }
            });

            ui.add_space(6.0);

            // 4. Footer Chroma tuning row
            ui.horizontal(|ui| {
                let half_row_width = (content_width + (side_btn_width * 2.0)) / 2.0;
                let footer_margin =
                    (ui.available_width() - (content_width + (side_btn_width * 2.0) + 4.0)) / 2.0;
                ui.add_space(footer_margin.max(0.0));

                if ui
                    .add_sized(
                        [half_row_width, button_height],
                        egui::Button::new("Chroma-- / Greyness++"),
                    )
                    .clicked()
                {
                    manipulator.decr_chroma(delta_size.for_chroma());
                }
                ui.add_space(4.0);
                if ui
                    .add_sized(
                        [half_row_width, button_height],
                        egui::Button::new("Chroma++ / Greyness--"),
                    )
                    .clicked()
                {
                    manipulator.incr_chroma(delta_size.for_chroma());
                }
            });

            ui.add_space(8.0);
            ui.separator();
            ui.add_space(4.0);

            // 5. Integral Automation Footer Controls
            ui.horizontal(|ui| {
                let footer_align_margin =
                    (ui.available_width() - (content_width + (side_btn_width * 2.0))) / 2.0;
                ui.add_space(footer_align_margin.max(0.0));

                // Manual AutoMatch action click trigger
                if ui.button("AutoMatch").clicked() {
                    let image_data_key = egui::Id::new("active_pasted_image_buffer_matrix");
                    if let Some(cropped_img) = ui.ctx().data_mut(|d| {
                        d.get_temp_mut_or_default::<Option<egui::ColorImage>>(image_data_key)
                            .clone()
                    }) {
                        let mut total_r: u64 = 0;
                        let mut total_g: u64 = 0;
                        let mut total_b: u64 = 0;
                        let count = cropped_img.pixels.len() as u64;

                        if count > 0 {
                            for pixel in &cropped_img.pixels {
                                let ch = pixel.to_array();
                                total_r += ch[0] as u64;
                                total_g += ch[1] as u64;
                                total_b += ch[2] as u64;
                            }
                            let avg_rgb = colour_math::rgb::RGB::<u8>::from([
                                (total_r / count) as u8,
                                (total_g / count) as u8,
                                (total_b / count) as u8,
                            ]);
                            manipulator.set_colour(&avg_rgb);
                        }
                    }
                }

                ui.add_space(16.0);

                let storage_key = egui::Id::new("on_paste_auto_toggle");
                let mut on_paste_automatic = ui
                    .ctx()
                    .data_mut(|d| *d.get_temp_mut_or_default::<bool>(storage_key));

                if ui
                    .checkbox(&mut on_paste_automatic, "On Paste? (Match Automatically)")
                    .changed()
                {
                    ui.ctx()
                        .data_mut(|d| d.insert_temp(storage_key, on_paste_automatic));
                }
            });
        });
    }
}
