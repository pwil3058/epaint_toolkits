// Copyright (c) 2026 Peter Williams <pwil3058@bigpond.net.au> <pwil3058@gmail.com>.
use std::ops::{Add, Mul, Sub};

use crate::{
    fdrn::{FDRNum, Prop, UFDRNum},
    hcv::HCV,
    hue::angle::Angle,
    ColourBasics,
};

#[cfg(test)]
mod test_beigui;

pub mod attr_display;
pub mod hue_wheel;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct Point {
    pub x: FDRNum,
    pub y: FDRNum,
}

impl Point {
    pub fn hypot(self) -> UFDRNum {
        let arg = self.x * self.x + self.y * self.y;
        UFDRNum::from(f64::from(arg).sqrt())
    }
}

impl Add for Point {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        Self {
            x: self.x.add(rhs.x),
            y: self.y.add(rhs.y),
        }
    }
}

impl Sub for Point {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self {
        Self {
            x: self.x.sub(rhs.x),
            y: self.y.sub(rhs.y),
        }
    }
}

impl Mul<UFDRNum> for Point {
    type Output = Self;

    fn mul(self, uscale: UFDRNum) -> Self {
        let scale: FDRNum = uscale.into();
        Self {
            x: self.x.mul(scale),
            y: self.y.mul(scale),
        }
    }
}

impl From<[FDRNum; 2]> for Point {
    fn from(array: [FDRNum; 2]) -> Self {
        Self {
            x: array[0],
            y: array[1],
        }
    }
}

