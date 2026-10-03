// Copyright (c) 2026 Peter Williams <pwil3058@bigpond.net.au> <pwil3058@gmail.com>.

use colour_math::{ColourBasics, Hue, HueConstants, Prop};
use eframe::egui;

pub struct SpectrumSlider<'a> {
    pub current_hue: &'a mut Hue, // Mutable so dragging shifts the value
    pub target_hue: Option<&'a Hue>,
    pub center_on_target: bool,
}

impl<'a> SpectrumSlider<'a> {
    pub fn new(current_hue: &'a mut Hue) -> Self {
        Self {
            current_hue,
            target_hue: None,
            center_on_target: false,
        }
    }

    pub fn target(mut self, target: &'a Hue) -> Self {
        self.target_hue = Some(target);
        self
    }

    pub fn center_on_target(mut self, center: bool) -> Self {
        self.center_on_target = center;
        self
    }

    /// Renders the rolling spectrum ribbon returning standard interaction metrics.
    pub fn show(self, ui: &mut egui::Ui, dimensions: egui::Vec2) -> egui::Response {
        let (rect, response) = ui.allocate_exact_size(dimensions, egui::Sense::click_and_drag());
        let painter = ui.painter();

        // 1. Establish our fixed centre reference point using your custom Hue models
        let fixed_center_hue = if self.center_on_target {
            self.target_hue.copied().unwrap_or(*self.current_hue)
        } else {
            *self.current_hue
        };

        // 2. INTERCEPT ACTIVE CLICK-AND-DRAG INTERACTIONS
        if response.dragged() {
            let drag_delta_x = response.drag_delta().x;
            if drag_delta_x != 0.0 {
                let degrees_delta = -(drag_delta_x as f64 / rect.width() as f64) * 360.0;
                let angle_delta = colour_math::hue::angle::Angle::from(degrees_delta);
                *self.current_hue = *self.current_hue + angle_delta;
                ui.ctx().request_repaint();
            }
        }

        // 3. PAINT SPECTRUM RIBBON VIA PURE GRAPHICS MESH QUADS
        let red_arr = <[Prop; 3]>::from(colour_math::HCV::RED);
        let yellow_arr = <[Prop; 3]>::from(colour_math::HCV::YELLOW);
        let blue_arr = <[Prop; 3]>::from(colour_math::HCV::BLUE);

        let color_start = egui::Color32::from_rgb(
            u8::from(red_arr[0]),
            u8::from(red_arr[1]),
            u8::from(red_arr[2]),
        );
        let color_mid = egui::Color32::from_rgb(
            u8::from(yellow_arr[0]),
            u8::from(yellow_arr[1]),
            u8::from(yellow_arr[2]),
        );
        let color_end = egui::Color32::from_rgb(
            u8::from(blue_arr[0]),
            u8::from(blue_arr[1]),
            u8::from(blue_arr[2]),
        );

        let mut left_mesh = egui::Mesh::default();
        left_mesh.vertices = vec![
            egui::epaint::Vertex {
                pos: rect.left_top(),
                uv: egui::pos2(0.0, 0.0),
                color: color_start,
            },
            egui::epaint::Vertex {
                pos: egui::pos2(rect.center().x, rect.top()),
                uv: egui::pos2(0.0, 0.0),
                color: color_mid,
            },
            egui::epaint::Vertex {
                pos: egui::pos2(rect.center().x, rect.bottom()),
                uv: egui::pos2(0.0, 0.0),
                color: color_mid,
            },
            egui::epaint::Vertex {
                pos: rect.left_bottom(),
                uv: egui::pos2(0.0, 0.0),
                color: color_start,
            },
        ];
        left_mesh.add_triangle(0, 1, 2);
        left_mesh.add_triangle(0, 2, 3);
        painter.add(egui::Shape::mesh(left_mesh));

        let mut right_mesh = egui::Mesh::default();
        right_mesh.vertices = vec![
            egui::epaint::Vertex {
                pos: egui::pos2(rect.center().x, rect.top()),
                uv: egui::pos2(0.0, 0.0),
                color: color_mid,
            },
            egui::epaint::Vertex {
                pos: rect.right_top(),
                uv: egui::pos2(0.0, 0.0),
                color: color_end,
            },
            egui::epaint::Vertex {
                pos: rect.right_bottom(),
                uv: egui::pos2(0.0, 0.0),
                color: color_end,
            },
            egui::epaint::Vertex {
                pos: egui::pos2(rect.center().x, rect.bottom()),
                uv: egui::pos2(0.0, 0.0),
                color: color_mid,
            },
        ];
        right_mesh.add_triangle(0, 1, 2);
        right_mesh.add_triangle(0, 2, 3);
        painter.add(egui::Shape::mesh(right_mesh));

        // 4. Render the static, fixed center reference marker line
        let center_x = rect.left() + 0.5 * rect.width();
        painter.line_segment(
            [
                egui::pos2(center_x, rect.top() - 2.0),
                egui::pos2(center_x, rect.bottom() + 2.0),
            ],
            egui::Stroke::new(2.0, egui::Color32::WHITE),
        );

        // 5. Render the rolling target triangle cursor if it drifts within view
        if let Some(target) = self.target_hue {
            if !self.center_on_target {
                let angle_distance = *target - fixed_center_hue;
                let degrees_diff = f64::from(angle_distance);
                let target_fractional_x = 0.5 + (degrees_diff / 360.0) as f32;

                if target_fractional_x >= 0.0 && target_fractional_x <= 1.0 {
                    let target_x = rect.left() + target_fractional_x * rect.width();

                    // 🌟 FIX: Employs the authentic egui 0.36.2 convex_polygon structural variant!
                    painter.add(egui::Shape::convex_polygon(
                        vec![
                            egui::pos2(target_x - 5.0, rect.top() - 4.0),
                            egui::pos2(target_x + 5.0, rect.top() - 4.0),
                            egui::pos2(target_x, rect.top() + 2.0),
                        ],
                        egui::Color32::BLACK,
                        egui::Stroke::NONE,
                    ));
                }
            }
        }

        response
    }
}
