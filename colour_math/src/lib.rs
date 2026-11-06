// Copyright (c) 2026 Peter Williams <pwil3058@bigpond.net.au> <pwil3058@gmail.com>.

use std::fmt::{Debug, UpperHex};

use hue::HueIfce;

pub use crate::fixed_denominator_rational_numbers::*;

pub use crate::{
    attributes::{Chroma, Family, Greyness, ScalarAttribute, ScalarAttributeType, Value, Warmth},
    beigui::{
        attr_display::{
            ChromaCAD, ColourAttributeDisplay, ColourAttributeDisplayIfce, ColourAttributeType,
            GreynessCAD, HueCAD, ValueCAD, WarmthCAD,
        },
        hue_wheel::{
            ColouredShape, Graticule, HueWheel, MakeColouredShape, Proximity, Shape, ShapeConsts,
            Zoom,
        },
        Dirn, Draw, DrawIsosceles, DrawShapes, Point, Size, TextPosn,
    },
    cmy::{ComponentMask, IntoCMY, CMY},
    // fixed_denominator_rational_numbers::{
    //     debug, FDRNum, Float, IntoProp, NumberConstants, Prop, Signed, SignedInteger, UFDRNum,
    //     Unsigned, UnsignedInteger,
    // },
    hcv::*,
    hue::{angle::Angle, CMYHue, Hue, RGBHue, Sextant, SextantHue},
    manipulate::{ManipulatedColour, Outcome, SetScalarPolicy},
    manipulator::{ColourManipulator, ColourManipulatorBuilder, DeltaSize, SetHuePolicy},
    mixing::WeightedAverageMixer,
    rgb::{IntoRGB, RGBFrom, RGB},
    useful_consts::UsefulFloatConstants,
};

pub mod attributes;
pub mod beigui;
pub mod cmy;
pub mod fixed_denominator_rational_numbers;
pub mod hcv;
pub mod hue;
pub mod manipulate;
pub mod manipulator;
pub mod mixing;
pub mod rgb;
pub mod useful_consts;

pub trait LightLevel:
    Clone
    + Copy
    + AsProportion
    + FromProp
    + IntoProp
    + Debug
    + Default
    + PartialOrd
    + PartialEq
    + std::ops::Sub<Output = Self>
{
}
impl LightLevel for u8 {}
impl LightLevel for u16 {}
impl LightLevel for u32 {}
impl LightLevel for u64 {}
impl LightLevel for Prop {}
// NB f64 LightLevel should only be used when working with systems that are dumb enough to use them
impl LightLevel for f64 {}

pub trait HueConstants: Sized + Copy {
    const RED: Self;
    const GREEN: Self;
    const BLUE: Self;

    const CYAN: Self;
    const MAGENTA: Self;
    const YELLOW: Self;

    const BLUE_CYAN: Self;
    const BLUE_MAGENTA: Self;
    const RED_MAGENTA: Self;
    const RED_YELLOW: Self;
    const GREEN_YELLOW: Self;
    const GREEN_CYAN: Self;

    const PRIMARIES: [Self; 3] = [Self::BLUE, Self::RED, Self::GREEN];
    const SECONDARIES: [Self; 3] = [Self::CYAN, Self::MAGENTA, Self::YELLOW];
    const IN_BETWEENS: [Self; 6] = [
        Self::BLUE_CYAN,
        Self::BLUE_MAGENTA,
        Self::RED_MAGENTA,
        Self::RED_YELLOW,
        Self::GREEN_YELLOW,
        Self::GREEN_CYAN,
    ];
}

pub trait RGBConstants: HueConstants + Copy {
    const WHITE: Self;
    const LIGHT_GREY: Self;
    const MEDIUM_GREY: Self;
    const DARK_GREY: Self;
    const BLACK: Self;

    const GREYS: [Self; 5] = [
        Self::BLACK,
        Self::DARK_GREY,
        Self::MEDIUM_GREY,
        Self::LIGHT_GREY,
        Self::WHITE,
    ];
}

pub trait FromProps {
    fn from_proportions(props: [Prop; 3]) -> Self;
}

pub trait PropsInto<T: FromProps>: Sized + Copy {
    fn proportions_into(self) -> T;
}

impl<T: FromProps> PropsInto<T> for [Prop; 3] {
    fn proportions_into(self) -> T {
        T::from_proportions(self)
    }
}

pub trait IntoProps: Sized + Copy {
    fn proportions(self) -> impl Iterator<Item = Prop>;

    fn into_proportions(self) -> [Prop; 3] {
        self.proportions()
            .collect::<Vec<_>>()
            .try_into()
            .expect("into_proportions failed")
    }
}

pub trait PropsFrom<T: IntoProps>: Sized + Copy {
    fn proportions_from(arg: T) -> [Prop; 3];
}

impl<T: IntoProps> PropsFrom<T> for [Prop; 3] {
    fn proportions_from(arg: T) -> [Prop; 3] {
        T::into_proportions(arg)
    }
}