impl From<(Angle, UFDRNum)> for Point {
    fn from((angle, radius): (Angle, UFDRNum)) -> Self {
        Self {
            x: FDRNum::from(radius) * angle.cos(),
            y: FDRNum::from(radius) * angle.sin(),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct Size {
    pub width: UFDRNum,
    pub height: UFDRNum,
}

impl Size {
    pub fn centre(&self) -> Point {
        [(self.width / 2).into(), (self.height / 2).into()].into()
    }
}

impl From<[UFDRNum; 2]> for Size {
    fn from(array: [UFDRNum; 2]) -> Self {
        Self {
            width: array[0],
            height: array[1],
        }
    }
}

/// Direction in which to draw isosceles triangle
pub enum Dirn {
    Down,
    Up,
    Right,
    Left,
}

pub enum TextPosn {
    TopLeftCorner(Point),
    TopRightCorner(Point),
    BottomLeftCorner(Point),
    BottomRightCorner(Point),
    Centre(Point),
}

pub trait Draw {
    fn size(&self) -> Size;
    fn draw_polygon(&self, polygon: &[Point], fill: bool);

    fn set_fill_colour(&self, colour: &impl ColourBasics);
    fn set_line_colour(&self, colour: &impl ColourBasics);
    fn set_text_colour(&self, colour: &impl ColourBasics);

    fn set_line_width(&self, width: UFDRNum);

    fn draw_line(&self, line: &[Point]);
    fn draw_text(&self, text: &str, posn: TextPosn, font_size: UFDRNum);

    fn paint_linear_gradient(&self, posn: Point, size: Size, colour_stops: &[(HCV, Prop)]);
}

pub trait DrawIsosceles: Draw {
    fn draw_isosceles(
        &self,
        centre: Point,
        dirn: Dirn,
        base: UFDRNum,
        height: UFDRNum,
        fill: bool,
    ) {
        let half_base = FDRNum::from(base / 2);
        let half_height = FDRNum::from(height / 2);
        let points = match dirn {
            Dirn::Up => vec![
                Point {
                    x: centre.x - half_base,
                    y: centre.y - half_height,
                },
                Point {
                    x: centre.x,
                    y: centre.y + half_height,
                },
                Point {
                    x: centre.x + half_base,
                    y: centre.y - half_height,
                },
            ],
            Dirn::Down => vec![
                Point {
                    x: centre.x - half_base,
                    y: centre.y + half_height,
                },
                Point {
                    x: centre.x,
                    y: centre.y - half_height,
                },
                Point {
                    x: centre.x + half_base,
                    y: centre.y + half_height,
                },
            ],
            Dirn::Right => vec![
                Point {
                    x: centre.x - half_height,
                    y: centre.y - half_base,
                },
                Point {
                    x: centre.x - half_height,
                    y: centre.y + half_base,
                },
                Point {
                    x: centre.x + half_height,
                    y: centre.y,
                },
            ],
            Dirn::Left => vec![
                Point {
                    x: centre.x + half_height,
                    y: centre.y - half_base,
                },
                Point {
                    x: centre.x + half_height,
                    y: centre.y + half_base,
                },
                Point {
                    x: centre.x - half_height,
                    y: centre.y,
                },
            ],
        };
        self.draw_polygon(&points, fill);
    }
}

pub trait DrawShapes: DrawIsosceles {
    fn set_background_colour(&self, colour: &impl ColourBasics);
    fn draw_circle(&self, centre: Point, radius: UFDRNum, fill: bool);

    fn draw_diamond(&self, centre: Point, side_length: UFDRNum, fill: bool) {
        let dist = FDRNum::from(side_length / 2);
        let points = vec![
            Point {
                x: centre.x,
                y: centre.y + dist,
            },
            Point {
                x: centre.x + dist,
                y: centre.y,
            },
            Point {
                x: centre.x,
                y: centre.y - dist,
            },
            Point {
                x: centre.x - dist,
                y: centre.y,
            },
        ];
        self.draw_polygon(&points, fill);
    }

    fn draw_square(&self, centre: Point, side_length: UFDRNum, fill: bool) {
        let half_side = FDRNum::from(side_length / 2);
        let points = vec![
            Point {
                x: centre.x - half_side,
                y: centre.y - half_side,
            },
            Point {
                x: centre.x - half_side,
                y: centre.y + half_side,
            },
            Point {
                x: centre.x + half_side,
                y: centre.y + half_side,
            },
            Point {
                x: centre.x + half_side,
                y: centre.y - half_side,
            },
        ];
        self.draw_polygon(&points, fill);
    }

    fn draw_equilateral(&self, centre: Point, dirn: Dirn, side_length: UFDRNum, fill: bool) {
        let half_base = FDRNum::from(side_length / 2);
        let half_height = FDRNum::from(side_length * UFDRNum::SQRT_3 / 4);
        let points = match dirn {
            Dirn::Up => vec![
                Point {
                    x: centre.x - half_base,
                    y: centre.y - half_height,
                },
                Point {
                    x: centre.x,
                    y: centre.y + half_height,
                },
                Point {
                    x: centre.x + half_base,
                    y: centre.y - half_height,
                },
            ],
            Dirn::Down => vec![
                Point {
                    x: centre.x - half_base,
                    y: centre.y + half_height,
                },
                Point {
                    x: centre.x,
                    y: centre.y - half_height,
                },
                Point {
                    x: centre.x + half_base,
                    y: centre.y + half_height,
                },
            ],
            Dirn::Right => vec![
                Point {
                    x: centre.x - half_height,
                    y: centre.y - half_base,
                },
                Point {
                    x: centre.x - half_height,
                    y: centre.y + half_base,
                },
                Point {
                    x: centre.x + half_height,
                    y: centre.y,
                },
            ],
            Dirn::Left => vec![
                Point {
                    x: centre.x + half_height,
                    y: centre.y - half_base,
                },
                Point {
                    x: centre.x + half_height,
                    y: centre.y + half_base,
                },
                Point {
                    x: centre.x - half_height,
                    y: centre.y,
                },
            ],
        };
        self.draw_polygon(&points, fill);
    }

    fn draw_plus_sign(&self, centre: Point, side_length: UFDRNum) {
        let half_side = FDRNum::from(side_length / 2);
        let points = vec![
            Point {
                x: centre.x,
                y: centre.y - half_side,
            },
            Point {
                x: centre.x,
                y: centre.y + half_side,
            },
        ];
        self.draw_line(&points);
        let points = vec![
            Point {
                x: centre.x - half_side,
                y: centre.y,
            },
            Point {
                x: centre.x + half_side,
                y: centre.y,
            },
        ];
        self.draw_line(&points);
    }
}
