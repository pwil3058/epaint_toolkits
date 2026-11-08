use colour_math::{Colour, Draw, FDRNum, HCV, Point, Prop, Size, TextPosn, UFDRNum};
use egui::{Align2, Color32, Painter, Pos2, Vec2};
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

impl FromColourMath<Size> for Vec2 {
    fn from_cm(size: Size) -> Vec2 {
        Vec2 {
            x: f32::from(size.width),
            y: f32::from(size.height),
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

impl FromColourMath<TextPosn> for (Pos2, Align2) {
    fn from_cm(text_posn: TextPosn) -> Self {
        match text_posn {
            TextPosn::Centre(p) => (p.cm_into(), egui::Align2::CENTER_CENTER),
            TextPosn::TopLeftCorner(p) => (p.cm_into(), egui::Align2::LEFT_TOP),
            TextPosn::TopRightCorner(p) => (p.cm_into(), egui::Align2::RIGHT_TOP),
            TextPosn::BottomLeftCorner(p) => (p.cm_into(), egui::Align2::LEFT_BOTTOM),
            TextPosn::BottomRightCorner(p) => (p.cm_into(), egui::Align2::RIGHT_BOTTOM),
        }
    }
}

pub struct Drawer {
    pub painter: Painter,
    size: Vec2,
    fill_colour: Cell<Color32>,
    line_colour: Cell<Color32>,
    text_colour: Cell<Color32>,
    line_width: Cell<f32>,
}

impl Draw for Drawer {
    fn size(&self) -> Size {
        self.size.into_cm()
    }

    fn draw_polygon(&self, polygon: &[Point], fill: bool) {
        let points: Vec<Pos2> = polygon.iter().map(|p| Pos2::from_cm(*p)).collect();
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
        self.line_width.set(f32::from(width));
    }

    fn draw_line(&self, line: &[Point]) {
        let points: Vec<Pos2> = line.iter().map(|p| Pos2::from_cm(*p)).collect();
        if points.len() > 1 {
            let stroke = egui::Stroke::new(self.line_width.get(), self.line_colour.get());
            self.painter.add(egui::Shape::line(points, stroke));
        }
    }

    fn draw_text(&self, _text: &str, _posn: TextPosn, _font_size: UFDRNum) {
        unreachable!();
    }

    fn paint_linear_gradient(&self, _posn: Point, _size: Size, _colour_stops: &[(HCV, Prop)]) {
        unreachable!();
    }
}
