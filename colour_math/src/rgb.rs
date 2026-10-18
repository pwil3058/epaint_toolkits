// Copyright (c) 2026 Peter Williams <pwil3058@bigpond.net.au> <pwil3058@gmail.com>.
use std::{
    cmp::Ordering,
    convert::TryInto,
    convert::{From, TryFrom},
    ops::{Div, Mul},
    ops::{Index, IndexMut},
    str::FromStr,
    sync::LazyLock,
};

use regex::Regex;

use crate::cmy::CMY;
use crate::{
    attributes::{Chroma, Family, Value, Warmth},
    debug::{ApproxEq, PropDiff},
    fdrn::{Prop, UFDRNum},
    hcv::HCV,
    hue::{angle::Angle, CMYHue, Hue, HueIfce, RGBHue, Sextant},
    ColourBasics, FromProps, HueConstants, LightLevel, ManipulatedColour, RGBConstants,
};

#[derive(Serialize, Deserialize, Debug, Clone, Copy, Hash, PartialEq, Eq, Default)]
pub struct RGB<T: LightLevel>(pub(crate) [T; 3]);

impl<T: LightLevel> AsRef<[T]> for RGB<T> {
    fn as_ref(&self) -> &[T] {
        &self.0
    }
}

impl<L: LightLevel> PropDiff for RGB<L> {}

impl<L: LightLevel + ApproxEq> ApproxEq for RGB<L> {
    fn approx_eq(&self, other: &Self, max_diff: Option<Prop>) -> bool {
        self[0].approx_eq(&other[0], max_diff)
            && self[1].approx_eq(&other[1], max_diff)
            && self[2].approx_eq(&other[2], max_diff)
    }
}

impl<T: LightLevel> HueConstants for RGB<T> {
    const RED: Self = Self([T::ONE, T::ZERO, T::ZERO]);
    const GREEN: Self = Self([T::ZERO, T::ONE, T::ZERO]);
    const BLUE: Self = Self([T::ZERO, T::ZERO, T::ONE]);

    const CYAN: Self = Self([T::ZERO, T::ONE, T::ONE]);
    const MAGENTA: Self = Self([T::ONE, T::ZERO, T::ONE]);
    const YELLOW: Self = Self([T::ONE, T::ONE, T::ZERO]);

    const BLUE_CYAN: Self = Self([T::ZERO, T::HALF, T::ONE]);
    const BLUE_MAGENTA: Self = Self([T::HALF, T::ZERO, T::ONE]);
    const RED_MAGENTA: Self = Self([T::ONE, T::ZERO, T::HALF]);
    const RED_YELLOW: Self = Self([T::ONE, T::HALF, T::ZERO]);
    const GREEN_YELLOW: Self = Self([T::HALF, T::ONE, T::ZERO]);
    const GREEN_CYAN: Self = Self([T::ZERO, T::ONE, T::HALF]);
}

impl<T: LightLevel> RGBConstants for RGB<T> {
    const WHITE: Self = Self([T::ONE, T::ONE, T::ONE]);
    const LIGHT_GREY: Self = Self([T::THREE_QUARTERS, T::THREE_QUARTERS, T::THREE_QUARTERS]);
    const MEDIUM_GREY: Self = Self([T::HALF, T::HALF, T::HALF]);
    const DARK_GREY: Self = Self([T::ONE_QUARTER, T::ONE_QUARTER, T::ONE_QUARTER]);
    const BLACK: Self = Self([T::ZERO, T::ZERO, T::ZERO]);
}

impl<T: LightLevel + Copy + From<Prop>> RGB<T> {
    pub fn new_grey(value: Value) -> Self {
        let value: Prop = value.into();
        Self::from_proportions([value, value, value])
    }

    pub fn new_warmth_rgb(warmth: Warmth) -> Self {
        let prop: Prop = warmth.into();
        Self::from_proportions([prop, Prop::ONE - prop, Prop::ONE - prop])
    }
}

impl<T: LightLevel + Into<Prop>> RGB<T> {
    pub fn from_proportions(array: [Prop; 3]) -> Self {
        let red: T = array[0].into();
        let green: T = array[1].into();
        let blue: T = array[2].into();
        Self([red, green, blue])
    }

