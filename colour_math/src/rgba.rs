// Copyright (c) 2026 Peter Williams <pwil3058@bigpond.net.au> <pwil3058@gmail.com>.
use std::{
    // cmp::Ordering,
    // convert::TryInto,
    // convert::{From, TryFrom},
    ops::Index,
    // ops::{Add, Mul},
    // str::FromStr,
    // sync::LazyLock,
};

// use regex::Regex;

// use crate::attributes::Family;
use crate::{
    // attributes::{Chroma, Value, Warmth},
    // debug::ApproxEq,
    // fdrn::{Prop, UFDRNumber},
    // hcv::HCV,
    // hue::{angle::Angle, CMYHue, Hue, HueIfce, RGBHue, Sextant},
    // ColourBasics, , ManipulatedColour,
    HueConstants,
    LightLevel,
    RGBConstants,
};

#[derive(Serialize, Deserialize, Debug, Clone, Copy, Hash, PartialEq, Default)]
pub struct RGBA<T: LightLevel>(pub(crate) [T; 4]);

impl<T: LightLevel> Eq for RGBA<T> where T: Eq {}

impl<T: LightLevel> HueConstants for RGBA<T> {
    const RED: Self = Self([T::ONE, T::ZERO, T::ZERO, T::ONE]);
    const GREEN: Self = Self([T::ZERO, T::ONE, T::ZERO, T::ONE]);
    const BLUE: Self = Self([T::ZERO, T::ZERO, T::ONE, T::ONE]);

    const CYAN: Self = Self([T::ZERO, T::ONE, T::ONE, T::ONE]);
    const MAGENTA: Self = Self([T::ONE, T::ZERO, T::ONE, T::ONE]);
    const YELLOW: Self = Self([T::ONE, T::ONE, T::ZERO, T::ONE]);

    const BLUE_CYAN: Self = Self([T::ZERO, T::HALF, T::ONE, T::ONE]);
    const BLUE_MAGENTA: Self = Self([T::HALF, T::ZERO, T::ONE, T::ONE]);
    const RED_MAGENTA: Self = Self([T::ONE, T::ZERO, T::HALF, T::ONE]);
    const RED_YELLOW: Self = Self([T::ONE, T::HALF, T::ZERO, T::ONE]);
    const GREEN_YELLOW: Self = Self([T::HALF, T::ONE, T::ZERO, T::ONE]);
    const GREEN_CYAN: Self = Self([T::ZERO, T::ONE, T::HALF, T::ONE]);
}

impl<T: LightLevel> RGBConstants for RGBA<T> {
    const WHITE: Self = Self([T::ONE, T::ONE, T::ONE, T::ONE]);
    const LIGHT_GREY: Self = Self([T::ONE_QUARTER, T::ONE_QUARTER, T::ONE_QUARTER, T::ONE]);
    const MEDIUM_GREY: Self = Self([T::HALF, T::HALF, T::HALF, T::ONE]);
    const DARK_GREY: Self = Self([
        T::THREE_QUARTERS,
        T::THREE_QUARTERS,
        T::THREE_QUARTERS,
        T::ONE,
    ]);
    const BLACK: Self = Self([T::ZERO, T::ZERO, T::ZERO, T::ONE]);
}

impl<T: LightLevel> Index<usize> for RGBA<T> {
    type Output = T;

    fn index(&self, index: usize) -> &T {
        debug_assert!(index < 4);
        self.0.index(index)
    }
}

impl<L: LightLevel> From<[L; 4]> for RGBA<L> {
    fn from(array: [L; 4]) -> Self {
        debug_assert!(array.iter().all(|a| *a >= L::ZERO && *a <= L::ONE));
        Self(array)
    }
}

impl<L: LightLevel> From<RGBA<L>> for [L; 4] {
    fn from(rgb: RGBA<L>) -> Self {
        rgb.0
    }
}

// impl<L: LightLevel> From<RGBA<L>> for [L; 3] {
//     fn from(rgb: RGBA<L>) -> Self {
//         rgb.0[..3]
//     }
// }
