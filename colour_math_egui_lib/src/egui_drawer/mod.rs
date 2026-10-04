// Copyright (c) 2026 Peter Williams <pwil3058@bigpond.net.au> <pwil3058@gmail.com>.

use colour_math::{
    ColourBasics,
    beigui::{Draw, DrawIsosceles, DrawShapes, TextPosn},
    fdrn::UFDRNumber,
};
use eframe::egui;
use std::cell::Cell;

/// Bridges your core domain traits with an immediate-mode egui Viewport Painter.
pub struct EguiDrawer<'a> {
    pub painter: &'a egui::Painter,
    pub rect: egui::Rect,
    pub canvas_rect: egui::Rect, // Fixed widget rect (for the stable square background)
    pub scale: f64,
    fill_colour: Cell<egui::Color32>,
    line_colour: Cell<egui::Color32>,
    text_colour: Cell<egui::Color32>,
    line_width: Cell<f32>,
}

impl<'a> EguiDrawer<'a> {
    pub fn new(painter: &'a egui::Painter, rect: egui::Rect, canvas_rect: egui::Rect) -> Self {
        // Differentiate layout rows by absolute pixel height instead of variable width aspect ratios
        let scale = if rect.height() < 32.0 {
            rect.height() as f64 / 2.0 // Lock indicator strips to prevent shapes from blowing out
        } else {
            rect.height() as f64 / 2.15 // Safe Cartesian plane scalar for square canvas widgets
        };

        Self {
            painter,
            rect,
            canvas_rect,
            scale,
            fill_colour: Cell::new(egui::Color32::BLACK),
            line_colour: Cell::new(egui::Color32::BLACK),
            text_colour: Cell::new(egui::Color32::WHITE),
            line_width: Cell::new(1.0),
        }
    }

    /// Map a domain Point [-1.0..=1.0] straight to absolute pixel coordinates on screen.
    fn to_egui_pos(&self, point: colour_math::beigui::Point) -> egui::Pos2 {
        let center = self.rect.center();

        // 🌟 FIX A: If we are handling a short slider row, the X coordinate represents an
        // absolute horizontal pixel offset fraction across the bar instead of a scaled Cartesian point.
        let (target_x, target_y) = if self.rect.height() < 32.0 {
            let x_pixel = self.rect.left() + (f64::from(point.x) as f32);
            let y_pixel = self.rect.top() + (f64::from(point.y) as f32);
            (x_pixel, y_pixel)
        } else {
            let x_offset = f64::from(point.x) * self.scale;
            let y_offset = -f64::from(point.y) * self.scale;
            (
                (center.x as f64 + x_offset) as f32,
                (center.y as f64 + y_offset) as f32,
            )
        };

        // Clamp the calculated screen point strictly within the current row rect
        egui::pos2(
            target_x.clamp(self.rect.left(), self.rect.right()),
            target_y.clamp(self.rect.top(), self.rect.bottom()),
        )
    }

    fn to_egui_color(colour: &impl ColourBasics) -> egui::Color32 {
        let prop_array = <[colour_math::fdrn::Prop; 3]>::from(colour.hcv());
        let r_f64 = f64::from(prop_array[0]);
        let g_f64 = f64::from(prop_array[1]);
        let b_f64 = f64::from(prop_array[2]);

        egui::Color32::from_rgb(
            (r_f64 * 255.0).clamp(0.0, 255.0) as u8,
            (g_f64 * 255.0).clamp(0.0, 255.0) as u8,
            (b_f64 * 255.0).clamp(0.0, 255.0) as u8,
        )
    }
}

impl<'a> Draw for EguiDrawer<'a> {
    fn size(&self) -> colour_math::beigui::Size {
        let size = self.rect.size();
        [
            UFDRNumber::from(size.x as f64),
            UFDRNumber::from(size.y as f64),
        ]
        .into()
    }

    fn set_fill_colour(&self, colour: &impl ColourBasics) {
        self.fill_colour.set(Self::to_egui_color(colour));
    }

    fn set_line_colour(&self, colour: &impl ColourBasics) {
        self.line_colour.set(Self::to_egui_color(colour));
    }

    fn set_text_colour(&self, colour: &impl ColourBasics) {
        self.text_colour.set(Self::to_egui_color(colour));
    }

    fn set_line_width(&self, width: UFDRNumber) {
        if self.rect.height() < 32.0 {
            self.line_width.set(f64::from(width) as f32);
        } else {
            self.line_width
                .set(f64::from(width) as f32 * self.scale as f32);
        }
    }

