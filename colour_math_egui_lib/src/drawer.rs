// Copyright (c) 2026 Peter Williams <pwil3058@bigpond.net.au> <pwil3058@gmail.com>.
use colour_math::{
    Colour, Draw, DrawIsosceles, DrawShapes, FDRNum, FromProp, HCV, Point, Prop, Size, TextPosn,
    UFDRNum,
};
use egui::{Align2, Color32, Painter, Pos2, Rect, Vec2};
use std::cell::Cell;

use crate::colour::*;
use crate::{ColourMathFrom, ColourMathInto, FromColourMath, IntoColourMath};

impl FromColourMath<Point> for Pos2 {
    fn from_cm(point: Point) -> Self {
        Pos2 {
            x: f32::from(point.x),
            y: f32::from(point.y),
        }
    }
}

impl ColourMathFrom<Pos2> for Point {
    fn cm_from(pos: Pos2) -> Point {
        Point {
            x: FDRNum::from(pos.x),
            y: FDRNum::from(pos.y),
        }
    }
}

impl ColourMathFrom<Vec2> for Point {
    fn cm_from(vec: Vec2) -> Point {
        Point {
            x: FDRNum::from(vec.x),
            y: FDRNum::from(vec.y),
        }
    }
}

impl FromColourMath<Size> for Vec2 {
    fn from_cm(size: Size) -> Vec2 {
        Vec2 {
            x: f32::from(size.width),
            y: f32::from(size.height),
        }
    }
}

impl FromColourMath<Point> for Vec2 {
    fn from_cm(point: Point) -> Vec2 {
        Vec2 {
            x: f32::from(point.x),
            y: f32::from(point.y),
        }
    }
}

impl ColourMathFrom<Vec2> for Size {
    fn cm_from(vec: Vec2) -> Self {
        Self {
            width: UFDRNum::from(vec.x),
            height: UFDRNum::from(vec.y),
        }
    }
}

impl ColourMathFrom<Rect> for Size {
    fn cm_from(rect: Rect) -> Self {
        Self {
            width: UFDRNum::from((rect.max.x - rect.min.x).abs()),
            height: UFDRNum::from((rect.max.y - rect.min.y).abs()),
        }
    }
}

impl FromColourMath<TextPosn> for (Point, Align2) {
    fn from_cm(text_posn: TextPosn) -> Self {
        match text_posn {
            TextPosn::Centre(p) => (p, egui::Align2::CENTER_CENTER),
            TextPosn::TopLeftCorner(p) => (p, egui::Align2::LEFT_TOP),
            TextPosn::TopRightCorner(p) => (p, egui::Align2::RIGHT_TOP),
            TextPosn::BottomLeftCorner(p) => (p, egui::Align2::LEFT_BOTTOM),
            TextPosn::BottomRightCorner(p) => (p, egui::Align2::RIGHT_BOTTOM),
        }
    }
}

pub struct Drawer {
    pub painter: Painter,
    size: Size,
    rect: Rect,
    scale: f32,
    offset_centre: Option<Pos2>,
    fill_colour: Cell<Color32>,
    line_colour: Cell<Color32>,
    text_colour: Cell<Color32>,
    line_width: Cell<f32>,
}

impl Drawer {
    pub fn new(painter: Painter, offset: Option<Vec2>) -> Self {
        let rect = painter.clip_rect();
        let size = Size::cm_from(rect);
        let scale = if size.height > size.width {
            f32::from(size.width) / 2.15
        } else {
            f32::from(size.height) / 2.15
        };
        let offset_centre = if let Some(offset) = offset {
            Some(rect.center() + offset)
        } else {
            None
        };
        Self {
            painter,
            size,
            rect,
            scale,
            offset_centre,
            fill_colour: Cell::new(Color32::BLACK),
            line_colour: Cell::new(Color32::BLACK),
            text_colour: Cell::new(Color32::WHITE),
            line_width: Cell::new(1.0),
        }
    }