    pub fn sum(&self) -> UFDRNum {
        let [red, green, blue] = self.into_proportions(); //<[Prop; 3]>::from(*self);
        red + green + blue
    }

    pub fn max_chroma_rgb(&self) -> RGB<T> {
        if let Ok(hue) = Hue::try_from(self) {
            hue.max_chroma_rgb::<T>()
        } else {
            RGB::<T>::new_grey(self.value())
        }
    }

    pub fn iter(&self) -> impl Iterator<Item = &T> {
        self.0.iter()
    }
}

impl<T: LightLevel + Into<Prop>> ColourBasics for RGB<T> {
    fn hue(&self) -> Option<Hue> {
        self.try_into().ok()
    }

    fn family(&self) -> Option<Family> {
        Some(self.hue()?.family())
    }

    fn is_grey(&self) -> bool {
        self.0[0] == self.0[1] && self.0[1] == self.0[2]
    }

    fn chroma(&self) -> Chroma {
        if let Ok(hue) = Hue::try_from(self) {
            let [red, green, blue] = self.into_proportions(); //<[Prop; 3]>::from(*self);
            let sum = self.sum();
            match hue {
                Hue::Primary(RGBHue::Red) => Chroma::from((red - blue, hue, sum)),
                Hue::Primary(RGBHue::Green) => Chroma::from((green - red, hue, sum)),
                Hue::Primary(RGBHue::Blue) => Chroma::from((blue - green, hue, sum)),
                Hue::Secondary(CMYHue::Cyan) => Chroma::from((blue - red, hue, sum)),
                Hue::Secondary(CMYHue::Magenta) => Chroma::from((red - green, hue, sum)),
                Hue::Secondary(CMYHue::Yellow) => Chroma::from((green - blue, hue, sum)),
                Hue::Sextant(sextant_hue) => match sextant_hue.sextant() {
                    Sextant::RedYellow => Chroma::from((red - blue, hue, sum)),
                    Sextant::RedMagenta => Chroma::from((red - green, hue, sum)),
                    Sextant::GreenYellow => Chroma::from((green - blue, hue, sum)),
                    Sextant::GreenCyan => Chroma::from((green - red, hue, sum)),
                    Sextant::BlueCyan => Chroma::from((blue - red, hue, sum)),
                    Sextant::BlueMagenta => Chroma::from((blue - green, hue, sum)),
                },
            }
        } else {
            Chroma::ZERO
        }
    }

    fn chroma_prop(&self) -> Prop {
        let [red, green, blue] = self.into_proportions(); //<[Prop; 3]>::from(*self);
        match red.cmp(&green) {
            Ordering::Greater => match green.cmp(&blue) {
                Ordering::Greater | Ordering::Equal => red - blue,
                Ordering::Less => match red.cmp(&blue) {
                    Ordering::Greater | Ordering::Equal => red - green,
                    Ordering::Less => blue - green,
                },
            },
            Ordering::Equal => match green.cmp(&blue) {
                Ordering::Equal => Prop::ZERO,
                Ordering::Less => blue - green,
                Ordering::Greater => green - blue,
            },
            Ordering::Less => match red.cmp(&blue) {
                Ordering::Equal | Ordering::Greater => green - blue,
                Ordering::Less => match green.cmp(&blue) {
                    Ordering::Greater | Ordering::Equal => green - red,
                    Ordering::Less => blue - red,
                },
            },
        }
    }

    fn value(&self) -> Value {
        (self.sum() / 3).into()
    }

    fn warmth(&self) -> Warmth {
        if let Some(hue) = self.hue() {
            hue.warmth()
        } else {
            Warmth::HALF
        }
    }

    fn hcv(&self) -> HCV {
        self.into()
    }

    fn rgb<L: LightLevel>(&self) -> RGB<L> {
        RGB::<L>::from_proportions(self.into_proportions())
    }

    fn cmy<L: LightLevel>(&self) -> CMY<L> {
        let props = CMY::<T>::from(*self).into_proportions();
        CMY::<L>::from_proportions(props)
    }

    fn into_proportions(&self) -> [Prop; 3] {
        [self.0[0].into(), self.0[1].into(), self.0[2].into()]
    }
}