    fn draw_polygon(&self, polygon: &[colour_math::beigui::Point], fill: bool) {
        let points: Vec<egui::Pos2> = polygon.iter().map(|p| self.to_egui_pos(*p)).collect();
        if points.len() > 1 {
            if fill {
                self.painter.add(egui::Shape::convex_polygon(
                    points,
                    self.fill_colour.get(),
                    egui::Stroke::NONE,
                ));
            } else {
                let stroke = egui::Stroke::new(self.line_width.get(), self.line_colour.get());
                self.painter.add(egui::Shape::closed_line(points, stroke));
            }
        }
    }

    fn draw_line(&self, line: &[colour_math::beigui::Point]) {
        let points: Vec<egui::Pos2> = line.iter().map(|p| self.to_egui_pos(*p)).collect();
        if points.len() > 1 {
            let stroke = egui::Stroke::new(self.line_width.get(), self.line_colour.get());
            self.painter.add(egui::Shape::line(points, stroke));
        }
    }

    fn draw_text(&self, text: &str, posn: TextPosn, font_size: UFDRNumber) {
        if text.is_empty() {
            return;
        }

        let (target_point, align) = match posn {
            TextPosn::Centre(p) => (p, egui::Align2::CENTER_CENTER),
            TextPosn::TopLeftCorner(p) => (p, egui::Align2::LEFT_TOP),
            TextPosn::TopRightCorner(p) => (p, egui::Align2::RIGHT_TOP),
            TextPosn::BottomLeftCorner(p) => (p, egui::Align2::LEFT_BOTTOM),
            TextPosn::BottomRightCorner(p) => (p, egui::Align2::RIGHT_BOTTOM),
        };

        let screen_pos = self.to_egui_pos(target_point);

        // 🌟 FIX B: Scale text sizes cleanly depending on row context
        let size_pixels = if self.rect.height() < 32.0 {
            f64::from(font_size)
        } else {
            f64::from(font_size) * self.scale
        };

        self.painter.text(
            screen_pos,
            align,
            text,
            egui::FontId::monospace(size_pixels as f32),
            self.text_colour.get(),
        );
    }

    fn paint_linear_gradient(
        &self,
        _posn: colour_math::beigui::Point,
        _size: colour_math::beigui::Size,
        colour_stops: &[(colour_math::hcv::HCV, colour_math::fdrn::Prop)],
    ) {
        if colour_stops.is_empty() {
            return;
        }

        let mut mesh = egui::Mesh::default();
        let rect = self.rect;

        for i in 0..(colour_stops.len() - 1) {
            let (c1, p1) = &colour_stops[i];
            let (c2, p2) = &colour_stops[i + 1];

            let x1 = rect.left() + rect.width() * f64::from(*p1) as f32;
            let x2 = rect.left() + rect.width() * f64::from(*p2) as f32;

            let col1 = Self::to_egui_color(c1);
            let col2 = Self::to_egui_color(c2);
            let uv = egui::epaint::WHITE_UV;

            let idx = mesh.vertices.len() as u32;
            mesh.vertices.push(egui::epaint::Vertex {
                pos: egui::pos2(x1, rect.top()),
                uv,
                color: col1,
            });
            mesh.vertices.push(egui::epaint::Vertex {
                pos: egui::pos2(x2, rect.top()),
                uv,
                color: col2,
            });
            mesh.vertices.push(egui::epaint::Vertex {
                pos: egui::pos2(x2, rect.bottom()),
                uv,
                color: col2,
            });
            mesh.vertices.push(egui::epaint::Vertex {
                pos: egui::pos2(x1, rect.bottom()),
                uv,
                color: col1,
            });

            mesh.add_triangle(idx, idx + 1, idx + 2);
            mesh.add_triangle(idx, idx + 2, idx + 3);
        }
        self.painter.add(egui::Shape::mesh(mesh));
    }
}

impl<'a> DrawIsosceles for EguiDrawer<'a> {}

impl<'a> DrawShapes for EguiDrawer<'a> {
    fn set_background_colour(&self, colour: &impl ColourBasics) {
        self.painter
            .rect_filled(self.canvas_rect, 0.0, Self::to_egui_color(colour));
    }

    fn draw_circle(&self, centre: colour_math::beigui::Point, radius: UFDRNumber, fill: bool) {
        let screen_center = self.to_egui_pos(centre);
        let screen_radius = if self.rect.height() < 32.0 {
            f64::from(radius)
        } else {
            f64::from(radius) * self.scale
        };

        if fill {
            self.painter
                .circle_filled(screen_center, screen_radius as f32, self.fill_colour.get());
        } else {
            let stroke = egui::Stroke::new(self.line_width.get(), self.line_colour.get());
            self.painter
                .circle_stroke(screen_center, screen_radius as f32, stroke);
        }
    }
}
