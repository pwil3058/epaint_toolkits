// Copyright (c) 2026 Peter Williams <pwil3058@bigpond.net.au> <pwil3058@gmail.com>.

use eframe::egui;

pub struct ValueSlider<'a> {
    pub current_val: &'a f32,
    pub target_val: Option<&'a f32>,
    pub min_bound: f32,
    pub max_bound: f32,
    pub bg_start_color: egui::Color32,
    pub bg_end_color: egui::Color32,
}

impl<'a> ValueSlider<'a> {
    pub fn new(current_val: &'a f32) -> Self {
        Self {
            current_val,
            target_val: None,
            min_bound: 0.0,
            max_bound: 1.0,
            bg_start_color: egui::Color32::BLACK,
            bg_end_color: egui::Color32::WHITE,
        }
    }

    pub fn target(mut self, target: &'a f32) -> Self {
        self.target_val = Some(target);
        self
    }

    pub fn bounds(mut self, min: f32, max: f32) -> Self {
        self.min_bound = min;
        self.max_bound = max;
        self
    }

    pub fn gradient(mut self, start: egui::Color32, end: egui::Color32) -> Self {
        self.bg_start_color = start;
        self.bg_end_color = end;
        self
    }

    /// Renders the linear bounded strip returning interaction metrics.
    pub fn show(self, ui: &mut egui::Ui, dimensions: egui::Vec2) -> egui::Response {
        let (rect, response) = ui.allocate_exact_size(dimensions, egui::Sense::click_and_drag());
        let painter = ui.painter();

        // 1. Paint the value gradient using pure mesh quads to lock down version safety completely!
        let mut mesh = egui::Mesh::default();
        mesh.vertices = vec![
            egui::epaint::Vertex {
                pos: rect.left_top(),
                uv: egui::pos2(0.0, 0.0),
                color: self.bg_start_color,
            },
            egui::epaint::Vertex {
                pos: rect.right_top(),
                uv: egui::pos2(0.0, 0.0),
                color: self.bg_end_color,
            },
            egui::epaint::Vertex {
                pos: rect.right_bottom(),
                uv: egui::pos2(0.0, 0.0),
                color: self.bg_end_color,
            },
            egui::epaint::Vertex {
                pos: rect.left_bottom(),
                uv: egui::pos2(0.0, 0.0),
                color: self.bg_start_color,
            },
        ];
        mesh.add_triangle(0, 1, 2);
        mesh.add_triangle(0, 2, 3);
        painter.add(egui::Shape::mesh(mesh));

        // Helper closure mapping bounded values to horizontal screen pixel coordinates
        let value_to_screen_x = |val: f32| -> f32 {
            let normalized = (val - self.min_bound) / (self.max_bound - self.min_bound);
            let clamped = normalized.clamp(0.0, 1.0);
            rect.left() + clamped * rect.width()
        };

        // 2. Render the static current value reference line marker
        let current_x = value_to_screen_x(*self.current_val);
        painter.line_segment(
            [
                egui::pos2(current_x, rect.top() - 1.0),
                egui::pos2(current_x, rect.bottom() + 1.0),
            ],
            egui::Stroke::new(2.0, egui::Color32::WHITE),
        );

        // 3. Render the target value triangle cursor pointer if one is set
        if let Some(target) = self.target_val {
            let target_x = value_to_screen_x(*target);

            // 🌟 FIX: Updated method to the verified egui::Shape::convex_polygon factory wrapper!
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

        response
    }
}