impl<L: LightLevel> FromProps for RGB<L> {
    fn from_proportions(props: [Prop; 3]) -> Self {
        Self([props[0].into(), props[1].into(), props[2].into()])
    }
}

impl<L: LightLevel> ManipulatedColour for RGB<L> {
    fn lightened(&self, prop: Prop) -> Self {
        let compl = Prop::ONE - prop;
        let mut array = self.into_proportions(); //<[Prop; 3]>::from(self);
        for item in &mut array {
            *item = (*item * compl + prop).into();
        }
        RGB::<L>::from_proportions(array)
    }

    fn darkened(&self, prop: Prop) -> Self {
        let compl = Prop::ONE - prop;
        let mut array = self.into_proportions(); // <[Prop; 3]>::from(self);
        for item in &mut array {
            *item = *item * compl;
        }
        RGB::<L>::from_proportions(array)
    }

    fn saturated(&self, prop: Prop) -> Self {
        let hcv = HCV::from(self).saturated(prop);
        RGB::<L>::from(hcv)
    }

    fn greyed(&self, prop: Prop) -> Self {
        let hcv = HCV::from(self).greyed(prop);
        RGB::<L>::from(hcv)
    }

    fn rotated(&self, angle: Angle) -> Self {
        let hcv = HCV::from(self).rotated(angle);
        RGB::<L>::from(hcv)
    }
}

impl<T: LightLevel> Index<usize> for RGB<T> {
    type Output = T;

    fn index(&self, index: usize) -> &T {
        debug_assert!(index < 3);
        self.0.index(index)
    }
}

impl<T: LightLevel> IndexMut<usize> for RGB<T> {
    fn index_mut(&mut self, index: usize) -> &mut T {
        debug_assert!(index < 3);
        self.0.index_mut(index)
    }
}

// Comparisons
impl<T: LightLevel> PartialOrd for RGB<T>
where
    T: PartialOrd,
{
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        if self.0 == other.0 {
            Some(Ordering::Equal)
        } else if let Ok(hue) = Hue::try_from(self) {
            if let Ok(other_hue) = Hue::try_from(other) {
                // This orders via hue from CYAN to CYAN via GREEN, RED, BLUE in that order
                match hue.cmp(&other_hue) {
                    Ordering::Equal => match self.chroma().cmp(&other.chroma()) {
                        Ordering::Equal => Some(self.sum().cmp(&self.sum())),
                        order => Some(order),
                    },
                    order => Some(order),
                }
            } else {
                Some(Ordering::Greater)
            }
        } else if Hue::try_from(other).is_ok() {
            Some(Ordering::Less)
        } else {
            // No need to look a chroma as it will be zero for both
            self.sum().partial_cmp(&other.sum())
        }
    }
}

impl<T: LightLevel> Ord for RGB<T>
where
    T: PartialOrd + Eq,
{
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other)
            .expect("restricted range of values means this is OK")
    }
}

impl<T: LightLevel + ApproxEq> RGB<T> {
    pub fn approx_eq(&self, other: &Self, max_diff: Option<Prop>) -> bool {
        for i in 0..3 {
            if !self.0[i].approx_eq(&other.0[i], max_diff) {
                return false;
            }
        }
        true
    }
}

impl<L: LightLevel> From<[L; 3]> for RGB<L> {
    fn from(array: [L; 3]) -> Self {
        debug_assert!(array.iter().all(|a| *a >= L::ZERO && *a <= L::ONE));
        Self(array)
    }
}

impl<L: LightLevel> From<Vec<L>> for RGB<L> {
    fn from(vec: Vec<L>) -> Self {
        debug_assert!(vec.len() == 3 && vec.iter().all(|a| *a >= L::ZERO && *a <= L::ONE));
        Self([vec[0], vec[1], vec[2]])
    }
}

impl<L: LightLevel> From<RGB<L>> for [L; 3] {
    fn from(rgb: RGB<L>) -> Self {
        rgb.0
    }
}

impl<L: LightLevel> From<&[L; 3]> for RGB<L> {
    fn from(array: &[L; 3]) -> Self {
        Self::from(*array)
    }
}