    pub fn transform_point(&self, point: Point) -> Pos2 {
        if let Some(offset_centre) = self.offset_centre {
            offset_centre + Vec2::from_cm(point) * self.scale
        } else {
            self.rect.min + Vec2::from_cm(point)
        }
    }

    pub fn transform_pos2(&self, pos: Pos2) -> Point {
        if let Some(offset_centre) = self.offset_centre {
            ((pos - offset_centre) / self.scale).into_cm()
        } else {
            Point::default()
        }
    }

    pub fn transform_points(&self, points: &[Point]) -> Vec<Pos2> {
        points.iter().map(|p| self.transform_point(*p)).collect()
    }

    pub fn rectangle(&self, point: Point, size: Size) -> Rect {
        let pos = self.transform_point(point);
        let min = Pos2 { x: pos.x, y: pos.y };
        let max = Pos2 {
            x: pos.x + f32::from(size.width),
            y: pos.y + f32::from(size.height),
        };
        Rect { min, max }
    }
}

impl Draw for Drawer {
    fn size(&self) -> Size {
        self.size
    }

    fn draw_polygon(&self, polygon: &[Point], fill: bool) {
        if polygon.len() > 1 {
            let points = self.transform_points(polygon);
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

    fn set_fill_colour(&self, colour: &impl Colour) {
        self.fill_colour.set(colour.into_rgb::<u8>().into_color32());
    }

    fn set_line_colour(&self, colour: &impl Colour) {
        self.line_colour.set(colour.into_rgb::<u8>().into_color32());
    }
    fn set_text_colour(&self, colour: &impl Colour) {
        self.text_colour.set(colour.into_rgb::<u8>().into_color32());
    }

    fn set_line_width(&self, width: UFDRNum) {
        self.line_width.set(f32::from(width) * self.scale);
    }

    fn draw_line(&self, line: &[Point]) {
        if line.len() > 1 {
            let points = self.transform_points(line);
            let stroke = egui::Stroke::new(self.line_width.get(), self.line_colour.get());
            self.painter.add(egui::Shape::line(points, stroke));
        }
    }

    fn draw_text(&self, text: &str, posn: TextPosn, font_size: UFDRNum) {
        if text.is_empty() {
            return;
        };

        let (point, align) = posn.cm_into();

        self.painter.text(
            self.transform_point(point),
            align,
            text,
            egui::FontId::monospace(f32::from(font_size)),
            self.text_colour.get(),
        );
    }

    fn paint_linear_gradient(&self, posn: Point, size: Size, colour_stops: &[(HCV, Prop)]) {
        if colour_stops.is_empty() {
            return;
        }

        let mut mesh = egui::Mesh::default();
        let rect = self.rectangle(posn, size);

        for window in colour_stops.windows(2) {
            let (c1, p1) = window[0];
            let (c2, p2) = window[1];
            let x1 = rect.left() + rect.width() * f64::from_prop(p1) as f32;
            let x2 = rect.left() + rect.width() * f64::from_prop(p2) as f32;

            let col1 = c1.into_color32();
            let col2 = c2.into_color32();
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

impl DrawIsosceles for Drawer {}

impl<'a> DrawShapes for Drawer {
    fn set_background_colour(&self, colour: &impl Colour) {
        self.painter
            .rect_filled(self.rect, 0.0, colour.into_rgb::<u8>().into_color32());
    }

    fn draw_circle(&self, centre: colour_math::beigui::Point, radius: UFDRNum, fill: bool) {
        let screen_center = self.transform_point(centre);
        let screen_radius = f32::from(radius) * self.scale;

        if fill {
            self.painter
                .circle_filled(screen_center, screen_radius, self.fill_colour.get());
        } else {
            let stroke = egui::Stroke::new(self.line_width.get(), self.line_colour.get());
            self.painter
                .circle_stroke(screen_center, screen_radius, stroke);
        }
    }
}
