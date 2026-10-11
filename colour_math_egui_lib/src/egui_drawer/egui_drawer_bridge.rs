// Copyright (c) 2026 Peter Williams <pwil3058@bigpond.net.au> <pwil3058@gmail.com>.
use egui::{Color32, Painter, Pos2, Stroke, Vec2};

// A wrapper struct to bridge egui context with your drawing backend
pub struct EguiDrawBridge {
    pub painter: Painter,
    pub center_offset: Pos2, // To handle coordinates relative to the widget center
    pub scale_factor: f32,   // Maps your normalized UFDRNumber coordinates to actual pixels
}

impl Draw for EguiDrawBridge {
    fn size(&self) -> Size {
        let rect = self.painter.clip_rect();
        // Convert egui's rect width/height back to your UFDRNumber space
        Size {
            width: UFDRNumber::from(rect.width() as f64),
            height: UFDRNumber::from(rect.height() as f64),
        }
    }

    fn draw_polygon(&self, polygon: &[Point], fill: bool) {
        // Map your Point slice to egui Pos2 vectors
        let points: Vec<Pos2> = polygon
            .iter()
            .map(|p| {
                Pos2::new(
                    self.center_offset.x + (f64::from(p.x) as f32 * self.scale_factor),
                    self.center_offset.y + (f64::from(p.y) as f32 * self.scale_factor),
                )
            })
            .collect();

        if fill {
            // egui uses Color32 for filling shapes
            self.painter.add(egui::Shape::convex_polygon(
                points,
                self.current_fill_color(), // Map your ColourBasics to egui Color32
                Stroke::NONE,
            ));
        } else {
            // Draw closed line loop if it's an unfilled outline
            self.painter.add(egui::Shape::closed_line(
                points,
                Stroke::new(self.current_line_width(), self.current_line_color()),
            ));
        }
    }

    // ... Implement set_line_width, draw_line, and colour states ...
}

impl DrawShapes for EguiDrawBridge {
    fn set_background_colour(&self, colour: &impl ColourBasics) {
        // Draw a giant filled rect over the widget area
        let rect = self.painter.clip_rect();
        self.painter
            .rect_filled(rect, 0.0, your_colour_to_egui(colour));
    }

    fn draw_circle(&self, centre: Point, radius: UFDRNumber, fill: bool) {
        let egui_centre = Pos2::new(
            self.center_offset.x + (f64::from(centre.x) as f32 * self.scale_factor),
            self.center_offset.y + (f64::from(centre.y) as f32 * self.scale_factor),
        );
        let egui_radius = f64::from(radius) as f32 * self.scale_factor;

        if fill {
            self.painter
                .circle_filled(egui_centre, egui_radius, self.current_fill_color());
        } else {
            self.painter.circle_stroke(
                egui_centre,
                egui_radius,
                Stroke::new(self.current_line_width(), self.current_line_color()),
            );
        }
    }
}
// egui::CentralPanel::default().show(ctx, |ui| {
// Allocate space for the custom color widget box
// let desired_size = egui::vec2(400.0, 400.0);
// let (rect, response) = ui.allocate_exact_size(desired_size, egui::Sense::click_and_drag());
//
// Initialize your paint bridge mapping your struct variables
// let bridge = EguiDrawBridge {
// painter: ui.painter_at(rect),
// center_offset: rect.center(),
// scale_factor: rect.width() / 2.0, // Scale normalized -1.0..1.0 bounds to screen pixels
// };
//
// Draw your entire domain logic unchanged!
// my_hue_wheel.draw(ScalarAttribute::Chroma, &bridge);
//
// Easy interaction handling using egui coordinates mapped back to your Point space
// if let Some(pointer_pos) = response.interact_pointer_pos() {
// let local_x = (pointer_pos.x - rect.center().x) / bridge.scale_factor;
// let local_y = (pointer_pos.y - rect.center().y) / bridge.scale_factor;
// let click_point = Point { x: local_x.into(), y: local_y.into() };
//
// Handle hovering, selection or tooltips natively
// if let Some(shape) = my_hue_wheel.item_at_point(click_point, ScalarAttribute::Chroma) {
// response.on_hover_text(shape.tooltip_text());
// }
// }
// });