impl<L: LightLevel> From<&RGB<L>> for [L; 3] {
    fn from(rgb: &RGB<L>) -> Self {
        Self::from(*rgb)
    }
}

impl<L: LightLevel> From<RGB<L>> for (L, L, L) {
    fn from(rgb: RGB<L>) -> Self {
        (rgb.0[0], rgb.0[1], rgb.0[2])
    }
}

// impl<T: LightLevel + From<Prop>> From<[Prop; 3]> for RGB<T> {
//     fn from(array: [Prop; 3]) -> Self {
//         let red: T = array[0].into();
//         let green: T = array[1].into();
//         let blue: T = array[2].into();
//         Self([red, green, blue])
//     }
// }

// impl<T: LightLevel + Into<Prop>> From<RGB<T>> for [Prop; 3] {
//     fn from(rgb: RGB<T>) -> Self {
//         [rgb.0[0].into(), rgb.0[1].into(), rgb.0[2].into()]
//     }
// }

// impl<T: LightLevel + From<Prop>> From<&[Prop; 3]> for RGB<T> {
//     fn from(array: &[Prop; 3]) -> Self {
//         Self::from(*array)
//     }
// }

// impl<T: LightLevel + From<Prop>> From<&RGB<T>> for [Prop; 3] {
//     fn from(rgb: &RGB<T>) -> Self {
//         Self::from(*rgb)
//     }
// }

// Arithmetic
impl<L: LightLevel + From<Prop>> Mul<Prop> for RGB<L> {
    type Output = Self;

    fn mul(self, scalar: Prop) -> Self {
        let [red, green, blue] = self.into_proportions(); //<[Prop; 3]>::from(self);
        let array: [Prop; 3] = [red * scalar, green * scalar, blue * scalar];
        Self::from_proportions(array)
    }
}

/// Divide all components by `scalar`
///
/// Example:
///
/// ```
/// use colour_math::{HueConstants, RGB};
///
/// assert_eq!((RGB::<u8>::RED / 2), RGB::<u8>::from([127, 0, 0]));
/// assert_eq!((RGB::<u8>::MAGENTA / 2), RGB::<u8>::from([127, 0, 127]));
/// ```
impl<L: LightLevel> Div<i32> for RGB<L> {
    type Output = Self;

    fn div(self, scalar: i32) -> Self {
        let [red, green, blue] = self.into_proportions();
        Self([
            (red / scalar).into(),
            (green / scalar).into(),
            (blue / scalar).into(),
        ])
    }
}

// impl<L: LightLevel + From<Prop>> Add<RGB<L>> for RGB<L> {
/////TODO: get rid of RGB addition
// type Output = Self;
//
// fn add(self, rhs: Self) -> Self {
//     let [red, green, blue] = self.into_proportions(); //<[Prop; 3]>::from(self);
//     let [rhs_red, rhs_green, rhs_blue] = rhs.into_proportions(); //<[Prop; 3]>::from(rhs);
//     let array: [Prop; 3] = [
//         (red / 2 + rhs_red / 2).into(),
//         (green / 2 + rhs_green / 2).into(),
//         (blue / 2 + rhs_blue / 2).into(),
//     ];
//     Self::from_proportions(array)
// }
// }

#[derive(Debug, PartialEq, Eq)]
pub enum RGBError {
    MalformedText(String),
}

impl std::fmt::Display for RGBError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RGBError::MalformedText(string) => write!(f, "Malformed text: {string}"),
        }
    }
}

impl std::error::Error for RGBError {}

impl From<std::num::ParseIntError> for RGBError {
    fn from(error: std::num::ParseIntError) -> Self {
        RGBError::MalformedText(format!("{error}"))
    }
}