pub trait ColourAttributes {
    fn hue(&self) -> Option<Hue>;

    fn hue_angle(&self) -> Option<Angle> {
        Some(self.hue()?.angle())
    }

    fn hue_rgb<L: LightLevel>(&self) -> Option<RGB<L>> {
        Some(self.hue()?.max_chroma_rgb())
    }
    fn hue_hcv(&self) -> Option<HCV> {
        Some(self.hue()?.max_chroma_hcv())
    }

    fn is_grey(&self) -> bool {
        self.chroma() == Chroma::ZERO
    }

    fn chroma(&self) -> Chroma;
    fn chroma_prop(&self) -> Prop;
    fn value(&self) -> Value;

    fn greyness(&self) -> Greyness {
        self.chroma().into()
    }

    fn warmth(&self) -> Warmth;

    fn scalar_attribute(&self, sat: ScalarAttributeType) -> ScalarAttribute {
        use ScalarAttributeType::*;
        match sat {
            Chroma => ScalarAttribute::Chroma(self.chroma()),
            Greyness => ScalarAttribute::Greyness(self.greyness()),
            Value => ScalarAttribute::Value(self.value()),
            Warmth => ScalarAttribute::Warmth(self.warmth()),
        }
    }

    fn family(&self) -> Option<Family>;

    fn monochrome_hcv(&self) -> HCV {
        HCV::new_grey(self.value())
    }

    fn monochrome_rgb<L: LightLevel>(&self) -> RGB<L> {
        RGB::<L>::new_grey(self.value())
    }

    fn best_foreground(&self) -> HCV {
        match self.chroma() {
            Chroma::Shade(_) => {
                if let Some(family) = self.family() {
                    match family {
                        Family::Yellows | Family::Cyans => {
                            if self.value() < Value::ONE / 4 {
                                HCV::WHITE
                            } else {
                                HCV::BLACK
                            }
                        }
                        _ => HCV::WHITE,
                    }
                } else {
                    HCV::WHITE
                }
            }
            Chroma::Tint(_) => HCV::BLACK,
            _ => {
                if self.value() < Value::ONE / 2 {
                    HCV::WHITE
                } else {
                    HCV::BLACK
                }
            }
        }
    }
}

pub trait Colour: Sized + Copy + ColourAttributes + IntoRGB + IntoHCV + IntoCMY {
    fn pango_string(&self) -> String {
        let rgb = self.into_rgb::<u8>();
        format!("#{:02X}{:02X}{:02X}", rgb.0[0], rgb.0[1], rgb.0[2])
    }

    fn hex_string<L: LightLevel + Unsigned + UpperHex>(&self) -> String {
        let rgb = self.into_rgb::<L>();
        let hex_digits_per_item = std::mem::size_of::<L>() * 2;

        // 3$ means "use the argument at index 3 for the width"
        format!(
            "0x{:03$X}{:03$X}{:03$X}",
            rgb.0[0], rgb.0[1], rgb.0[2], hex_digits_per_item
        )
    }
}

pub trait ScalarAttributes: Colour {
    fn scalar_attribute_prop(&self, attr: ScalarAttributeType) -> Prop {
        match attr {
            ScalarAttributeType::Chroma => self.chroma().into_prop(),
            ScalarAttributeType::Greyness => Prop::ONE - self.chroma().into_prop(),
            ScalarAttributeType::Value => self.value().into(),
            ScalarAttributeType::Warmth => self.warmth().into(),
        }
    }

    fn scalar_attribute_rgb<T: LightLevel>(&self, attr: ScalarAttributeType) -> RGB<T> {
        match attr {
            ScalarAttributeType::Chroma => self.into_rgb(),
            ScalarAttributeType::Greyness => self.into_rgb(),
            ScalarAttributeType::Value => RGB::<T>::new_grey(self.value()),
            ScalarAttributeType::Warmth => RGB::<T>::new_warmth_rgb(self.warmth()),
        }
    }
}

impl ScalarAttributes for HCV {}
impl<L: LightLevel> ScalarAttributes for RGB<L> {}

pub trait ColourIfce: ColourAttributes + ScalarAttributes {}

impl ColourIfce for HCV {}
impl<L: LightLevel> ColourIfce for RGB<L> {}

#[cfg(test)]
mod rgb_tests {
    use super::*;
    use std::str::FromStr;

    #[test]
    fn write_hex() {
        assert_eq!(RGB::<u8>::RED.hex_string::<u8>(), "0xFF0000");
        assert_eq!(RGB::<u8>::CYAN.hex_string::<u16>(), "0x0000FFFFFFFF");
    }

    #[test]
    fn read_hex() {
        assert_eq!(RGB::<u8>::from_str("0xFF0000"), Ok(RGB::<u8>::RED));
        assert_eq!(RGB::<u16>::from_str("0x0000FFFFFFFF"), Ok(RGB::<u16>::CYAN));
    }
}