// lazy_static! {
pub static RGB16_RE: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(
        r"RGB(16)?\((red=)?0x(?P<red>[a-fA-F0-9]{4}), (green=)?0x(?P<green>[a-fA-F0-9]{4}), (blue=)?0x(?P<blue>[a-fA-F0-9]{4})\)"
    ).unwrap()
});
pub static RGB16_BASE_10_RE: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(
        r"RGB(16)?\((red=)?(?P<red>\d{1,5}), (green=)?(?P<green>\d{1,5}), (blue=)?(?P<blue>\d{1,5})\)"
    ).unwrap()
});
pub static RGB8_RE: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(
        r"RGB(8)?\((red=)?0x(?P<red>[a-fA-F0-9]{2}), (green=)?0x(?P<green>[a-fA-F0-9]{2}), (blue=)?0x(?P<blue>[a-fA-F0-9]{2})\)"
    ).unwrap()
});
pub static RGB8_BASE_10_RE: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(
        r"RGB(8)?\((red=)?(?P<red>\d{1,3}), (green=)?(?P<green>\d{1,3}), (blue=)?(?P<blue>\d{1,3})\)"
    ).unwrap()
});
pub static RGB_PANGO_RE: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(
        r#"#(?P<red>[a-fA-F0-9][a-fA-F0-9])(?P<green>[a-fA-F0-9][a-fA-F0-9])(?P<blue>[a-fA-F0-9][a-fA-F0-9])"#
    ).unwrap()
});

pub static RGB8_HEX_RE: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"0x(?P<red>[a-fA-F0-9]{2})(?P<green>[a-fA-F0-9]{2})(?P<blue>[a-fA-F0-9]{2})")
        .unwrap()
});

pub static RGB16_HEX_RE: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"0x(?P<red>[a-fA-F0-9]{4})(?P<green>[a-fA-F0-9]{4})(?P<blue>[a-fA-F0-9]{4})")
        .unwrap()
});

impl FromStr for RGB<u8> {
    type Err = RGBError;

    fn from_str(string: &str) -> Result<Self, Self::Err> {
        if let Some(captures) = RGB8_HEX_RE.captures(string) {
            let red = u8::from_str_radix(captures.name("red").unwrap().as_str(), 16)?;
            let green = u8::from_str_radix(captures.name("green").unwrap().as_str(), 16)?;
            let blue = u8::from_str_radix(captures.name("blue").unwrap().as_str(), 16)?;
            Ok([red, green, blue].into())
        } else if let Some(captures) = RGB8_RE.captures(string) {
            let red = u8::from_str_radix(captures.name("red").unwrap().as_str(), 16)?;
            let green = u8::from_str_radix(captures.name("green").unwrap().as_str(), 16)?;
            let blue = u8::from_str_radix(captures.name("blue").unwrap().as_str(), 16)?;
            Ok([red, green, blue].into())
        } else if let Some(captures) = RGB8_BASE_10_RE.captures(string) {
            let red = captures.name("red").unwrap().as_str().parse::<u8>()?;
            let green = captures.name("green").unwrap().as_str().parse::<u8>()?;
            let blue = captures.name("blue").unwrap().as_str().parse::<u8>()?;
            Ok([red, green, blue].into())
        } else {
            Err(RGBError::MalformedText(string.to_string()))
        }
    }
}

impl FromStr for RGB<u16> {
    type Err = RGBError;

    fn from_str(string: &str) -> Result<Self, Self::Err> {
        if let Some(captures) = RGB16_HEX_RE.captures(string) {
            let red = u16::from_str_radix(captures.name("red").unwrap().as_str(), 16)?;
            let green = u16::from_str_radix(captures.name("green").unwrap().as_str(), 16)?;
            let blue = u16::from_str_radix(captures.name("blue").unwrap().as_str(), 16)?;
            Ok([red, green, blue].into())
        } else if let Some(captures) = RGB16_RE.captures(string) {
            let red = u16::from_str_radix(captures.name("red").unwrap().as_str(), 16)?;
            let green = u16::from_str_radix(captures.name("green").unwrap().as_str(), 16)?;
            let blue = u16::from_str_radix(captures.name("blue").unwrap().as_str(), 16)?;
            Ok([red, green, blue].into())
        } else if let Some(captures) = RGB16_BASE_10_RE.captures(string) {
            let red = captures.name("red").unwrap().as_str().parse::<u16>()?;
            let green = captures.name("green").unwrap().as_str().parse::<u16>()?;
            let blue = captures.name("blue").unwrap().as_str().parse::<u16>()?;
            Ok([red, green, blue].into())
        } else {
            Err(RGBError::MalformedText(string.to_string()))
        }
    }
}
